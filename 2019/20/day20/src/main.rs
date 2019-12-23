// day 20 donut maze

use std::vec::Vec;
use std::collections::{HashMap};

const WALL : usize = 200;
const PATH : usize = 201;
const VOID : usize = 202;
const ERR : usize = 255;
const WALL_CHAR : char = '#';
const PATH_CHAR : char = '.';
// anything that is 1-200 is a teleporter label

#[derive(Debug)]
struct Map
{
    data: Vec<usize>,
    width: usize,
    length: usize,
    // represents the src and dst of teleporters
    // where key is the teleporter number
    // and the value is the index
    teleport_src: HashMap<usize, usize>,
    teleport_dst: HashMap<usize, usize>,
    // set of labels where the chars are sorted
    // the index in the list is the ID of the teleporter
    teleport_labels: Vec<(char, char)>
}

impl Map
{
    pub fn new() -> Map
    {
        Map { data: Vec::new(), width: 0, length: 0, teleport_src: HashMap::new(), teleport_dst: HashMap::new(), teleport_labels: Vec::new() }
    }

    pub fn print_map(&self)
    {
        for x in 0..self.width
        {
            for y in 0..self.length
            {
                let idx = self.get_coord_index(x, y)
                    .expect("index out of bounds");
                match self.data[idx]
                {
                    WALL => print!("#"),
                    PATH => print!("."),
                    VOID => print!(" "),
                    0..=100 => print!("{}", self.data[idx] % 10),
                    _ => print!("_"),
                };
            }
            println!("");
        }
    }

    pub fn get_coord_index(&self, x: usize, y: usize) -> Option<usize>
    {
        if x > self.width || y > self.length
        {
            return None
        }
        Some(x + self.length * y)        
    }

    pub fn from_map(input: String) -> Map
    {
        let width = input.find('\n')
            .expect("couldn't find a line break in the map") + 1;
        let length = input.matches('\n').count() + 1;

        let mut v = Vec::new();
        let chars : Vec<char> = input.chars().collect();
        let mut teleport_labels = Vec::new();
        let mut teleport_src = HashMap::new();
        let mut teleport_dst = HashMap::new();

        for (idx, x) in input.chars().enumerate()
        {
            if x.is_ascii_uppercase()
            {
                // char at current position is a label
                let left_idx = idx - 1;
                let right_idx = idx + 1;
                let up_idx = if idx < width
                {
                    0
                }
                else
                {
                    idx - width
                };
                let down_idx = idx + width;

                // might have to deal with an edge case here
                // should a label be found on one line and not on the other
                // not going to worry about it rn
                // actually, this shouldn't be an issue because \n char

                // also assuming that there are no horizontal labels
                // that are in the first position

                // check horizontal labels
                if left_idx > 0 && right_idx < input.len()
                {
                    let left = chars[left_idx];
                    let right = chars[right_idx];

                    if left.is_ascii_uppercase() && right == PATH_CHAR
                    {
                        // label pointing right
                        let label = (left, x);
                        if let Some(i) = teleport_labels
                            .iter()
                            .position(|&x| x == label)
                        {
                            // add the dst of an existing one
                            teleport_dst.insert(i, right_idx);
                            v.push(i);
                            continue;
                        }
                        else
                        {
                            // insert the new one
                            let i = teleport_labels.len();
                            teleport_src.insert(i, right_idx);
                            teleport_labels.push(label);
                            v.push(i);
                            continue;
                        }
                    }
                    else if right.is_ascii_uppercase() && left == PATH_CHAR
                    {
                        // label pointing left
                        let label = (x, right);
                        if let Some(i) = teleport_labels
                            .iter()
                            .position(|&x| x == label)
                        {
                            // add the dst of an existing one
                            teleport_dst.insert(i, left_idx);
                            v.push(i);
                            continue;
                        }
                        else
                        {
                            // insert the new one
                            let i = teleport_labels.len();
                            teleport_src.insert(i, left_idx);
                            teleport_labels.push(label);
                            v.push(i);
                            continue;
                        }
                    }
                }

                if up_idx > 0 && down_idx < input.len()
                {
                    let up = chars[up_idx];
                    let down = chars[down_idx];

                    if up.is_ascii_uppercase() && down == PATH_CHAR
                    {
                        // label pointing down
                        let label = (up, x);
                        if let Some(i) = teleport_labels
                            .iter()
                            .position(|&x| x == label)
                        {
                            // add the dst of an existing one
                            teleport_dst.insert(i, up_idx);
                            v.push(i);
                            continue;
                        }
                        else
                        {
                            // insert the new one
                            let i = teleport_labels.len();
                            teleport_src.insert(i, up_idx);
                            teleport_labels.push(label);
                            v.push(i);
                            continue;
                        }
                    }
                    else if down.is_ascii_uppercase() && up == PATH_CHAR
                    {
                        // label pointing up
                        let label = (x, down);
                        if let Some(i) = teleport_labels
                            .iter()
                            .position(|&x| x == label)
                        {
                            // add the dst of an existing one
                            teleport_dst.insert(i, down_idx);
                            v.push(i);
                            continue;
                        }
                        else
                        {
                            // insert the new one
                            let i = teleport_labels.len();
                            teleport_src.insert(i, down_idx);
                            teleport_labels.push(label);
                            v.push(i);
                            continue;
                        }
                    }
                }

                v.push(ERR);
            }
            else
            {
                v.push(
                match x
                {
                    ' ' => VOID,
                    WALL_CHAR => WALL,
                    PATH_CHAR => PATH,
                    _ => ERR,
                });
            }
        }

        Map {
            data: v,
            width: width,
            length: length,
            teleport_src: teleport_src,
            teleport_dst: teleport_dst,
            teleport_labels: teleport_labels,
        }
    }
}

fn main() {
    let sample1 = Map::from_map(
        String::from(
"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "));

    sample1.print_map();
}
