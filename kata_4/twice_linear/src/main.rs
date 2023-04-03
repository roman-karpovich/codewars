use std::collections::{BTreeSet, VecDeque};
use std::thread::current;

fn get_next_elements(x: u32) -> (u32, u32) {
    (2*x+1, 3*x+1)
}

fn dbl_linear(n: u32) -> u32{
    // calculate iterations number
    let mut data  = BTreeSet::from([1]);
    let mut queue = BTreeSet::from([1]);;

    loop {
        if queue.len() == 0 {
            break;
        }
        let (y, z) = get_next_elements(queue.pop_first().unwrap());
        let len = data.len() as u32;
        if len < n + 1 {
            queue.insert(y.clone());
            queue.insert(z.clone());
        }

        data.insert(y);
        data.insert(z);
    }

    loop {
        if data.len() as u32 == n + 1 {
            break;
        } else {
            data.pop_last();
        }
    }
    data.pop_last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}

fn main() {
    println!("Hello, world!");
}


// 1, 3, 4, 7, 9, 10, 13, 15, 19, 21, 22, 27,
// 1, 3, 4, 7, 9, 10, 13, 15, 19, 22, 28