// day 18
// plan:
// for each positon, BFS to all reachable keys. if a door is reached that cannot be
// passed, then do not pass
// for each of the possible keys that are found, branch off and add the distance by BFS
// and "teleport" the cursor to that spot
// evaluate all of the branches until all keys have been found
// choose the minimum branch

use std::vec::Vec;
use std::collections::{VecDeque, HashMap, HashSet};
use std::option::Option;
use std::fs;

#[derive(Debug, Clone)]
struct Map
{
    data: Vec<char>,
    width: usize,
    length: usize,
    // coordinates are the index in the data vec
    // they can be converted back to x,y
    key_coordinates: HashMap<char, usize>,
    door_coordinates: HashMap<char, usize>,
    starting_coordinate: usize,
}

impl Map
{
    pub fn from_map(input: String) -> Map
    {
        // get the width of the map
        let width = input.find('\n')
            .expect("Couldn't find a line break in the map.");
        let width = width + 1;
        let length = input.matches('\n').count() + 1;
        let mut starting_coord = 0;
        let mut key_coords = HashMap::new();
        let mut door_coords = HashMap::new();

        let mut v = Vec::new();
        for (idx, x) in input.chars().enumerate()
        {
            v.push(x);
            match x
            {
                '@' =>
                {
                    // starting coord
                    starting_coord = idx;
                },
                'a'..='z' =>
                {
                    // key
                    key_coords.insert(x, idx);
                },
                'A'..='Z' =>
                {
                    // door
                    door_coords.insert(x, idx);
                },
                // discard
                _ => {},
            }
        }

        Map { data: v, width: width, starting_coordinate: starting_coord, key_coordinates: key_coords,
            door_coordinates: door_coords, length: length }
    }

    pub fn get_coordinate_value(&self, x: usize, y: usize) -> Option<char>
    {
        let idx = Map::get_coordinate_index(&self, x, y);

        if let Some(p) = idx {
            return Some(self.data[p]);
        }

        None
    }

    pub fn get_coordinate_index(&self, x: usize, y: usize) -> Option<usize>
    {
        if x > self.width
        {
            // panic!("x index out of range");
            return None
        }
        if y > self.length
        {
            // panic!("y index out of range");
            return None
        }

        Some(x + self.width * y)
    }

    pub fn get_index_coordinate(&self, idx: usize) -> (usize, usize)
    {
        return (idx % self.width, idx / self.width);
    }

    pub fn get_starting_coordinate(&self) -> (usize, usize)
    {
        self.get_index_coordinate(self.starting_coordinate)
    }
}

#[derive(Debug, Clone)]
struct Progression
{
    distance: usize,
    x: usize,
    y: usize,
    collected_keys: Vec<char>,
}

// need to maintain a set of visited nodes
#[derive(Debug, Clone)]
struct BreadthFirstSearchNode
{
    depth: usize,
    x: usize,
    y: usize,
    // all of the required keys to get to this point
    required_keys: Vec<char>,
}

// strategy
// from the starting point
// use BFS to find the depth to all of the accessible keys
// while keeping track of keys that can only be reached by reaching other keys
// repeat this process as new keys are added

