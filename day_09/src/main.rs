use std::fs::read_to_string;

fn main() {
    let file_path = "./input/day_09.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let all: Vec<_> = liness.iter().map(|x| part_1(x)).collect();
    println!("{:?}", all.iter().sum::<i64>());

    let all2: Vec<_> = liness.iter().map(|x| part_2(x)).collect();
    println!("{:?}", all2.iter().sum::<i64>());
}

fn part_1(input: &str) -> i64 {
    let n: Vec<_> = input.split_whitespace().collect();
    println!("{:?}", n);
    let nums: Vec<_> = n
        .iter()
        .map(|x| x.parse::<i64>().expect("errorrr"))
        .collect();
    let mut diff_1 = nums;
    let mut all_diff = Vec::new();
    all_diff.push(diff_1.clone());

    while diff_1.iter().sum::<i64>() != 0 {
        // println!("{:?}", diff_1);
        diff_1 = make_diff(&diff_1);
        all_diff.push(diff_1.clone())
    }

    // println!("{:?}", all_diff);
    let mut index = all_diff.len() - 1;
    all_diff[index].push(0);
    // println!("{:?}", all_diff);

    while index != 0 {
        let diff = all_diff[index][all_diff[index].len() - 1]
            + all_diff[index - 1][all_diff[index - 1].len() - 1];

        all_diff[index - 1].push(diff);
        index -= 1;
        // println!("{:?}", all_diff);
    }

    // println!("{:?}", all_diff);
    all_diff[0][all_diff[0].len() - 1]
}

fn make_diff(nums: &Vec<i64>) -> Vec<i64> {
    if nums.iter().sum::<i64>() != 0 {
        let v: Vec<_> = (0..(nums.len() - 1))
            .map(|index| nums[index + 1] - nums[index])
            .collect();
        return v;
    }
    nums.clone()
}

fn part_2(input: &str) -> i64 {
    let n: Vec<_> = input.split_whitespace().collect();
    println!("{:?}", n);
    let nums: Vec<_> = n
        .iter()
        .map(|x| x.parse::<i64>().expect("errorrr"))
        .collect();
    let mut diff_1 = nums;
    let mut all_diff = Vec::new();
    all_diff.push(diff_1.clone());

    while diff_1.iter().sum::<i64>() != 0 {
        // println!("{:?}", diff_1);
        diff_1 = make_diff(&diff_1);
        all_diff.push(diff_1.clone())
    }

    // println!("{:?}", all_diff);
    let mut index = all_diff.len() - 1;
    all_diff[index].insert(0, 0);
    // println!("{:?}", all_diff);

    while index != 0 {
        let diff = all_diff[index - 1][0] - all_diff[index][0];

        all_diff[index - 1].insert(0, diff);
        index -= 1;
        // println!("{:?}", all_diff);
    }

    // println!("{:?}", all_diff);
    all_diff[0][0]
}
