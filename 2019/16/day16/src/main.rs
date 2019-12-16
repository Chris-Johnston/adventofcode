use std::vec::Vec;

fn main()
{
    // solution();
    // example_part2();
    solution_part2();
}

fn example()
{

    let mut digits = Vec::new();
    // get_digits(15243, &mut digits);

    // let result = get_value(vec![9, 8, 7, 6, 5], vec![1, 2, 3]);
    // println!("result {}", result, 1);
    // assert!(2 == result);

    // while each elem in output array uses same input array
    // repeating pattern depends on which output element
    let base_pattern = vec![0, 1, 0, -1];
    // repeat each value in the pattern a number of times equal to the position
    // in the output list being considered
    // skip first value exactly once, so shift left


    get_digits(12345678, &mut digits);
    println!("digits {:?}", digits);
    for phase in 0..4
    {
        println!("phase {}", phase);
        let mut phase_digits = Vec::new();
        for d in (0..digits.len())
        {
            let val = get_value(&digits, &base_pattern, d as isize);
            phase_digits.push(val);
            println!("val: {}", val);
        }

        println!("digits: {:?}", phase_digits);

        digits = phase_digits;
    }
}

fn solution()
{
    let mut digits : Vec<isize> = "59717513948900379305109702352254961099291386881456676203556183151524797037683068791860532352118123252250974130706958763348105389034831381607519427872819735052750376719383812473081415096360867340158428371353702640632449827967163188043812193288449328058464005995046093112575926165337330100634707115160053682715014464686531460025493602539343245166620098362467196933484413717749680188294435582266877493265037758875197256932099061961217414581388227153472347319505899534413848174322474743198535953826086266146686256066319093589456135923631361106367290236939056758783671975582829257390514211329195992209734175732361974503874578275698611819911236908050184158"
        .chars()
        .map(|x| x.to_digit(10).expect("erro") as isize)
        .collect();

    let base_pattern = vec![0, 1, 0, -1];

    println!("digits {:?}", digits);
    for phase in 0..100
    {
        println!("phase {}", phase);
        let mut phase_digits = Vec::new();
        for d in (0..digits.len())
        {
            let val = get_value(&digits, &base_pattern, d as isize);
            phase_digits.push(val);
            // println!("val: {}", val);
        }

        // 75719763 wrong
        // digits: [6, 3, 7, 9, 4, 4, 0, 7
        // correct

        digits = phase_digits;
    }
    println!("digits: {:?}", digits);
}

fn example_part2()
{
    let mut start_digits : Vec<isize> = "03036732577212944063491565474664"
        .chars()
        .map(|x| x.to_digit(10).expect("erro") as isize)
        .collect();
    let mut offset = 0;
    for d in start_digits[..7].iter()
    {
        offset *= 10;
        offset += d;
    }
    println!("offset {}", offset);
    let digits_real_len = start_digits.len();
    let digits_len = digits_real_len * 10000;

    let mut digits = Vec::new();
    for _ in 0..10000
    {
        for val in &start_digits
        {
            digits.push(*val);
        }
    }

    let base_pattern = vec![0, 1, 0, -1];

    // println!("digits {:?}", digits);
    for phase in 0..100
    {
        // println!("phase {}", phase);
        // let mut phase_digits = Vec::new();
        // for d in (offset..digits.len() as isize)
        // {
        //     if d % 1000 == 0
        //     {
        //         println!("phase {} idx {} / {}", phase, d, digits.len());
        //     }

        //     let val = get_value_part2(&digits, &base_pattern, d as isize, offset as usize);
        //     phase_digits.push(val);
        // }
        // digits = phase_digits;
        
        let mut partial_sum = 0;
        for idx in offset..digits_len as isize
        {
            partial_sum += digits[idx as usize];
        }

        for j in offset..digits_len as isize
        {
            let mut t = partial_sum;
            partial_sum -= digits[j as usize];
            digits[j as usize] = (t % 10).abs();
        }

    }
    println!("expected 84462026");
    // println!("digits: {:?}", digits);
    let offset_end = offset as usize + 8;
    println!("digits sub {:?}", &digits[offset as usize..offset_end]);
}

