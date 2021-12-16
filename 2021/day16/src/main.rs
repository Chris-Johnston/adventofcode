use std::fs;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use regex::Regex;

static INPUT_FILE: &str = "input.txt";

fn main() {
    let in_text = // fs::read_to_string(INPUT_FILE).unwrap();
        include_str!("../input.txt");
    let input = parse_input(&in_text);

    let answer = part1(&input);
    println!("part 1 {}", answer);
    
    let answer = part2(&input);
    println!("part 2 {}", answer);
}

fn char_to_bytes(c: char) -> Vec<usize>
{
    match c // yolo do this the dumb way
    {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("aaa!!!"),
    }
}

fn parse_input(input: &str) -> Vec<usize> // yes I should be using u8, no I wont
{
    // hex string into a bit array
    let mut result = Vec::new();
    for c in input.trim().chars()
    {
        result.extend(char_to_bytes(c));
    }
    
    result
}

fn get_pkt_version(packet: &Vec<usize>, pkt_start_idx: usize) -> usize
{
    let mut ver = 0;
    for c_idx in pkt_start_idx..pkt_start_idx + 3
    {
        ver = ver << 1;
        ver += packet[c_idx];
    }

    ver
}

fn get_pkt_type(packet: &Vec<usize>, pkt_start_idx: usize) -> usize
{
    let mut pkt_type = 0;
    for c_idx in 3 + pkt_start_idx..3 + pkt_start_idx + 3
    {
        pkt_type = pkt_type << 1;
        pkt_type += packet[c_idx];
    }

    pkt_type
}

fn get_bin_number(packet: &Vec<usize>, pkt_start_idx: usize) -> (usize, usize)
{
    let mut idx = pkt_start_idx + 6;
    let idx_start = idx;
    let mut has_packets = true;
    let mut value = 0;
    while has_packets
    {
        idx += 1;
        has_packets = packet[idx] == 1;
        for _ in 0..4
        {
            idx += 1;
            let bit = packet[idx];
            value = value << 1;
            value += bit;
        }
    }

    (value, idx - idx_start)
}

fn get_operator_packet(packet: &Vec<usize>, pkt_start_idx: usize)
{
    // length type id is right after heade
    let length_type_id_idx = pkt_start_idx + 7;
    if (packet[length_type_id_idx] == 0)
    {
        // next 15 bites are number for the total len in bytes of subpackets contained
        let len = get_len_subpackets(packet, pkt_start_idx);
    }
    else
    {
        // next 11 bites are number of sub-packets contained
        let num = get_num_subpackets(packet, pkt_start_idx);
    }
}

fn get_len_subpackets(packet: &Vec<usize>, pkt_start_idx: usize) -> usize
{
    let mut len = 0;
    for idx in pkt_start_idx + 7..pkt_start_idx + 7 + 15
    {
        len = len << 1;
        len += packet[idx];
    }
    len
}

fn get_num_subpackets(packet: &Vec<usize>, pkt_start_idx: usize) -> usize
{
    let mut len = 0;
    for idx in pkt_start_idx + 7..pkt_start_idx + 7 + 11
    {
        len = len << 1;
        len += packet[idx];
    }
    len
}

struct Packet
{
    version: usize,
    pkt_type: usize,
    bin_literal: Option<usize>,
    operator_type: Option<usize>,
    len_subpackets: Option<usize>,
    num_subpackets: Option<usize>,

    subpackets: Vec<Packet>,
}

fn parse_packet(packet: &Vec<usize>) -> Packet
{
    let ver = get_pkt_version(packet, 0);
    let pkt_type = get_pkt_type(packet, 0);
    let mut literal = None;

    if pkt_type == 4
    {
        // literal
        literal = Some(get_bin_number(packet, 0));
    }
    else
    {
        // operator type
        
    }

    Packet {

    }
}

fn part1(input: &Vec<usize>) -> usize
{
    

    0
}

fn part2(input: &Vec<usize>) -> usize
{
    0
}

#[test]
fn example()
{
    let input =
    "8A004A801A8002F478";
    let input = parse_input(input);
    assert_eq!(part1(&input), 31);
    assert_eq!(part2(&input), 2188189693529);
}