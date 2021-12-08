use std::fs;
use std::collections::HashSet;

static INPUT_FILE: &str = "input.txt";

struct DisplayInput
{
    scrambled_digits: Vec<String>,
    output_digits: Vec<String>
}

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);

    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Vec<DisplayInput>
{
    input
    .lines()
    .map(str::trim)
    .map(|x|
        {
            let mut parts = x.split("|");

            let scrambled = parts
                .next()
                .unwrap()
                .split(" ")
                .map(String::from)
                .collect();
            
            let digits = parts
                .next()
                .unwrap()
                .split(" ")
                .map(String::from)
                .collect();

            DisplayInput {
                scrambled_digits: scrambled,
                output_digits: digits,
            }
        } )
    .collect()
}

fn part2(input: &[DisplayInput]) -> usize
{
    let mut sum = 0;
    for i in input
    {
        // println!("{:?} | {:?}", i.scrambled_digits, i.output_digits);

        // digit - segments     - len
        // 1     - c f          - 2
        // 7     - a c f        - 3
        // 4     - b d c f      - 4 // can determine b here, and can use it for 5 digits
                                    // since d is always set
        // 5     - a b d f g    - 5 // if len = 5and c == false && f == true, is 5 else 2
        // 2     - a   c d e g    - 5 // c == true && f == false
        // 3     - a   c d   f g    - 5 // c == true && f == true
        // 6     - a b   d e f g  - 6 // if len = 6 and c == false, is 6
                                    // and c can be determined by 1 7 4 digits
        // 0     - a b c   e f g  - 6
        // 9     - a b c d f g  - 6
        // 8     - a b c d e f g- 7


        // expanded truth table for len 5 and len 6
        // digit -- len == 5
        // 5 - !c && f && b
        // 2 - c && !f
        // 3 - c && f

        // 6 c == false
        // 0 d == false
        // 9 d == true && c == true

        // determine c
        // can determine the c flag by the one which, out of the len 6 digits
        // is only true twice

        // determine f
        // can determine f by the other one in the digit of len 2 that isn't c

        // determine b
        // pick out the two flags that are not in digit 1: b and d
        // out of the len 5 segments, find the one that is not always true
        // remaining one is b

        // determine d
        // d can be determined by the 4 digit, excluding c and f and b

        // start with the easy ones
        fn get_digit_ez(digit: &String) -> Option<usize>
        {
            match digit.len() {
                7 => Some(8),
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                _ => None,
            }
        }

        let mut c_flag : HashSet<char> = HashSet::new();
        let mut f_flag : HashSet<char> = HashSet::new();
        let mut b_flag : HashSet<char> = HashSet::new();
        let mut d_flag : HashSet<char> = HashSet::new();

        // determine candidates for c and f from the 1 digit
        // also store what b CANNOT be
        for disp in &i.scrambled_digits
        {
            if let Some(known) = get_digit_ez(disp)
            {
                let chars : Vec<char> = disp.chars().collect();

                if known == 1
                {
                    // set c and f candidates
                    for c in chars{
                        c_flag.insert(c);
                        f_flag.insert(c);
                    }
                }
            }
        }

        // determine candidates for d and b from the 4 digit after knowing the 1 digit
        for disp in &i.scrambled_digits
        {
            if let Some(known) = get_digit_ez(disp)
            {
                let chars : Vec<char> = disp.chars().collect();

                if known == 4
                {
                    // set c and f candidates
                    for c in chars{
                        if c_flag.contains(&c) || f_flag.contains(&c)
                        {
                            continue;
                        }

                        d_flag.insert(c); // d or b
                        b_flag.insert(c); // d or b
                    }
                }
            }
        }

        // solve for b (d is always true when len == 5)
        for disp in i.scrambled_digits
                        .iter()
                        .filter(|x| x.len() == 5)
        {
            let mut b = '\0';
            for c in d_flag.iter()
            {
                if !disp.contains(*c)
                {
                    // this char cannot be d
                    // and must be b
                    b = *c;
                    break;
                }
            }

            if b != '\0'
            {
                d_flag.remove(&b);

                // solved b and d
                break;
            }
        }

        // solve for c
        for c in c_flag.iter()
        {
            let count_true =
                i.scrambled_digits
                .iter()
                .filter(|x| x.len() == 6)
                .filter(|x| x.contains(*c))
                .count();

            if count_true == 2
            {
                // c is the c_flag
                f_flag.remove(c);
                break;
            }
        }

        let f_char = f_flag
            .iter()
            .next()
            .unwrap();

        c_flag.remove(f_char);
        
        let c_char = c_flag
            .iter()
            .next()
            .unwrap();

        // solve for d from b
        let b_char = b_flag
            .iter()
            .next()
            .unwrap();

        let d_char = d_flag
            .iter()
            .next()
            .unwrap();

        println!("SOLVED: c: {} f: {} b: {} d: {}", c_char, f_char, b_char, d_char);

        fn solve(digit: &String, b_char: char, c_char: char, d_char: char, f_char: char) -> usize
        {
            if let Some(ez) = get_digit_ez(digit)
            {
                return ez;
            }
            else
            {
                if digit.len() == 5
                {
                    if !digit.contains(c_char) && digit.contains(f_char) && digit.contains(b_char)
                    {
                        return 5;
                    }

                    if digit.contains(c_char) && !digit.contains(f_char)
                    {
                        return 2;
                    }

                    if digit.contains(c_char) && digit.contains(f_char)
                    {
                        return 3;
                    }
                }
                // 5 - !c && f && b
                // 2 - c && !f
                // 3 - c && f

                if (digit.len() == 6)
                {
                    if !digit.contains(c_char)
                    {
                        return 6;
                    }

                    if !digit.contains(d_char)
                    {
                        return 0;
                    }

                    if digit.contains(c_char) && digit.contains(d_char)
                    {
                        return 9;
                    }
                }

                // 6 c == false
                // 0 d == false
                // 9 d == true && c == true

            }

            println!("BAD DIGIT");

            0
        }

        // ok... solve it

        let mut num = 0;

        for digit in &i.output_digits
        {
            let n = solve(digit, *b_char, *c_char, *d_char, *f_char);
            num *= 10;
            num += n;
        }

        println!("NUM {}", num);

        sum += num;
    }

    sum
}

fn part1(input: &[DisplayInput]) -> usize
{
    // start with the easy ones
    fn get_digit_ez(digit: &String) -> Option<usize>
    {
        match digit.len() {
            7 => Some(8),
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            _ => None,
        }
    }

    let mut count = 0;
    for i in input
    {
        println!("{:?} | {:?}", i.scrambled_digits, i.output_digits);
        
        for o in &i.output_digits
        {
            if let Some(_) = get_digit_ez(o)
            {
                count += 1;
            }
        }
    }

    count
}



#[test]
fn example()
{
    let input =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |    fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |    cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |    efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |    gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |    gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |    cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |    ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |    gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |    fgae cfgab fg bagce";
    let input = parse_input(input);
    assert_eq!(part1(&input), 26);
    assert_eq!(part2(&input), 61229);
}