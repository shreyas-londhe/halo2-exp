use halo2_base::{ gates::{ GateChip, GateInstructions }, utils::BigPrimeField, Context };

struct CircuitInput<F: BigPrimeField> {
    base: F,
    exp: F,
    res: F,
    max_bits: usize,
}

fn circuit<F: BigPrimeField>(ctx: &mut Context<F>, gate: &GateChip<F>, input: CircuitInput<F>) {
    let base = ctx.load_witness(input.base);
    let exp = ctx.load_witness(input.exp);
    let res = ctx.load_witness(input.res);

    let bits_of_exponent = gate.num_to_bits(ctx, exp, input.max_bits);
    // Utilizing the square-and-multiply algorithm
    let mut _res = ctx.load_constant(F::ONE);
    for (index, each_bit) in bits_of_exponent.into_iter().rev().enumerate() {
        if index != 0 {
            // Perform squaring
            _res = gate.mul(ctx, _res, _res);
        }
        // Perform multiplication
        let product = gate.mul(ctx, _res, base);

        // Select the product based on the bit
        _res = gate.select(ctx, product, _res, each_bit);
    }

    ctx.constrain_equal(&res, &_res);
}

#[cfg(test)]
mod test {
    use halo2_base::{
        gates::RangeInstructions,
        halo2_proofs::halo2curves::bn256::Fr,
        utils::{ testing::base_test, ScalarField },
    };
    use num_bigint::{ BigUint, RandomBits };
    use num_traits::Num;
    use rand::{ thread_rng, Rng };

    use crate::{ circuit, CircuitInput };

    #[test]
    fn test_circuit() {
        let mut rng = thread_rng();

        // BN254 modulus
        let modulus = BigUint::from_str_radix(
            "30644e72e131a029b85045b68181585d28333e84879b9709143e1f593f0000001",
            16
        ).unwrap();

        let base: BigUint = rng.sample(RandomBits::new(253));
        let exp: BigUint = rng.sample(RandomBits::new(253));
        let res: BigUint = base.modpow(&exp, &modulus);

        base_test()
            .k(15)
            .expect_satisfied(true)
            .run(|ctx, range| {
                let gate = range.gate();
                let input = CircuitInput {
                    base: Fr::from_bytes_le(base.to_bytes_le().as_slice()),
                    exp: Fr::from_bytes_le(exp.to_bytes_le().as_slice()),
                    res: Fr::from_bytes_le(res.to_bytes_le().as_slice()),
                    max_bits: 253,
                };
                circuit(ctx, gate, input);
            });
    }
}
