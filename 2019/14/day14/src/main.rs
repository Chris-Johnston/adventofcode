// day 14
// reactions x chemicals -> output chemicals
// ORE = raw material input
// FUEL = target output

use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Hash, Debug, Copy)]
struct Chemical
{
    amount: isize,
    name: String,
}

impl Chemical
{
    fn parse(&mut self, chemical: String)
    {
        let parts : Vec<_> = chemical.trim().split(" ").collect();
        self.amount = parts.get(0)
            .unwrap()
            .parse()
            .expect("Couldn't parse the amount.");
        self.name = String::from(*parts.get(1).unwrap());
    }
}

fn main() {
    example();
}

fn example()
{
    let input = 
"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    let mut mapping : HashMap<Chemical, Vec<Chemical>> = HashMap::new();

    parse_input(String::from(input), &mut mapping);
    let result = calc_ore(&mut mapping);
    println!("solution: {}", result);

    // println!("mapping {:?}", mapping);
    assert!(result == 31);
}

fn calc_ore(mapping: &mut HashMap<Chemical, Vec<Chemical>>) -> isize
{
    let mut required_amounts : HashMap<String, isize> = HashMap::new();
    required_amounts.insert("FUEL".to_string(), 1);
    let mut ingredients_list = VecDeque::new();
    ingredients_list.push_back("FUEL".to_string());

    while !ingredients_list.is_empty()
    {
        let chem = ingredients_list.pop_front()
            .expect("list empty");
        println!("working on {}", chem);
        let mut reaction_key : Chemical;
        // find the reaction
        for key in mapping.keys()
        {
            if key.name == chem
            {
                reaction_key = *key;
                break;
            }
        }
        let mut produced = &reaction_key.amount;
        // get the min required amount produced
        let mut required = required_amounts[&reaction_key.name];

        // ceil division
        let mut multiplier = required / produced +
            if required % produced != 0 {
                1
            }
            else
            {
                0
            };

        let mut inputs = mapping.get(&reaction_key).expect("not found");
        for input in inputs
        {
            if required_amounts.contains_key(&input.name)
            {
                required_amounts.insert(input.name.to_string(), multiplier * input.amount);
                ingredients_list.push_back(input.name.to_string());
            }
            else
            {
                let current = required_amounts.get(&input.name)
                    .expect("couldn't find the key");
                required_amounts.insert(input.name.to_string(), multiplier * input.amount + current);
            }
        }        
    }


    // let mut starting = mapping.get(&Chemical { amount: 1, name: String::from("FUEL")})
    //     .expect("couldn't get the fuel entry");

    

    0
}

fn parse_input(input: String, mapping: &mut HashMap<Chemical, Vec<Chemical>>)
{
    for line in input.lines()
    {
        let split : Vec<_> = line.split("=>")
            .collect();

        let inputs = split.get(0).unwrap();
        let outputs = split.get(1).unwrap();

        let mut o = Chemical { amount : -1, name : String::from("123") };
        o.parse(outputs.to_string());
        // println!("output: {:?}", o);

        let mut parsed_inputs = Vec::new();

        for x in inputs.split(",")
        {
            let mut chem = Chemical { amount : -1, name : String::from("123") };
            chem.parse(x.to_string());

            // println!("input parsed: {:?}", chem);
            parsed_inputs.push(chem);
        }

        mapping.insert(o, parsed_inputs);
    }
}

