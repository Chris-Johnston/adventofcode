#[macro_use] extern crate lazy_static;
use std::fs;
use std::string;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

static EXAMPLE_INPUT: &str = 
"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

const EXAMPLE_ANSWER_1: usize = 2;
const EXAMPLE_ANSWER_2: usize = 2;

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("failed to read file");

    let example_solution = solution(EXAMPLE_INPUT)
        .expect("no result");
    
    println!("Example Part 1 {} = {}", EXAMPLE_ANSWER_1, example_solution);
    assert!(EXAMPLE_ANSWER_1 == example_solution);

    let answer = solution(&input)
        .expect("no result");
    
    println!("Answer Part 1 {}", answer);
    // assert!(answer == 42069);

    let invalid_test_data =
    "eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
    
    iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946
    
    hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
    
    hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007";
    let test_data = solution_part2(&invalid_test_data).expect("no result");
    assert!(test_data == 0);

    let valid_test_data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

pid:012345678 hgt:190cm ecl:grn iyr:2012 eyr:2030 byr:2002
hcl:#123a2f
";

    let test_data = solution_part2(&valid_test_data).expect("no result");
    println!("valid test {}", test_data);
    assert!(test_data == 4);

    // part 2
    let example_solution = solution_part2(EXAMPLE_INPUT)
        .expect("no result");

    println!("Example Part 2 {} = {}", EXAMPLE_ANSWER_2, example_solution);
    assert!(EXAMPLE_ANSWER_2 == example_solution);

    let answer = solution_part2(&input)
        .expect("no result");
    
    println!("Answer Part 2 {}", answer);
    assert!(answer != 177);
    assert!(answer != 183); // too high
    assert!(answer < 183); // too high
    assert!(answer != 176);
}

#[derive(Debug)]
struct Passport{
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport
{
    fn new() -> Passport
    {
        Passport {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: None
        }
    }

    fn set_field(&mut self, field: &str, value: &str)
    {
        match field
        {
            "byr" => self.byr = String::from(value),
            "iyr" => self.iyr = String::from(value),
            "eyr" => self.eyr = String::from(value),
            "hgt" => self.hgt = String::from(value),
            "hcl" => self.hcl = String::from(value),
            "ecl" => self.ecl = String::from(value),
            "pid" => self.pid = String::from(value),
            "cid" => self.cid = Some(String::from(value)),
            _ => panic!("undefined field {} with value {}", field, value),
        };
    }

    fn get_valid(&self) -> bool
    {
        !self.byr.is_empty() &&
        !self.iyr.is_empty() &&
        !self.eyr.is_empty() &&
        !self.hgt.is_empty() &&
        !self.hcl.is_empty() &&
        !self.ecl.is_empty() &&
        !self.pid.is_empty()
    }

    // valid check for p2
    fn extended_valid(&self) -> bool
    {
        if !self.get_valid() { return false; }

        println!("{:?}", self);

        // the condition gauntlet
        match self.byr.parse::<usize>()
        {
            Ok(x) => {
                if x < 1920 || x > 2002
                {
                    return false;
                }
            },
            _ => return false,
        };

        match self.iyr.parse::<usize>()
        {
            Ok(x) => {
                if x < 2010 || x > 2020
                {
                    return false;
                }
            },
            _ => return false,
        };

        match self.eyr.parse::<usize>()
        {
            Ok(x) => {
                if x < 2020 || x > 2030
                {
                    return false;
                }
            },
            _ => return false,
        };

        lazy_static! {
            static ref regex_height: Regex = Regex::new(r"([0-9]+)(in|cm)").unwrap();
        }

        // something borked with the match here, not going to debug
        let mut r = true;
        match regex_height.captures(&self.hgt)
        {
            Some(x) => {
                let val = match x[1].parse::<usize>()
                {
                    Ok(x) => x,
                    _ => {
                        r = false;
                        0
                    },
                };
                println!("val {}", val);
                match &x[2] {
                    "cm" => {
                        println!("cm");
                        if val < 150 || val > 193 { println!("bad!!!!"); r = false}
                    },
                    "in" => {
                        println!("in");
                        if val < 59 || val > 76 { println!("bad!!"); r = false}
                    },
                    _ =>{println!("bad!! {:?}", &x); r = false},
                };
            },
            None => {
                println!("bad!!");
                r = false;
            },
        };  
        if !r { return false; }

        println!("wait...");

        lazy_static! {
            static ref regex_hair: Regex = Regex::new(r"\#[a-f0-9]{6}").unwrap();
        }
        if !regex_hair.is_match(&self.hcl)
        {
            return false;
        }

        lazy_static! {
            static ref regex_eye: Regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        }
        if !regex_eye.is_match(&self.ecl)
        {
            println!("ecl bad {}", self.ecl);
            return false;
        }
        println!("ecl good {}", self.ecl);
        lazy_static! {
            static ref regex_pid: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        if !regex_pid.is_match(&self.pid)
        {
            // println!("pid bad {}", self.pid);
            return false;
        }

        true
    }
}

fn solution(input: &str) -> Option<usize>
{
    // I feel like if I split by \n\n and then grepped for each of the dimensions I
    // could quickly get away with this
    // but I have a feeling that I'll actually need this data later
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{3}):([a-z0-9#]+)").unwrap();
    }

    let mut entries = Vec::new();

    // split by blank lines, inside that nothing matters
    for section in input.split("\n\n")
    {
        let mut entry = Passport::new();
        for capture in RE.captures_iter(section)
        {
            let field = &capture[1];
            let val = &capture[2];

            entry.set_field(field, val);
        }
        // println!("Pushing {:?}", entry);
        entries.push(entry);
    }

    let result = entries
        .iter()
        .filter(|&x| x.get_valid())
        .count();

    Some(result)
}

fn solution_part2(input: &str) -> Option<usize>
{
    // I feel like if I split by \n\n and then grepped for each of the dimensions I
    // could quickly get away with this
    // but I have a feeling that I'll actually need this data later
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{3}):([a-z0-9#]+)").unwrap();
    }

    let mut entries = Vec::new();

    // split by blank lines, inside that nothing matters
    for section in input.split("\n\n")
    {
        let mut entry = Passport::new();
        for capture in RE.captures_iter(section)
        {
            let field = &capture[1];
            let val = &capture[2];

            entry.set_field(field, val);
        }
        // println!("Pushing {:?}", entry);
        entries.push(entry);
    }

    let result = entries
        .iter()
        .filter(|&x| x.extended_valid())
        .count();

    Some(result)
}