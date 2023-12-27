use std::fs;

pub fn day8() {
    let content: String = fs::read_to_string("input-aoc-2022-8.txt").unwrap();
    let tmp: &str = content.as_str();
    let tree_map: Vec<Vec<u32>> = tmp.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.chars()
            .map(|x: char| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect();
    println!("Advent of Code 2022/8/1: {}", count_visible_trees(tree_map));
}

fn count_visible_trees(tree_map: Vec<Vec<u32>>) -> u32 {
    let mut visible_trees = 0;
    for i in 0..tree_map.len() {
        for j in 0..tree_map[i].len() {
            if is_tree_visible(&tree_map, i, j) {
                visible_trees += 1;
            }
        }
    }
    return visible_trees;
}

fn is_tree_visible(tree_map: &Vec<Vec<u32>>, y: usize, x: usize) -> bool {
    return is_tree_visible_from_up(&tree_map, &y, &x)
        || is_tree_visible_from_down(&tree_map, &y, &x)
        || is_tree_visible_from_left(&tree_map[y], &x)
        || is_tree_visible_from_right(&tree_map[y], &x);
}

fn is_tree_visible_from_up(tree_map: &Vec<Vec<u32>>, y: &usize, x: &usize) -> bool {
    let tree_height = tree_map[*y][*x];
    for i in 0..*y {
        if tree_map[i][*x] >= tree_height {
            return false;
        }
    }
    return true;
}

fn is_tree_visible_from_down(tree_map: &Vec<Vec<u32>>, y: &usize, x: &usize) -> bool {
    let tree_height = tree_map[*y][*x];
    for i in (y + 1)..tree_map.len() {
        if tree_map[i][*x] >= tree_height {
            return false;
        }
    }
    return true;
}

fn is_tree_visible_from_left(row: &Vec<u32>, x: &usize) -> bool {
    let tree_height = row[*x];
    for i in 0..*x {
        if row[i] >= tree_height {
            return false;
        }
    }
    return true;
}

fn is_tree_visible_from_right(row: &Vec<u32>, x: &usize) -> bool {
    let tree_height = row[*x];
    for i in (x + 1)..row.len() {
        if row[i] >= tree_height {
            return false;
        }
    }
    return true;
}

pub fn day8_part2() {
    let content: String = fs::read_to_string("input-aoc-2022-8.txt").unwrap();
    let tmp: &str = content.as_str();
    let tree_map: Vec<Vec<u32>> = tmp.split("\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.chars()
            .map(|x: char| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect();
    println!("Advent of Code 2022/8/2: {}", get_highest_scenic_score(tree_map));
}



fn get_highest_scenic_score(tree_map: Vec<Vec<u32>>) -> u32 {
    let mut scenic_scores: Vec<u32> = vec![];
    for i in 0..tree_map.len() {
        for j in 0..tree_map[i].len() {
            scenic_scores.push(get_scenic_score(&tree_map, i, j));
        }
    }
    return *scenic_scores.iter().max().unwrap();
}

fn get_scenic_score(tree_map: &Vec<Vec<u32>>, y: usize, x: usize) -> u32 {
    return get_scenic_score_from_up(&tree_map, &y, &x)
        * get_scenic_score_from_down(&tree_map, &y, &x)
        * get_scenic_score_from_left(&tree_map[y], &x)
        * get_scenic_score_from_right(&tree_map[y], &x);
}

fn get_scenic_score_from_up(tree_map: &Vec<Vec<u32>>, y: &usize, x: &usize) -> u32 {
    let tree_height = tree_map[*y][*x];
    for i in (0..*y).rev() {
        if tree_map[i][*x] >= tree_height {
            return (*y - i) as u32;
        }
    }
    return *y as u32;
}

fn get_scenic_score_from_down(tree_map: &Vec<Vec<u32>>, y: &usize, x: &usize) -> u32 {
    let tree_height = tree_map[*y][*x];
    for i in (y + 1)..tree_map.len() {
        if tree_map[i][*x] >= tree_height {
            return (i - y) as u32;
        }
    }
    return (tree_map.len() - 1 - y) as u32;
}

fn get_scenic_score_from_left(row: &Vec<u32>, x: &usize) -> u32 {
    let tree_height = row[*x];
    for i in (0..*x).rev() {
        if row[i] >= tree_height {
            return (x - i) as u32;
        }
    }
    return *x as u32;
}

fn get_scenic_score_from_right(row: &Vec<u32>, x: &usize) -> u32 {
    let tree_height = row[*x];
    for i in (x + 1)..row.len() {
        if row[i] >= tree_height {
            return (i - x) as u32;
        }
    }
    return (row.len() - 1 - x) as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible_trees() {
        let mut tree_map = vec![vec![1]];
        assert_eq!(1, count_visible_trees(tree_map));

        tree_map = vec![
            vec![1,1,1],
            vec![1,2,1],
            vec![1,1,1],
        ];
        assert_eq!(9, count_visible_trees(tree_map));

        tree_map = vec![
            vec![5,2,7],
        ];
        assert_eq!(false, is_tree_visible_from_left(&tree_map[0], &(1usize)));
        assert_eq!(false, is_tree_visible_from_right(&tree_map[0], &(1usize)));

        tree_map = vec![
            vec![3,3,2],
            vec![5,2,7],
            vec![4,5,3],
        ];
        assert_eq!(false, is_tree_visible_from_up(&tree_map, &1usize, &1usize));
        assert_eq!(false, is_tree_visible_from_down(&tree_map, &1usize, &1usize));

        tree_map = vec![
            vec![3,3,2],
            vec![5,2,7],
            vec![4,5,3],
        ];
        assert_eq!(8, count_visible_trees(tree_map));

        tree_map = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0]
        ];
        assert_eq!(21, count_visible_trees(tree_map));
    }

    #[test]
    fn test_get_scenic_score() {
        let mut tree_map = vec![vec![1]];
        assert_eq!(0, get_scenic_score(&tree_map, 0, 0));

        tree_map = vec![
            vec![1,1,1],
            vec![1,2,1],
            vec![1,1,1],
        ];
        assert_eq!(1, get_scenic_score(&tree_map, 1, 1));

        tree_map = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0]
        ];
        assert_eq!(0, get_scenic_score(&tree_map, 0, 1));
        assert_eq!(0, get_scenic_score(&tree_map, 1, 0));
        assert_eq!(0, get_scenic_score(&tree_map, 4, 1));
        assert_eq!(0, get_scenic_score(&tree_map, 2, 4));

        assert_eq!(1, get_scenic_score_from_up(&tree_map, &1, &2));
        assert_eq!(2, get_scenic_score_from_down(&tree_map, &1, &2));
        assert_eq!(1, get_scenic_score_from_left(&tree_map[1], &2));
        assert_eq!(2, get_scenic_score_from_right(&tree_map[1], &2));
        assert_eq!(4, get_scenic_score(&tree_map, 1, 2));

        assert_eq!(2, get_scenic_score_from_up(&tree_map, &3, &2));
        assert_eq!(1, get_scenic_score_from_down(&tree_map, &3, &2));
        assert_eq!(2, get_scenic_score_from_left(&tree_map[3], &2));
        assert_eq!(2, get_scenic_score_from_right(&tree_map[3], &2));
        assert_eq!(8, get_scenic_score(&tree_map, 3, 2));
    }

    #[test]
    fn test_asd() {
        let mut tree_map = vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0]
        ];
        assert_eq!(8, get_highest_scenic_score(tree_map));

        tree_map = vec![
            vec![3]
        ];

        assert_eq!(0, get_highest_scenic_score(tree_map));

        tree_map = vec![
            vec![3,3,3],
            vec![3,4,3],
            vec![3,3,3],
        ];

        assert_eq!(1, get_highest_scenic_score(tree_map));

        tree_map = vec![
            vec![3,3,3,3,3],
            vec![3,4,4,4,3],
            vec![3,4,5,4,3],
            vec![3,4,4,4,3],
            vec![3,3,3,3,3],
        ];

        assert_eq!(16, get_highest_scenic_score(tree_map));

        tree_map = vec![
            vec![3,3,3,3,3],
            vec![3,4,5,4,3],
            vec![3,5,4,4,3],
            vec![3,4,4,5,3],
            vec![3,3,3,3,3],
        ];

        assert_eq!(12, get_highest_scenic_score(tree_map));

        tree_map = vec![
            vec![3,3,3,3,3],
            vec![3,4,5,4,3],
            vec![3,5,5,4,3],
            vec![3,4,4,5,3],
            vec![3,3,3,3,3],
        ];

        assert_eq!(9, get_highest_scenic_score(tree_map));

        tree_map = vec![                  //     |     \\
            vec![1,2,2,2,0,2,2,1,2,3,1,1,2,0,0,2,1,0,3,3,3,0,4,],
            vec![2,1,2,1,2,2,1,2,2,0,0,0,2,2,1,3,1,1,3,2,3,4,4,],
            vec![2,0,0,0,1,0,2,3,3,1,1,0,1,3,2,0,3,2,2,0,1,0,4,],
            vec![1,2,0,1,0,1,1,1,0,2,0,3,3,3,3,1,1,0,2,4,0,1,2,],
            vec![1,2,0,1,1,0,2,2,3,2,0,1,2,3,3,1,4,0,4,3,4,3,3,],
            vec![0,0,0,1,0,2,3,2,2,0,1,0,0,2,2,3,4,1,1,0,4,0,3,],
            vec![0,1,2,1,3,2,0,2,2,0,2,0,3,1,2,4,4,4,1,0,2,4,3,],
        ];
        assert_eq!(1, get_scenic_score_from_up(&tree_map, &5, &16));
        assert_eq!(1, get_scenic_score_from_down(&tree_map, &5, &16));
        assert_eq!(16, get_scenic_score_from_left(&tree_map[5], &16));
        assert_eq!(4, get_scenic_score_from_right(&tree_map[5], &16));

        println!("{:?}", &tree_map[0][..=16]);
        assert_eq!(0, get_scenic_score_from_up(&tree_map, &0, &16));
        assert_eq!(1, get_scenic_score_from_down(&tree_map, &0, &16));
        assert_eq!(1, get_scenic_score_from_left(&tree_map[0], &16));
        assert_eq!(2, get_scenic_score_from_right(&tree_map[0], &16));

        //assert_eq!(16*4*1*1, get_highest_scenic_score(tree_map));
    }
}
