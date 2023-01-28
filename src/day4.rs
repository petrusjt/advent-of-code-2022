use std::fs;

pub fn day4() {
    let content = fs::read_to_string("input-aoc-2022-4.txt")
        .expect("File should be readable");
    let result = content.split_terminator("\n")
        .map(get_range_pair_from_line)
        .filter(is_range_fully_contained_in_other)
        .collect::<Vec<(Range, Range)>>();
    println!("Advent of Code 2022/4: {}", result.len());
}

fn get_range_pair_from_line(line: &str) -> (Range, Range) {
    let mut iter_linesplit = line.splitn(2, ",");
    let (section1, section2) = (iter_linesplit.next().unwrap(),
                                          iter_linesplit.next().unwrap());

    return (get_range_from_section(section1), get_range_from_section(section2))
}

fn get_range_from_section(section: &str) -> Range {
    let mut section_split = section.splitn(2, "-");
    let begin = section_split.next().unwrap().parse::<i32>().unwrap();
    let end = section_split.next().unwrap().parse::<i32>().unwrap();
    Range { begin, end }
}

fn is_range_fully_contained_in_other((range1, range2): &(Range, Range)) -> bool {
    range1.begin <= range2.begin && range1.end >= range2.end
        || range2.begin <= range1.begin && range2.end >= range1.end
}

struct Range {
    begin: i32,
    end: i32
}