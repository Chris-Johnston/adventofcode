use std::fs;
use std::vec;
use std::string::String;

// too high 4324

fn main() {
    let input_file = "/home/chris/Git/adventofcode/2019/8/day8/input.txt";
    let data = fs::read_to_string(input_file)
        .expect("Failed to read file");

    let width_x = 25;
    let width_y = 6;
    
    // let data = "123456789012";
    // let width_x = 2;
    // let width_y = 3;

    // let data = String::from("0222112222120000");
    // let width_x = 2;
    // let width_y = 2;

    let layer_size = width_x * width_y;

    let mut image = Vec::new();
    parse_image_data_str(data, &mut image);

    println!("{:?}", image);

    let mut min_zero_layer = 0;
    let mut min_zeros = 99;

    let mut num_one = 0;
    let mut num_two = 0;

    // default transparent
    let mut rendered_image = vec![2; layer_size];

    for (layer_index, _) in (0..image.len()).step_by(layer_size).enumerate().rev()
    {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for idx in (layer_index*layer_size)..(layer_index*layer_size+layer_size)
        {
            // match image[idx]
            // {
            //     0 => zeros += 1,
            //     1 => ones += 1,
            //     2 => twos += 1,
            //     _ => continue,
            // };

            let render_index = idx % layer_size;
            let rendered_value = rendered_image[render_index];

            match image[idx]
            {
                0 => {
                    zeros += 1;
                    rendered_image[render_index] = 0;
                },
                1 => {
                    ones += 1;
                    rendered_image[render_index] = 1;
                },
                2 => {
                    twos += 1;
                },
                _ => continue,
            };
        }
        
        println!("layer {} 0 = {} 1 = {} 2 = {}", layer_index, zeros, ones, twos);

        if zeros < min_zeros
        {
            min_zeros = zeros;
            min_zero_layer = layer_index;
            num_one = ones;
            num_two = twos;
        }
    }

    println!("layer {} had the fewest zeros ({}), answer {}", min_zero_layer, min_zeros, num_one * num_two);

    for y in 0..width_y
        {
            for x in 0..width_x
            {
            let idx = y * width_x + x;
            let idx = rendered_image[idx];
            match idx
            {
                1 => print!("□"),
                _ => print!(" "),
            };
        }
        println!("");
        }
}

fn parse_image_data_str(data: String, output: &mut Vec<u32>)
{
    for c in data.trim().chars()
    {
        let v : u32 = c.to_digit(10)
            .expect("failed to parse value");
        output.push(v);
    }
}