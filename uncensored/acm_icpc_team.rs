use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

// https://www.hackerrank.com/challenges/acm-icpc-team/problem?isFullScreen=true

fn acmTeam(topic: &[String]) -> Vec<i32> {
    let n = topic.len();
    let m = topic[0].len();
    let mut sub_count = 0;
    let mut team_count = 0;
    
    for i in 0..n-1 {
        for j in i+1..n {
            if i == j {
                let mut count = 0;
                for k in 0..m {
                    if topic[j].chars().nth(k).unwrap() == '1' {
                        count += 1;
                    }
                }
                if count > sub_count || count == sub_count && team_count == 2 {
                    sub_count = count;
                    team_count = 1;
                }
            }
            let mut count = 0;
            for k in 0..m {
                if topic[i].chars().nth(k).unwrap() == '1' || topic[j].chars().nth(k).unwrap() == '1' {
                    count += 1;
                }
            }
            if count > sub_count {
                sub_count = count;
                team_count = 2;
            }
        }
    }
    
    vec![sub_count, team_count]
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

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut topic: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let topic_item = stdin_iterator.next().unwrap().unwrap();
        topic.push(topic_item);
    }

    let result = acmTeam(&topic);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

