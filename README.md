# Halo2 Exponentiation Circuit

This project implements an exponentiation circuit using the Halo2 proving system.

### Algorithm

The implementation utilizes the **Square-and-Multiply Algorithm** for exponentiation.

```
FUNCTION square_and_multiply(base, exponent):
    result = 1
    temp = base

    WHILE exponent > 0:
        IF exponent % 2 == 1:
            result = result * temp

        temp = temp * temp
        exponent = exponent >> 1

    RETURN result
END FUNCTION
```

### Testing

To run the tests and verify the correctness of the circuit implementation, run:

```bash
cargo test
```

This will execute the test defined in the `test_circuit` function.
