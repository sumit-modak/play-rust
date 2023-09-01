use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use core::cmp;
// use std::cmp::{min, max};

// https://www.hackerrank.com/challenges/queens-attack-2/problem?isFullScreen=true

fn queensAttack(n: i32, k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    let mut l_row = 1;
    let mut r_row = n;
    let mut d_col = 1;
    let mut u_col = n;
    let mut bl = (r_q-(cmp::min(r_q, c_q)-1), c_q-(cmp::min(r_q, c_q)-1));
    let mut tr = (r_q+(n-cmp::max(r_q, c_q)), c_q+(n-cmp::max(r_q, c_q)));
    let mut tl = (r_q+cmp::min(n-r_q, c_q-1), c_q-cmp::min(n-r_q, c_q-1));
    let mut br = (r_q-cmp::min(r_q-1, n-c_q), c_q+cmp::min(r_q-1, n-c_q));
    println!("{} {} {} {} {:?} {:?} {:?} {:?}", l_row, r_row, d_col, u_col, bl, tr, tl, br);
    
    for (i, v) in obstacles.iter().enumerate() {
        println!("{} {:?}", i, v);
        if obstacles[i][0] == r_q {
            if obstacles[i][1] < c_q && obstacles[i][1] > l_row {
                println!("hello1");
                l_row = obstacles[i][1] - 1;
                println!("{}", l_row);
            } else if obstacles[i][1] > c_q && obstacles[i][1] < r_row {
                println!("hello2");
                r_row = obstacles[i][1] - 1;
            }
        } else if obstacles[i][1] == c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > d_col {
                println!("hello3");
                d_col = obstacles[i][0] - 1;
                println!("{}", d_col);
            } else if obstacles[i][0] > r_q && obstacles[i][0] < u_col {
                println!("hello4");
                u_col = obstacles[i][0] - 1;
            }
        } else if obstacles[i][0] - r_q == obstacles[i][1] - c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > bl.0 {
                println!("hello5");
                bl.0 = obstacles[i][0] - 1;
            } else if obstacles[i][0] > r_q && obstacles[i][0] < tr.0 {
                println!("hello6");
                tr.0 = obstacles[i][0] - 1;
            }
        } else if obstacles[i][0] + obstacles[i][1] == r_q + c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > tl.0 {
                println!("hello7");
                tl.0 = obstacles[i][0] - 1;
            } else if obstacles[i][0] > r_q && obstacles[i][0] < br.0 {
                println!("hello8");
                br.0 = obstacles[i][0] - 1;
            }
        }
    }
    
    println!("{} {} {} {} {:?} {:?} {:?} {:?}", l_row, r_row, d_col, u_col, bl, tr, tl, br);
    (r_row-l_row) + (u_col-d_col) + num::abs(tr.0-bl.0) + num::abs(br.0-tl.0)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let r_q = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let c_q = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut obstacles: Vec<Vec<i32>> = Vec::with_capacity(k as usize);

    for i in 0..k as usize {
        obstacles.push(Vec::with_capacity(2_usize));

        obstacles[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = queensAttack(n, k, r_q, c_q, &obstacles);

    writeln!(&mut fptr, "{}", result).ok();
}
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use core::cmp;
// use std::cmp::{min, max};

// https://www.hackerrank.com/challenges/queens-attack-2/problem?isFullScreen=true

fn queensAttack(n: i32, k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    let mut l_row = 1;
    let mut r_row = n;
    let mut d_col = 1;
    let mut u_col = n;
    let mut bl = (r_q-(cmp::min(r_q, c_q)-1), c_q-(cmp::min(r_q, c_q)-1));
    let mut tr = (r_q+(n-cmp::max(r_q, c_q)), c_q+(n-cmp::max(r_q, c_q)));
    let mut tl = (r_q+cmp::min(n-r_q, c_q-1), c_q-cmp::min(n-r_q, c_q-1));
    let mut br = (r_q-cmp::min(r_q-1, n-c_q), c_q+cmp::min(r_q-1, n-c_q));
    println!("{} {} {} {} {:?} {:?} {:?} {:?}", l_row, r_row, d_col, u_col, bl, tr, tl, br);
    
    for (i, v) in obstacles.iter().enumerate() {
        println!("{} {:?}", i, v);
        if obstacles[i][0] == r_q {
            if obstacles[i][1] < c_q && obstacles[i][1] > l_row {
                l_row = obstacles[i][1] - 1;
            } else if obstacles[i][1] > c_q && obstacles[i][1] < r_row {
                r_row = obstacles[i][1] - 1;
            }
        } else if obstacles[i][1] == c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > d_col {
                d_col = obstacles[i][0] - 1;
            } else if obstacles[i][0] > r_q && obstacles[i][0] < u_col {
                u_col = obstacles[i][0] - 1;
            }
        } else if obstacles[i][0] - r_q == obstacles[i][1] - c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > bl.0 {
                bl.0 = obstacles[i][0] - 1;
            } else if obstacles[i][0] > r_q && obstacles[i][0] < tr.0 {
                tr.0 = obstacles[i][0] - 1;
            }
        } else if obstacles[i][0] + obstacles[i][1] == r_q + c_q {
            if obstacles[i][0] < r_q && obstacles[i][0] > tl.0 {
                tl.0 = obstacles[i][0] - 1;
            } else if obstacles[i][0] > r_q && obstacles[i][0] < br.0 {
                br.0 = obstacles[i][0] - 1;
            }
        }
    }
    
    println!("{} {} {} {} {:?} {:?} {:?} {:?}", l_row, r_row, d_col, u_col, bl, tr, tl, br);
    (r_row-l_row) + (u_col-d_col) + (tr.0-bl.0) + (br.0-tl.0)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let r_q = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let c_q = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut obstacles: Vec<Vec<i32>> = Vec::with_capacity(k as usize);

    for i in 0..k as usize {
        obstacles.push(Vec::with_capacity(2_usize));

        obstacles[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = queensAttack(n, k, r_q, c_q, &obstacles);

    writeln!(&mut fptr, "{}", result).ok();
}

