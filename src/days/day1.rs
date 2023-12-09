use crate::aoc;

fn part1(lines: &[String]) -> u64 {
    let mut sum = 0;
    for line in lines {
        let mut numbers = Vec::new();

        for chr in line.chars() {
            if let Some(digit) = chr.to_digit(10) {
                numbers.push(digit as u64);
            }
        }

        let first = numbers.first().unwrap();
        let num = first * 10 + numbers.last().unwrap_or(first);
        sum += num;
    }

    sum
}

fn part2(lines: &[String]) -> u64 {
    let mut sum = 0;
    for line in lines {
        let mut numbers = Vec::new();
        let chrs: Vec<_> = line.chars().collect();

        for i in 0..line.len() {
            if line[i..].starts_with("one") {
                numbers.push(1);
            } else if line[i..].starts_with("two") {
                numbers.push(2);
            } else if line[i..].starts_with("three") {
                numbers.push(3);
            } else if line[i..].starts_with("four") {
                numbers.push(4);
            } else if line[i..].starts_with("five") {
                numbers.push(5);
            } else if line[i..].starts_with("six") {
                numbers.push(6);
            } else if line[i..].starts_with("seven") {
                numbers.push(7);
            } else if line[i..].starts_with("eight") {
                numbers.push(8);
            } else if line[i..].starts_with("nine") {
                numbers.push(9);
            } else if let Some(digit) = chrs[i].to_digit(10) {
                numbers.push(digit as u64);
            }
        }

        let num = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        sum += num;
    }

    sum
}

pub fn run() -> (String, String) {
    let lines = aoc::lines_from_file("day1.txt");
    let part1_result = part1(&lines);
    let part2_result = part2(&lines);

    (part1_result.to_string(), part2_result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{aoc, days::day1::{part1, part2}};

    #[test]
    fn part_1_test() {
        let lines = aoc::lines_from_test(
            r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(part1(&lines), 142);
    }

    #[test]
    fn part_2_test() {
        let lines = aoc::lines_from_test(
            r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(part2(&lines), 281);
    }
}
