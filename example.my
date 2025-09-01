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