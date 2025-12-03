fn main() {
    const INPUT: &str = include_str!("input.txt");

    let battery_banks = BatteryBank::parse_multiple(INPUT);

    let sum: u64 = battery_banks
        .iter()
        .map(|bb| bb.find_highest_voltage(12))
        .sum();

    println!("The sum was: {}", sum);
}

struct BatteryBank {
    batteries: Vec<u64>,
}

struct FindDigitResult {
    digit: u64,
    index: usize,
}

impl BatteryBank {
    const BATTERY_BANK_RADIX: u32 = 10;

    fn parse(battery_line: &str) -> Self {
        Self {
            batteries: battery_line
                .chars()
                .map(|c| BatteryBank::char_to_digit(c))
                .collect(),
        }
    }

    fn find_highest_digit(digits: &[u64]) -> FindDigitResult {
        let mut highest = FindDigitResult {
            digit: digits[0],
            index: 0,
        };

        for (index, digit) in digits.iter().enumerate() {
            if *digit > highest.digit {
                highest = FindDigitResult {
                    digit: *digit,
                    index,
                }
            }
        }

        highest
    }

    fn char_to_digit(c: char) -> u64 {
        c.to_digit(BatteryBank::BATTERY_BANK_RADIX).unwrap() as u64
    }

    fn find_highest_voltage(&self, num_batteries_to_use: usize) -> u64 {
        let mut number = String::new();

        let mut current_index: usize = 0;

        for i in (0..num_batteries_to_use).rev() {
            let allowed_batteries = &self.batteries[current_index..self.batteries.len() - i];

            let result = BatteryBank::find_highest_digit(allowed_batteries);

            number.push_str(&result.digit.to_string());
            current_index = current_index + result.index + 1;
        }

        number.parse::<u64>().unwrap()
    }

    fn parse_multiple(battery_lines: &str) -> Vec<Self> {
        let mut battery_banks: Vec<Self> = Vec::new();

        for line in battery_lines.lines() {
            battery_banks.push(BatteryBank::parse(line));
        }

        battery_banks
    }
}

#[cfg(test)]
mod tests {
    use crate::BatteryBank;

    const SINGLE_BATTERY_BANK: &str = "987654321111111";
    const SAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_find_highest_joltage() {
        let battery_bank = BatteryBank::parse(SINGLE_BATTERY_BANK);

        let voltage = battery_bank.find_highest_voltage(2);

        assert_eq!(voltage, 98);
    }

    #[test]
    fn test_muliple_battery_banks_2() {
        let battery_banks = BatteryBank::parse_multiple(SAMPLE_INPUT);

        assert_eq!(
            battery_banks
                .iter()
                .map(|b| b.find_highest_voltage(2))
                .collect::<Vec<u64>>(),
            vec![98, 89, 78, 92],
        );
    }

    #[test]
    fn test_muliple_battery_banks_12() {
        let battery_banks = BatteryBank::parse_multiple(SAMPLE_INPUT);

        assert_eq!(
            battery_banks
                .iter()
                .map(|b| b.find_highest_voltage(12))
                .collect::<Vec<u64>>(),
            vec![987654321111, 811111111119, 434234234278, 888911112111],
        );
    }
}
