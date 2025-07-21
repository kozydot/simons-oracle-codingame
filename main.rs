// https://www.codingame.com/ide/puzzle/simons-oracle

use std::io;
use std::collections::{HashMap, HashSet};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/// converts binary string to u64
fn bin_to_int(bin_str: &str) -> u64 {
    u64::from_str_radix(bin_str, 2).unwrap()
}

/// finds the secret string and generates query results
fn solve(l: i32, n: i32, queries_str: &Vec<String>) -> (u64, Vec<char>) {
    // parse queries into integers for efficient xor
    let queries: Vec<u64> = queries_str.iter().map(|s| bin_to_int(s)).collect();
    let mut s_determined: Option<u64> = None;

    // holds unique queries that create constraints
    let mut unique_queries: Vec<u64> = Vec::new();
    let mut unique_queries_set: HashSet<u64> = HashSet::new();
    // stores forbidden s values, built incrementally
    let mut p_forbidden: HashSet<u64> = HashSet::new();

    // part 1: deduce secret string 's' by simulating the oracle
    for q_k in &queries {
        // skip duplicate queries; they add no new information
        if unique_queries_set.contains(q_k) {
            continue;
        }

        // new constraints if q_k were unique
        let mut b = HashSet::new();
        for &uq in &unique_queries {
            b.insert(q_k ^ uq);
        }
        
        // valid s candidates from new constraints
        let c: HashSet<u64> = b.difference(&p_forbidden).cloned().collect();

        // count of all possible non-zero s values
        let total_non_zero_s = (1u64 << l) - 1;
        let num_possible_s_before = total_non_zero_s.saturating_sub(p_forbidden.len() as u64);

        // check for forcing condition
        if num_possible_s_before > 0 && num_possible_s_before == c.len() as u64 {
            // forced: pair with earliest unique query
            for &uq_j in &unique_queries {
                let s_candidate = q_k ^ uq_j;
                if c.contains(&s_candidate) {
                    s_determined = Some(s_candidate);
                    break;
                }
            }
            // s is determined, exit simulation
            if s_determined.is_some() {
                break;
            }
        }

        // not forced: add new constraints
        for &uq in &unique_queries {
            p_forbidden.insert(q_k ^ uq);
        }
        unique_queries.push(*q_k);
        unique_queries_set.insert(*q_k);
    }

    let final_s = match s_determined {
        Some(s) => s,
        None => {
            // unforced: find largest possible s
            let max_s_val = (1u64 << l) - 1;
            let mut s_found = 0;
            for s_cand in (1..=max_s_val).rev() {
                if !p_forbidden.contains(&s_cand) {
                    s_found = s_cand;
                    break;
                }
            }
            s_found
        }
    };

    // part 2: generate final query results
    let mut results = Vec::new();
    let mut rep_to_char: HashMap<u64, char> = HashMap::new();
    let mut next_char_code = 'A' as u8;

    for &q in &queries {
        let partner = q ^ final_s;
        // use the smaller of {q, q^s} as the pair's id
        let rep = q.min(partner);
        
        // assign a character to the pair's representative
        let current_char = *rep_to_char.entry(rep).or_insert_with(|| {
            let ch = next_char_code as char;
            next_char_code += 1;
            ch
        });
        results.push(current_char);
    }

    (final_s, results)
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split_whitespace().collect::<Vec<_>>();
    let l = parse_input!(inputs[0], i32);
    let n = parse_input!(inputs[1], i32);
    
    let mut queries_str = Vec::new();
    for _ in 0..n {
        let mut query_line = String::new();
        io::stdin().read_line(&mut query_line).unwrap();
        queries_str.push(query_line.trim().to_string());
    }

    // handle empty query set
    if n == 0 {
        let s_val = (1u64 << l) - 1;
        println!("{:0width$b}", s_val, width = l as usize);
        return;
    }

    let (s_val, results) = solve(l, n, &queries_str);

    // print s, padded to length l
    println!("{:0width$b}", s_val, width = l as usize);

    for result in results {
        println!("{}", result);
    }
}
