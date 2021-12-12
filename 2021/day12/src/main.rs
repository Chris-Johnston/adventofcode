// All of them?!

use std::fs;
use std::collections::{HashSet, HashMap};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);
    assert!(answer > 3575); // too low
    assert!(answer == 3576); // paths dont HAVE to include small cave

    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn parse_input(input: &str) -> Vec<(&str, &str)>
{
    input
    .trim()
    .lines()
    .map(str::trim)
    .map(|x| {
        let mut s = x.split("-");
        (s.next().unwrap(), s.next().unwrap())
    })
    .collect()
    
}

fn is_lowercase(s: &str) -> bool
{
    for c in s.chars()
    {
        if !c.is_lowercase()
        {
            return false;
        }
    }
    return true;
}

fn is_small_cave(s: &str) -> bool
{
    !is_start_end(s) && is_lowercase(s)
}

fn is_start_end(s: &str) -> bool
{
    s == "start" && s == "end"
}

fn part1(input: &Vec<(&str, &str)>) -> usize
{
    // set up adjacency
    // adjacencies are doubly-linked, there are no one-way links
    let mut adjacency = HashMap::new();

    for (src, dst) in input
    {
        // connect src to dst
        let connects_to = adjacency.entry(*src).or_insert(HashSet::new());
        connects_to.insert(*dst);

        // and connect dst to src
        let connects_to = adjacency.entry(*dst).or_insert(HashSet::new());
        connects_to.insert(*src);
    }

    println!("{:?}", adjacency);

    // trying this recursively at first

    let mut paths : Vec<Vec<&str>> = Vec::new();

    fn walk<'a>(adjacency_map: &HashMap<&str, HashSet<&'a str>>, path: &Vec<&'a str>, paths: &mut Vec<Vec<&'a str>>)
    {
        // println!("WALK {:?}", path);

        // get the head
        let head = path[path.len() - 1];

        if head == "end"
        {
            // paths.push(path.iter().map(|x| x.clone()).collect());
            paths.push(path.clone());
            return;
        }

        // if head has no adjacent nodes, go back
        if !adjacency_map.contains_key(head)
        {
            let go_back = path[path.len() - 2];

            // if go_back is a small cave, this is a dead-end
            if is_lowercase(go_back)
            {
                println!("hit dead end {:?}", path);
                return;
            }

            let mut new_path = path.clone();
            new_path.push(go_back);

            walk(adjacency_map, &new_path, paths);
        }

        if adjacency_map.contains_key(head)
        {
            let adjacent_nodes = &adjacency_map[head];

            for node in adjacent_nodes
            {
                if is_lowercase(node) && path.contains(node)
                {
                    // println!("path {:?} already visited small cave {}", path, node);
                    // skip
                    continue;
                }

                // need to prevent visiting small caves already visited
                let mut new_path = path.clone();
                new_path.push(node);

                walk(adjacency_map, &new_path, paths);
            }
        }
    }
    let mut path = vec!["start"];
    
    walk(&adjacency, &path, &mut paths);

    println!("paths: {:?}", paths);

    // let mut small_caves = Vec::new();
    // for key in adjacency.keys()
    // {
    //     if *key != "start" && *key != "end" && is_lowercase(key)
    //     {
    //         small_caves.push(*key);
    //     }
    // }

    // paths
    // .iter()
    // .filter(|p|
    // {
    //     for point in p.iter()
    //     {
    //         if *point != "start" && *point != "end" && is_lowercase(point)
    //         {
    //             return true;
    //         }
    //     }

    //     return false;
    // })
    // .count()

    paths.len()
}

fn part2(input: &Vec<(&str, &str)>) -> usize
{
    // set up adjacency
    // adjacencies are doubly-linked, there are no one-way links
    let mut adjacency = HashMap::new();

    for (src, dst) in input
    {
        // connect src to dst
        let connects_to = adjacency.entry(*src).or_insert(HashSet::new());
        connects_to.insert(*dst);

        // and connect dst to src
        let connects_to = adjacency.entry(*dst).or_insert(HashSet::new());
        connects_to.insert(*src);
    }

    println!("{:?}", adjacency);

    // trying this recursively at first

    // let mut paths : Vec<Vec<&str>> = Vec::new();
    let mut paths : HashSet<Vec<&str>> = HashSet::new();

    fn walk<'a>(adjacency_map: &HashMap<&str, 
        HashSet<&'a str>>, path: &Vec<&'a str>, 
        paths: &mut HashSet<Vec<&'a str>>)
    {
        println!("WALK {:?}", path);

        // get the head
        let head = path[path.len() - 1];

        if head == "end"
        {
            // paths.push(path.iter().map(|x| x.clone()).collect());
            paths.insert(path.clone());
            return;
        }

        // check if any have been visited twice already
        let mut visited_twice = None;
        for &p in path
        {
            // get count of each occurance in the path
            let count = path
                .iter()
                .filter(|x| is_small_cave(x))
                .filter(|x| ***x == *p)
                .count();
            if count > 1
            {
                visited_twice = Some(p);
                println!("using node {:?} visited twice", visited_twice);
                break;
            }
        }

        // if head has no adjacent nodes, go back
        if !adjacency_map.contains_key(head)
        {
            let go_back = path[path.len() - 2];

            // if go_back is a small cave, this is a dead-end
            if is_lowercase(go_back)
            {
                println!("hit dead end {:?}", path);
                return;
            }

            let mut new_path = path.clone();
            new_path.push(go_back);

            walk(adjacency_map, &new_path, paths);
        }

        if adjacency_map.contains_key(head)
        {
            let adjacent_nodes = &adjacency_map[head];

            for node in adjacent_nodes
            {
                if visited_twice.is_none()
                {
                    // allow visiting twice
                    if is_small_cave(node) && path.iter().filter(|x| *x == node).count() >= 2
                    {
                        println!("path {:?} already visited small cave {} twice", path, node);
                        // skip
                        continue;
                    }
                }
                else
                {
                    if is_lowercase(node) && path.contains(node)
                    {
                        println!("path {:?} already visited small cave {}", path, node);
                        // skip
                        continue;
                    }
                }

                
                if *node == "start"
                {
                    continue;
                }

                // need to prevent visiting small caves already visited
                let mut new_path = path.clone();
                new_path.push(node);

                walk(adjacency_map, &new_path, paths);
            }
        }
    }
    let mut path = vec!["start"];
    
    walk(&adjacency, &path, &mut paths);

    let mut paths_format = HashSet::new();
    for p in &paths
    {
        paths_format.insert(p.join(","));
    }

    let mut paths_format = paths_format.iter().collect::<Vec<_>>();
    paths_format.sort();

    // println!("paths: {:?}", paths);
    println!("Paths:");
    for p in &paths_format
    {
        println!("{:?}", p);
    }

    paths_format.len()
}

#[test]
fn example()
{
    // cannot test with this because d is unreachable
    // edit nvm
    let input =
    "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    let input = parse_input(input);
    assert_eq!(part1(&input), 10);

    let input =
    "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";
    let input = parse_input(input);
    assert_eq!(part1(&input), 19);

    let input2 = 
    "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    let input2 = parse_input(input2);
    assert_eq!(part1(&input2), 226);

    let input =
    "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    let input = parse_input(input);
    assert_eq!(part2(&input), 36);

    assert_eq!(part2(&input2), 3509);
}