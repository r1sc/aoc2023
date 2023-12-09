use crate::aoc;

// fn parse_sections(input: &str) {
//     input.split("\n\n")
//     .map(|section| section.split(":"))
//     .map(|section| section.t)
// }

fn part1(input: &str) -> u64 {
    todo!()
}

fn part2(input: &str) -> u64 {
    todo!()
}

pub fn run() -> (String, String) {
    let input = aoc::string_from_file("day5.txt");
    let part1_result = part1(&input);
    let part2_result = part2(&input);

    (part1_result.to_string(), part2_result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{aoc, days::day5::{part1, part2}};

    #[test]
    fn part_1_test() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part1(input), 35);
    }

//     #[test]
//     fn part_2_test() {
//         let lines = aoc::lines_from_test(
//             r"two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen",
//         );

//         assert_eq!(part2(&lines), 281);
//     }
}
