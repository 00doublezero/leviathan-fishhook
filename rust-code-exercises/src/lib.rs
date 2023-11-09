pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn opposites_attract(flower1: u16, flower2: u16) -> bool {
    let result =  if  (flower1 + flower2) % 2 > 0 {true} else {false};
    result
}

pub fn simple_multiplication(number: u8) -> u8 {
    let result: u8 = if number % 2 > 0 {number * 9} else { number * 8};
    result
 }

pub fn high_and_low(numbers: &str) -> String {
    // https://www.codewars.com/kata/554b4ac871d6813a03000035
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    for (i, n) in numbers.split_whitespace().enumerate() {
        if i == 0 {
            min = n.parse::<i32>().unwrap();
            max = min;
        } 
        min = min.min(n.parse::<i32>().unwrap());
        max = max.max(n.parse::<i32>().unwrap());
    }
    format!("{}{}{}",max.to_string(), " ", min.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn opposites_attract_tests() {
        opposites_attract_test(1,4, true);
        opposites_attract_test(2,2, false);
        opposites_attract_test(0,1, true);
        opposites_attract_test(0,0, false);
    }

    fn opposites_attract_test(f1: u16, f2:u16, exp: bool ) {
        assert_eq!(opposites_attract(f1, f2), exp, "\nFailed with flower1 {}, flower2 {}", f1, f2);
    }

    #[test]
    fn simple_multiplication_tests() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }

    #[test]
    fn high_and_low_tests() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
        assert_eq!("3 1", high_and_low("1 2 3"));
    }

}
