use crate::aoc;

fn get_rows(line: &str) -> Vec<Vec<i64>> {
    let nums: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    fn get_next_row(nums: &[i64]) -> Vec<i64> {
        let mut next_row = Vec::new();
        for ns in nums.windows(2) {
            let n1 = ns[0];
            let n2 = ns[1];
            next_row.push(n2 - n1);
        }
        next_row
    }

    let mut rows = vec![nums];
    loop {
        let mut current_row = get_next_row(rows.last().unwrap());
        let all_zeroes = current_row.iter().all(|n| *n == 0);
        if all_zeroes {
            current_row.push(0);
        }

        rows.push(current_row);

        if all_zeroes {
            break;
        }
    }

    rows
}

fn extrapolate_rows(rows: &mut Vec<Vec<i64>>) {
    for i in (0..(rows.len() - 1)).rev() {
        let j = i + 1;
        let wanted = *rows[j].last().unwrap();
        let row = &mut rows[i];
        let left = *row.last().unwrap();
        row.push(left + wanted);
    }
}

fn extrapolate_backwards(rows: &mut Vec<Vec<i64>>) {
    for i in (0..(rows.len() - 1)).rev() {
        let j = i + 1;
        let wanted = *rows[j].first().unwrap();
        let row = &mut rows[i];
        let right = *row.first().unwrap();
        row.insert(0, right - wanted);
    }
}

fn part1(lines: &Vec<String>) -> i64 {
    let line_rows = lines.iter().map(|l| get_rows(l));

    let mut sum = 0;
    for mut line_row in line_rows {
        extrapolate_rows(&mut line_row);
        sum += line_row.first().unwrap().last().unwrap();
    }

    sum
}

fn part2(lines: &Vec<String>) -> i64 {
    let line_rows = lines.iter().map(|l| get_rows(l));

    let mut sum = 0;
    for mut line_row in line_rows {
        extrapolate_backwards(&mut line_row);
        sum += line_row.first().unwrap().first().unwrap();
    }

    sum
}

pub fn run() -> (String, String) {
    let lines = aoc::lines_from_file("day9.txt");

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);

    (part1_result.to_string(), part2_result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        aoc,
        days::day9::{extrapolate_rows, get_rows, extrapolate_backwards, part2},
    };

    use super::part1;

    #[test]
    fn history_test() {
        let input = r"0 3 6 9 12 15";

        let mut rows = get_rows(input);
        assert_eq!(
            rows,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0]
            ]
        );

        extrapolate_rows(&mut rows);

        assert_eq!(
            rows,
            vec![
                vec![0, 3, 6, 9, 12, 15, 18],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0]
            ]
        );
    }

    #[test]
    fn history_test_backwards() {
        let input = r"10 13 16 21 30 45";

        let mut rows = get_rows(input);
        assert_eq!(
            rows,
            vec![
                vec![10, 13, 16, 21, 30, 45],
                vec![3, 3, 5, 9, 15],
                vec![0, 2, 4, 6],
                vec![2, 2, 2],
                vec![0, 0, 0]
            ]
        );

        extrapolate_backwards(&mut rows);

        assert_eq!(
            rows,
            vec![
                vec![5, 10, 13, 16, 21, 30, 45],
                vec![5, 3, 3, 5, 9, 15],
                vec![-2, 0, 2, 4, 6],
                vec![2, 2, 2, 2],
                vec![0, 0, 0]
            ]
        );
    }

    #[test]
    fn part1_test() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let lines = aoc::lines_from_test(input);
        assert_eq!(part1(&lines), 114);
    }

    #[test]
    fn part2_test() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let lines = aoc::lines_from_test(input);
        assert_eq!(part2(&lines), 2);
    }
}