fn solution_part2()
{
    let mut start_digits : Vec<isize> = "59717513948900379305109702352254961099291386881456676203556183151524797037683068791860532352118123252250974130706958763348105389034831381607519427872819735052750376719383812473081415096360867340158428371353702640632449827967163188043812193288449328058464005995046093112575926165337330100634707115160053682715014464686531460025493602539343245166620098362467196933484413717749680188294435582266877493265037758875197256932099061961217414581388227153472347319505899534413848174322474743198535953826086266146686256066319093589456135923631361106367290236939056758783671975582829257390514211329195992209734175732361974503874578275698611819911236908050184158"
        .chars()
        .map(|x| x.to_digit(10).expect("erro") as isize)
        .collect();
    let mut offset = 0;
    for d in start_digits[..7].iter()
    {
        offset *= 10;
        offset += d;
    }
    println!("offset {}", offset);
    let digits_real_len = start_digits.len();
    let digits_len = digits_real_len * 10000;

    let mut digits = Vec::new();
    for _ in 0..10000
    {
        for val in &start_digits
        {
            digits.push(*val);
        }
    }

    let base_pattern = vec![0, 1, 0, -1];

    // println!("digits {:?}", digits);
    for phase in 0..100
    {
        println!("phase {}", phase);
        // let mut phase_digits = Vec::new();
        // for d in (offset..digits.len() as isize)
        // {
        //     if d % 1000 == 0
        //     {
        //         println!("phase {} idx {} / {}", phase, d, digits.len());
        //     }

        //     let val = get_value_part2(&digits, &base_pattern, d as isize, offset as usize);
        //     phase_digits.push(val);
        // }
        // digits = phase_digits;
        
        // get the sum of all digits for this iteration
        // since the offset is so large, the base modifier will be
        //            v-- offset index
        // [0, 0, .., 1, 1, 1, 1, ..., 1]
        // [0, 0, .., 0, 1, 1, 1, ..., 1]
        // [0, 0, .., 0, 0, 1, 1, ..., 1]
        // so the sum can be taken, and each digit can just subtract that index
        let mut row_sum = 0;
        for idx in offset..digits_len as isize
        {
            row_sum += digits[idx as usize];
        }

        for d in offset..digits_len as isize
        {
            let mut temp_sum = row_sum;
            row_sum -= digits[d as usize];
            digits[d as usize] = (temp_sum % 10).abs();
        }

    }
    // 7 7 2 4 7 5 3 8
    // println!("digits: {:?}", digits);
    let offset_end = offset as usize + 8;
    println!("p2 sub {:?}", &digits[offset as usize..offset_end]);
}

fn get_digits(num: usize, output: &mut Vec<isize>)
{
    let mut x = num;
    while x > 0
    {
        let y = x % 10;
        output.insert(0, y as isize);
        x /= 10;
    }
}

fn get_value(input: &Vec<isize>, pattern: &Vec<isize>, iter: isize) -> isize
{
    let mut result = 0;
    for (idx, val) in input.iter().enumerate()
    {
        let pattern_idx : usize = (((idx as isize + 1) / (iter + 1)) as usize % pattern.len()) as usize;
        // println!("+ {} * {}", val, pattern[pattern_idx]);
        result += *val as isize * pattern[pattern_idx];
    }
    (result % 10).abs()
}

fn get_value_part2(input: &Vec<isize>, pattern: &Vec<isize>, iter: isize, skip: usize) -> isize
{
    let mut result = 0;
    for (idx, val) in input[skip..].iter().enumerate()
    {
        let pattern_idx : usize = (((idx as isize + 1) / (iter + 1)) as usize % pattern.len()) as usize;
        // println!("+ {} * {}", val, pattern[pattern_idx]);
        result += *val as isize * pattern[pattern_idx];
    }
    (result % 10).abs()
}