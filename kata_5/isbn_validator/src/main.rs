use regex::Regex;

fn valid_isbn10(isbn: &str) -> bool {
    let re = Regex::new(r"^\d{9}(\d|X)$").unwrap();
    if !re.is_match(&isbn) {
        return false;
    }
    let mut sum = 0;
    let mut chars: Vec<char> = isbn.chars().collect();
    for i in 0 .. isbn.len() {
        let s = chars[i];
        let num = match s {
            'X' => 10,
            v => v.to_string().parse::<i32>().unwrap()
        };
        sum += ((i as i32) + 1) * num;
    }
    sum % 11 == 0
}

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(actual == expected, "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}

fn main() {
    println!("Hello, world!");
}
