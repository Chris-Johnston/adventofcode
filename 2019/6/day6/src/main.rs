// Doesn't work, had issues with ref lifecycle.
// still learning

// COM - 0
// B 1
// C B 2
// D C B 3
// I D C B 4
// E D C B 4
// F E D C B 5
// J E D C B 5
// K J E D C B 6
// L K J E D C B 7
// G B 2
// H G B 3

use std::vec;
use std::string;
use std::collections::HashMap;

#[derive(Hash)]
struct Orbit
{
    depth: isize,
    name: String,
    parent: String,
}

fn main() {
    let test_set = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L");
    let mut test_map : HashMap<&String, &Orbit> = HashMap::new();
    parse_input(test_set, &mut test_map);
    let test_depth = sum_depth(test_map);
    assert!(test_depth == 42);

}

fn sum_depth(map: HashMap<&String, &Orbit>) -> isize
{
    let mut depth = 0;
    for key in map.keys()
    {
        let orbit = map.get(key)
                    .expect("Couldn't get the orbit");
        depth += orbit.depth;
    }
    return depth;
}

fn parse_input(map: Vec<&str>, orbit_map: &mut HashMap<&String, &Orbit>) // -> &HashMap<&String, &Orbit>
{
    // let mut orbit_map = HashMap::new();

    // add center
    let com = Orbit { depth: 0, name: String::from("COM"), parent: String::from("None")};
    orbit_map.insert(&com.name, &com);

    // iterate through all lines of the map
    for orbit in map
    {
        let split : Vec<&str> = orbit.split(")").collect();

        let center = String::from(split[0]);
        let orbit = String::from(split[1]);

        let parent_mass = orbit_map.get(&center)
            .expect("center of orbit not in map");

        let insert = Orbit { depth: &parent_mass.depth + 1, name: orbit.to_owned(), parent: String::from(&parent_mass.name) };
        orbit_map.insert(&insert.name, &insert);
    }

    // return &orbit_map;
}