fn bfs_from_point(x: usize, y: usize, map: &Map, key_depths: &mut HashMap<char, BreadthFirstSearchNode> )
{
    let mut visited_indexes = HashSet::new();
    let mut visit_queue = VecDeque::new();
    visit_queue.push_back(
        BreadthFirstSearchNode { depth: 0, x: x, y: y, required_keys: Vec::new() }
    );
    // keyed by the keys that can be reached and their depths
    // let mut key_depths = HashMap::new();

    while !visit_queue.is_empty()
    {
        let mut node = visit_queue.pop_front()
            .expect("visit queue was empty");
        let node_index = map.get_coordinate_index(node.x, node.y);
        let point = (node.x, node.y);

        // should not have already been here
        assert!(!visited_indexes.contains(&point));
        visited_indexes.insert(point);

        // value of current index
        let this_value = map.get_coordinate_value(node.x, node.y)
            .expect("failed to get coordinate for current index");
        if this_value.is_lowercase()
        {
            // key_depths.insert(this_value, node.depth);
            key_depths.insert(this_value, node.clone());
        }

        // left
        let next_coord = (node.x - 1, node.y);
        if !visited_indexes.contains(&next_coord)
        {
            let val = map.get_coordinate_value(next_coord.0, next_coord.1)
                .unwrap_or('#');
            if is_open(val)
            {
                if val.is_ascii_uppercase() // val is a door
                {
                    node.required_keys.push(val.to_ascii_lowercase());
                }
                visit_queue.push_back(
                    BreadthFirstSearchNode {
                        depth: node.depth + 1, x: next_coord.0, y: next_coord.1,
                        required_keys: node.required_keys.clone() });
            }
        }

        // up
        let next_coord = (node.x, node.y + 1);
        if !visited_indexes.contains(&next_coord)
        {
            let val = map.get_coordinate_value(next_coord.0, next_coord.1)
                .unwrap_or('#');
            if is_open(val)
            {
                if val.is_ascii_uppercase() // val is a door
                {
                    node.required_keys.push(val.to_ascii_lowercase());
                }
                visit_queue.push_back(
                    BreadthFirstSearchNode {
                        depth: node.depth + 1, x: next_coord.0, y: next_coord.1,
                        required_keys: node.required_keys.clone() });
            }
        }

        // right
        let next_coord = (node.x + 1, node.y);
        if !visited_indexes.contains(&next_coord)
        {
            let val = map.get_coordinate_value(next_coord.0, next_coord.1)
                .unwrap_or('#');
            if is_open(val)
            {
                if val.is_ascii_uppercase() // val is a door
                {
                    node.required_keys.push(val.to_ascii_lowercase());
                }
                visit_queue.push_back(
                    BreadthFirstSearchNode {
                        depth: node.depth + 1, x: next_coord.0, y: next_coord.1,
                        required_keys: node.required_keys.clone() });
            }
        }

        // down
        let next_coord = (node.x, node.y - 1);
        if !visited_indexes.contains(&next_coord)
        {
            let val = map.get_coordinate_value(next_coord.0, next_coord.1)
                .unwrap_or('#');
            if is_open(val)
            {
                if val.is_ascii_uppercase() // val is a door
                {
                    node.required_keys.push(val.to_ascii_lowercase());
                }
                visit_queue.push_back(
                    BreadthFirstSearchNode {
                        depth: node.depth + 1, x: next_coord.0, y: next_coord.1,
                        required_keys: node.required_keys.clone() });
            }
        }
    }
}

fn find_shortest_path(map: &Map) -> Option<Progression>
{
    None
}

// fn find_shortest_path(map: &Map) -> Option<Progression>
// {
//     // todo this needs to be optimized
//     // could use Dijkstra to 
//     let start_coord = map.get_starting_coordinate();
//     let starting = Progression
//     {
//         distance: 0,
//         x: start_coord.0,
//         y: start_coord.1,
//         collected_keys: Vec::new(),
//     };

//     let mut result = Vec::new();
//     let mut best_result = 99999999;
    
//     let mut path_queue = VecDeque::new();
//     path_queue.push_back(starting);

//     while !path_queue.is_empty()
//     {
//         // println!("\n---------------------path");

//         // by popping the back, should get to creating a complete path
//         let current = path_queue.pop_front()
//             .expect("couldn't pop path queue");
//         // println!("current: {:?}", current);

//         if current.distance >= best_result
//         {
//             // skip over the ones that are already too long
//             continue;
//         }
        
//         println!("len {} collected keys: {} total keys {}", path_queue.len(), current.collected_keys.len(), map.key_coordinates.len());

//         // check if all keys have been collected
//         if map.key_coordinates.len() == current.collected_keys.len()
//         {
//             best_result = current.distance;
//             println!("found result {} = {:?}", current.distance, current.collected_keys);
//             result.push(current);
//             continue;
//         }

//         // do BFS
//         let key_depths = bfs_from_point(current.x, current.y, map);
//         // println!("key depths {:?}", key_depths);

//         for key in key_depths.keys()
//         {
//             let depth = key_depths[&key];
//             let key_idx = map.key_coordinates[&key];
//             let key_coord = map.get_index_coordinate(key_idx);

//             if !current.collected_keys.contains(&key)
//             {
//                 // println!("branching to {}", key);
//                 // if the key hasn't been visited already
//                 // branch off

//                 // collect this key
//                 let mut collected_keys = current.collected_keys.clone();
//                 collected_keys.push(*key);

//                 let next_branch = Progression
//                 {
//                     distance: current.distance + depth,
//                     x: key_coord.0,
//                     y: key_coord.1,
//                     // collect this key
//                     collected_keys: collected_keys,
//                 };
//                 path_queue.push_back(next_branch);
//             }
//         }
//     }

