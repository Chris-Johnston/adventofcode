use std::fs;

static INPUT_FILE: &str = "input.txt";

fn main() {
    let example_input = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    let answer_input = fs::read_to_string(INPUT_FILE).unwrap();

    let input : Vec<usize> = 
    answer_input
    // example_input
    .lines()
    .map(|x| x.trim().parse::<usize>().unwrap())
    .collect();

    let mut prev : usize = 0;
    let mut count : usize = 0;

    for (i, v) in input.iter().enumerate()
    {
        if i != 0
        {
            if v > &prev
            {
                count += 1;
            }
        }

        prev = *v;
        if i == 0
        {
            continue;
        }
    }

    println!("part 1 count {}", count);

    count = 0;
    prev = 0;

    for (i, _) in input.iter().enumerate()
    {
        let mut window = 0;
        for wi in i..i+3
        {
            if wi < input.len()
            {
                window += input[wi];
            }
        }

        // println!("window {}", window);

        if i != 0
        {
            if window > prev
            {
                count += 1;
            }
        }

        prev = window;
    }

    println!("part 2 count {}", count);
}
