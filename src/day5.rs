use std::fs;

pub fn day5() {
    let content = fs::read_to_string("input-aoc-2022-5.txt")
        .expect("File should be readable");
    let split_content = content.split("\n\n").collect::<Vec<&str>>();
    let mut state = parse_initial_state(split_content[0]);
    let commands = parse_commands(split_content[1]);
    println!("Advent of Code 2022/5/1: {}", get_top_items(execute_commands(&mut state, commands)));
}

fn parse_initial_state(initial_state_input: &str) -> Vec<Vec<char>> {
    let mut split_initial_state_input = initial_state_input.split("\n")
        .filter(|&x| x.contains("["))
        .collect::<Vec<&str>>();

    let mut transposed: Vec<Vec<char>> = vec![vec![]; split_initial_state_input[0].len() / 4 + 1];
    for i in (0..split_initial_state_input.len()).rev() {
        for j in (1..split_initial_state_input[0].len()).step_by(4) {
            let char_at = split_initial_state_input[i].chars().nth(j).unwrap();
            if char_at != ' ' {
                transposed[(j - 1) / 4].push(char_at);
            }
        }
    }
    return transposed;
}

fn parse_commands(commands: &str) -> Vec<(usize, usize, usize)> {
    return commands.split("\n")
        .filter(|&command| !command.is_empty())
        .map(parse_command)
        .collect();
}

fn parse_command(command: &str) -> (usize, usize, usize) {
    let command_numbers = command.split(" ")
        .filter(|&line| line.parse::<usize>().is_ok())
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    assert_eq!(command_numbers.len(), 3);
    return (command_numbers[0], command_numbers[1], command_numbers[2])
}

fn get_top_items(state: &mut Vec<Vec<char>>) -> String {
    return state.iter()
        .map(|s| s.last().unwrap())
        .collect();
}

fn execute_commands(state: &mut Vec<Vec<char>>, commands: Vec<(usize, usize, usize)>) -> &mut Vec<Vec<char>> {
    for command in commands {
        for _ in 0..command.0 {
            let popped = state[command.1 - 1].pop().unwrap();
            state[command.2 - 1].push(popped);
        }
    }

    return state;
}