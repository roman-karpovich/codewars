// n = 12; decomp(12) -> "2^10 * 3^5 * 5^2 * 7 * 11"
// since 12! is divisible by 2 ten times, by 3 five times, by 5 two times and by 7 and 11 only once.

use std::thread::current;

// 12! = 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12
// 12! = 2 * 3 * (2^2) * 5 * (2 * 3) * 7 * (2^4) * 9 * (2*5) * 11 * (2^2 * 3)
// 12! = 2
fn decompose_prime(n: i32) -> Vec<i32> {
    let mut elements = vec![];
    let mut is_prime= true;
    for x in (2..n).rev() {
        if n % x == 0 {
            elements.extend(decompose_prime(x));
            elements.extend(decompose_prime(n/x));
            is_prime = false;
            break;
        }
    }
    if is_prime {
        elements.push(n)
    }
    elements
}

fn decomp(n: i32) -> String {
    // iterate from 2 to number inclusively
    // for every number get biggest prime divider. return divider, do recursively with remaining part
    // count all of them, return smaller to biggest recursively
    let mut prime_elements = vec![];
    for x in 2..n+1 {
        prime_elements.extend(decompose_prime(x));
    }
    prime_elements.sort();
    // println!("!!!1111{:?}", prime_elements);
    let mut result = vec![];
    let mut counter= 1;
    for i in 0..prime_elements.len() {
        // check last element
        if i != prime_elements.len() - 1 && prime_elements[i] == prime_elements[i+1] {
            counter += 1;
        } else {
            // println!("found {}", prime_elements[i]);
            if counter > 1 {
                result.push(format!("{}^{}", prime_elements[i], counter));
            } else {
                result.push(prime_elements[i].to_string());
            }
            counter = 1;
        }
    }
    result.join(" * ")
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");

    }
}


fn main() {
    println!("Hello, world!");
}
