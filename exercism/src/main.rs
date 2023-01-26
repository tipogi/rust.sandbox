/// Check a Luhn checksum.
/// - The first step of the Luhn algorithm is to double every second digit, starting from the right
/// - If doubling the number results in a number greater than 9 then subtract 9 from the product


pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        let card: Vec<u32> = code
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                return c.to_digit(10).unwrap()
            })
            .collect();
        let mut sum = 0;
        let mut multiply = true;
        for number in card {
            if multiply {
                match number {
                    x if x < 5 => sum += number * 2,
                    x if x > 4 && x < 9 => sum += (number * 2) % 9,
                    _ => sum += 9
                }
                multiply = false;
            } else {
                sum += number;
                multiply = true;
            }
            
            println!("SUM: {}, number: {}", sum, number);
        }
        println!("res: {}", sum);
        return sum % 10 == 0
    }
    false
}


pub fn is_valid_optimised(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}


pub fn test_try_fold() {
    let correct = vec![ '1','6', '8' ,'5', '7', '8', '3', '2', '1', '4', '6'];
    let wrong = vec![ 'd','6', '8' ,'5', '7', '8', '3', '2', '1', '4', '6'];

    let experiment = correct
        .iter()
        .try_fold((0, 0), |(sum, count), number| {
            number
                .to_digit(10)
                .map(|num| (num + sum, count + 1))
        });
    println!("Experiment: {:?}", experiment);
    let no = None.map_or(false, |other: i32| true);
    println!("Map {:?}", no);
}
pub fn main() {
    let card = "4539 3195 0343 6467";
    let valid_card = is_valid(card);
    println!("Is the {} card valid? {}", card, valid_card);
    test_try_fold();
}