//     result.sort_by(|a, b| a.distance.cmp(&b.distance));
//     println!("results: {:?}", result);
//     for x in result
//     {
//         return Some(x.clone())
//     }

//     return None;
// }

fn find_shortest(map: &Map) -> Option<Progression>
{
    let start_coord = map.get_starting_coordinate();
    let starting = Progression
    {
        distance: 0,
        x: start_coord.0,
        y: start_coord.1,
        collected_keys: Vec::new(),
    };

    let mut key_distances = HashMap::new();

    // do BFS from each of the keys
    for k in map.key_coordinates.keys()
    {
        let coord_idx = map.key_coordinates[&k];
        let coord = map.get_index_coordinate(coord_idx);
        println!("for key {} at coord {:?}", k, coord);

        // do BFS from this point
        let mut m = HashMap::new();
        let result = bfs_from_point(coord.0, coord.1, map, &mut m);
        println!("bfs from point {:?}", m);

        key_distances.insert(k, m);
    }

    // get starting position distances
    let mut starting_distances = HashMap::new();
    bfs_from_point(starting.x, starting.y, map, &mut starting_distances);

    println!("distances from starting point {:#?}", starting_distances);
    println!("key distances {:#?}", key_distances);

    // get neighbors of the current point
    let mut current_distance = starting_distances;
    
    // use dijkstra for all accessible    

    None
}


fn is_open(val: char) -> bool
{
    match val
    {
        '@' => true,
        '#' => false,
        '.' => true,
        '\n' => false,
        // can pass through key
        'a'..='z' => true,
        'A'..='Z' => true,
        // other, false
        _ => {
            println!("encountered an unexpected val {}", val);
            false
        },
    }
}

fn is_direction_free(collected_keys: &Vec<char>, val: char) -> bool
{
    match val
    {
        '@' => true,
        '#' => false,
        '.' => true,
        '\n' => false,
        // can pass through key
        'a'..='z' => true,
        'A'..='Z' => {
            // println!("collected keys: {:?} val {}", collected_keys, val);
            // door, check that the progression has a key
            collected_keys.contains(&val.to_ascii_lowercase())
        },
        // other, false
        _ => {
            println!("encountered an unexpected val {}", val);
            false
        },
    }
}

fn main() {
    // I should just figure out how to write tests the proper way
    example_one();
    example_two();
    example_three();
    example_four();
    example_five();

    solution();
}

fn solution()
{
    let input_file = "/home/chris/Git/adventofcode/2019/18/input.txt";
    let content = fs::read_to_string(input_file)
        .expect("failed to read file");
    let map = Map::from_map(content);
    println!("parsed map: {:?}", map);
    find_shortest(&map);
    // let result = find_shortest_path(&map)
    //     .expect("failed to find path");
    // println!("result: {:?}", result);

    println!("DONE SOLUTION PART 1");
}

fn example_one()
{
//     let map = 
// "#########
// #b.A.@.a#
// #########";
let map =
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    // b a c d f e g
    // 132
    let map = Map::from_map(map.to_string());
    println!("parsed map: {:?}", map);
    find_shortest(&map);
    // result is 8 steps
    // assert!(result.distance == 8);
    println!("ONE pass");
}

fn example_two()
{
    let map =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    // result is 86 steps
    // need to hit d before e E

    let map = Map::from_map(map.to_string());
    println!("parsed map: {:?}", map);
    let result = find_shortest_path(&map)
        .expect("couldn't find path");

    // result is 8 steps
    assert!(result.distance == 86);
    println!("TWO pass");
}

fn example_three()
{
    let map =
"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    // b a c d f e g
    // 132
    let map = Map::from_map(map.to_string());
    // println!("parsed map: {:?}", map);
    let result = find_shortest_path(&map)
        .expect("couldn't find path");

    // result is 8 steps
    assert!(result.distance == 132);
    println!("THREE pass");
}

fn example_four()
{
    let map =
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    // a f b j g n h d l o e p c i k m
    // 136

    let map = Map::from_map(map.to_string());
    println!("parsed map: {:?}", map);
    let result = find_shortest_path(&map)
        .expect("couldn't find path");

    // result is 8 steps
    assert!(result.distance == 136);
    println!("FOUR pass");
}

fn example_five()
{
    let map =
"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
    // a c f i d g b e n
    // 81

    let map = Map::from_map(map.to_string());
    // println!("parsed map: {:?}", map);
    let result = find_shortest_path(&map)
        .expect("couldn't find path");

    // result is 8 steps
    assert!(result.distance == 81);
    println!("FIVE pass");
}