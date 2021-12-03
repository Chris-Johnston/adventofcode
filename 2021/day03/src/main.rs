use std::fs;

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = &parse_input(&in_text);

    let answer = part1(input, 12);
    println!("part 1 {}", answer);

    let answer = part2(input, 12);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Vec<Vec<u32>>
{
    input
    .lines()
    .map(str::trim)
    .map(|x| {
        x.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
    })
    .collect()
}

fn part1(input: &Vec<Vec<u32>>, width: usize) -> u32
{
    let mut bit_count = vec![0; width];
    for num in input
    {
        for (idx, digit) in num.iter().enumerate()
        {
            if *digit == 1
            {
                bit_count[idx] += 1;
            }
        }
    }

    // most common bits
    let mut gamma = 0;

    // least common bit in each position, or inverse of gamma
    let mut epsilon = 0;

    let num_rows = input.len();

    for (idx, count) in bit_count.iter().enumerate()
    {
        if *count > num_rows - *count
        {
            gamma |= 1 << (width - 1 - idx);
        }
        else
        {
            epsilon |= 1 << (width - 1 - idx);
        }

    }

    gamma * epsilon
}

fn part2(input: &Vec<Vec<u32>>, width: usize) -> u32
{
    let mut filtered_inputs_oxy : Vec<Vec<u32>> = input.to_vec();
    let mut filtered_inputs_co2 : Vec<Vec<u32>> = input.to_vec();

    let num_rows = input.len();

    // for (idx, count) in bit_count.iter().enumerate()
    for idx in 0..width
    {
        let mut count_oxy = 0;
        for row in &filtered_inputs_oxy
        {
            if row[idx] == 1
            {
                count_oxy += 1;
            }
        }

        let mut count_co2 = 0;
        for row in &filtered_inputs_co2
        {
            if row[idx] == 1
            {
                count_co2 += 1;
            }
        }

        println!("idx {} count oxy {} count co2 {}", idx, count_oxy, count_co2);
        println!("oxy - len is {}", filtered_inputs_oxy.len());
        for row in &filtered_inputs_oxy
        {
            println!("{:?}", row);
        }

        // println!("co2 - len is {}", filtered_inputs_co2.len());
        // for row in &filtered_inputs_co2
        // {
        //     println!("{:?}", row);
        // }

        // let num_rows = filtered_inputs_oxy.len();

        if filtered_inputs_oxy.len() > 1
        {
            let mut bit = 0;
            if count_oxy >= filtered_inputs_oxy.len() - count_oxy
            {
                bit = 1;
            }
            println!("bit {} -> {} > {}", bit, count_oxy, filtered_inputs_oxy.len() - count_oxy);

            filtered_inputs_oxy = filtered_inputs_oxy
                    .iter()
                    .filter(|&r| {
                        r[idx] == bit
                    })
                    .map(|x| x.clone())
                    .collect();
        }

        // let num_rows = filtered_inputs_co2.len();

        if filtered_inputs_co2.len() > 1
        {
            let mut bit = 0;
            if count_co2 < filtered_inputs_co2.len() - count_co2
            {
                bit = 1;
            }

            filtered_inputs_co2 = filtered_inputs_co2
                    .iter()
                    .filter(|&r| {
                        r[idx] == bit
                    })
                    .map(|x| x.clone())
                    .collect();
        }
    }

    let mut oxy = 0;
    let mut co2 = 0;

    for (idx, digit) in filtered_inputs_oxy[0].iter().enumerate()
    {
        oxy |= digit << (width - 1 - idx);
    }

    for (idx, digit) in filtered_inputs_co2[0].iter().enumerate()
    {
        co2 |= digit << (width - 1 - idx);
    }

    println!("oxy {:b} co2 {:b}", oxy, co2);

    oxy * co2
}



#[test]
fn example()
{
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    let input = parse_input(input);
    assert_eq!(part1(&input, 5), 198);
     assert_eq!(part2(&input, 5), 230);
}