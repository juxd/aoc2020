use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Instr {
    MaskSet {
        zeroes: u64,
        ones: u64,
        x_s: Vec<usize>,
    },
    MemSet {
        dst: u64,
        val: u64,
    },
}

struct Mem {
    memory_status: HashMap<u64, u64>,
    zeroes_mask: u64,
    ones_mask: u64,
    x_s: Vec<usize>,
}

impl Instr {
    fn of_instr_string(instr: &str) -> Option<Instr> {
        let mask_re = Regex::new(r"mask\s=\s([01X]+)").unwrap();
        let mem_re = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)").unwrap();
        mask_re
            .captures(instr)
            .map(|caps| {
                let ones = u64::from_str_radix(&caps[1].replace('X', "0"), 2).unwrap();
                let zeroes = 0xFFFFFFF000000000
                    | u64::from_str_radix(&caps[1].replace('X', "1"), 2).unwrap();
                let x_s = &caps[1]
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == 'X' { Some(35 - i) } else { None })
                    .collect::<Vec<_>>();
                Instr::MaskSet {
                    zeroes,
                    ones,
                    x_s: x_s.to_vec(),
                }
            })
            .or_else(move || {
                mem_re.captures(instr).map(|caps| Instr::MemSet {
                    dst: caps[1].parse().unwrap(),
                    val: caps[2].parse().unwrap(),
                })
            })
    }
}

fn sim_instrs(mem: &mut Mem, instrs: &[Instr]) -> u64 {
    for instr in instrs {
        match instr {
            Instr::MaskSet { zeroes, ones, .. } => {
                mem.zeroes_mask = *zeroes;
                mem.ones_mask = *ones;
            }
            Instr::MemSet { dst, val } => {
                let masked_val = *val & mem.zeroes_mask | mem.ones_mask;
                mem.memory_status.insert(*dst, masked_val);
            }
        }
    }
    mem.memory_status.values().sum()
}

fn sim_instrs_p2(mem: &mut Mem, instrs: &[Instr]) -> u64 {
    for instr in instrs {
        match instr {
            Instr::MaskSet { zeroes, ones, x_s } => {
                mem.zeroes_mask = *zeroes;
                mem.ones_mask = *ones;
                mem.x_s = x_s.to_vec();
            }
            Instr::MemSet { dst, val } => {
                let masked_dst = *dst | mem.ones_mask;
                for addr in modified_address(masked_dst, &mem.x_s) {
                    mem.memory_status.insert(addr, *val);
                }
            }
        }
    }
    mem.memory_status.values().sum()
}

fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .filter_map(|line| Instr::of_instr_string(line))
        .collect::<Vec<_>>()
}

pub fn solve_p1(input: &str) -> u64 {
    sim_instrs(
        &mut Mem {
            memory_status: HashMap::new(),
            zeroes_mask: !0,
            ones_mask: 0,
            x_s: [].to_vec(),
        },
        &parse(input),
    )
}

fn modified_address_helper(addr: u64, x_s: &[usize], idx: usize, vec_dst: &mut Vec<u64>) {
    if idx >= x_s.len() {
        vec_dst.push(addr);
    } else {
        let mask = (1_i64 << x_s[idx]) as u64;
        modified_address_helper(addr | mask, x_s, idx + 1, vec_dst);
        modified_address_helper(addr & !mask, x_s, idx + 1, vec_dst);
    }
}

fn modified_address(addr: u64, x_s: &[usize]) -> Vec<u64> {
    let res = &mut Vec::new();
    modified_address_helper(addr, x_s, 0, res);
    res.to_vec()
}

pub fn solve_p2(input: &str) -> u64 {
    sim_instrs_p2(
        &mut Mem {
            memory_status: HashMap::new(),
            zeroes_mask: !0,
            ones_mask: 0,
            x_s: [].to_vec(),
        },
        &parse(input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static SAMPLE_INPUT2: &str = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_parser() {
        assert_eq!(
            vec![
                Instr::MaskSet {
                    zeroes: !2,
                    ones: 64,
                    x_s: vec![
                        35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17,
                        16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 5, 4, 3, 2, 0
                    ],
                },
                Instr::MemSet { dst: 8, val: 11 },
                Instr::MemSet { dst: 7, val: 101 },
                Instr::MemSet { dst: 8, val: 0 },
            ],
            parse(SAMPLE_INPUT)
        );
        assert_eq!(
            vec![
                Instr::MaskSet {
                    zeroes: 0xFFFFFFF000000000 | 0b110011,
                    ones: 18,
                    x_s: vec![5, 0]
                },
                Instr::MemSet { dst: 42, val: 100 },
                Instr::MaskSet {
                    zeroes: 0xFFFFFFF000000000 | 0b1011,
                    ones: 0,
                    x_s: vec![3, 1, 0]
                },
                Instr::MemSet { dst: 26, val: 1 }
            ],
            parse(SAMPLE_INPUT2)
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(165, solve_p1(SAMPLE_INPUT));
    }

    #[test]
    fn address_modifier() {
        assert_eq!(
            vec![27, 26, 25, 24, 19, 18, 17, 16],
            modified_address(26, &[3, 1, 0])
        );
        assert_eq!(vec![59, 58, 27, 26], modified_address(58, &[5, 0]));
    }

    #[test]
    fn test_p2() {
        assert_eq!(208, solve_p2(SAMPLE_INPUT2));
    }
}
