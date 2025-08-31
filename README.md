# ELAN Compiler

ELAN is a simple statically typed language with a modern syntax. It's supposed to be as easy as Python but as powerful as C.

## Code Example
```elan
/* 
 * This function checks whether a number is prime or composite.
 * 
 * Arguments:
 * - `num` the number we want to check for primality.
 *
 * Returns:
 * - `true` if the number is prime.
 * - `false` if the number is composite.
 */
proc is_prime(num: u64) u64 {
    for div in 2..(num / 2) {
        if num % div == 0 {
            return false;
        }
    }

    return true;
}

proc main() i32 {
    for n in 1..100 {
        if is_prime(n) {
            printf("{} is prime!", n);
        } else {
            printf("{} is composite!", n);
        }
    }

    return 0;
}
```

## Language Support

If you are looking for ELAN Language Support look no further than [here](https://github.com/jooris-hadeler/elan-language-support).