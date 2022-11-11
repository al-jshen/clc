use fast_float::parse;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("No input");
        exit(1);
    }

    let stdin = io::stdin();
    let nums = stdin
        .lock()
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let num_res = parse::<f32, &[u8]>(l.as_bytes());
            match num_res {
                Ok(v) => v,
                Err(v) => panic!("Failed on \"{}\". Parsing error: \"{}\"", l, v),
            }
        })
        .collect::<Vec<_>>();

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "sum" => {
                let sum = nums.iter().fold(0.0, |acc, x| acc + x);
                println!("{sum}");
            }
            "mean" => {
                let sum = nums.iter().fold(0.0, |acc, x| acc + x);
                let mean = sum / nums.len() as f32;
                println!("{mean}");
            }
            "median" => {
                let mut nums = nums;
                nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let median = nums[nums.len() / 2];
                println!("{median}");
            }
            "count" => {
                println!("{}", nums.len());
            }
            "min" => {
                let min = nums.iter().fold(f32::INFINITY, |acc, x| acc.min(*x));
                println!("{min}");
            }
            "max" => {
                let max = nums.iter().fold(f32::NEG_INFINITY, |acc, x| acc.max(*x));
                println!("{max}");
            }
            "range" => {
                let min = nums.iter().fold(f32::INFINITY, |acc, x| acc.min(*x));
                let max = nums.iter().fold(f32::NEG_INFINITY, |acc, x| acc.max(*x));
                println!("{}", max - min);
            }
            "variance" => {
                let sum = nums.iter().fold(0.0, |acc, x| acc + x);
                let mean = sum / nums.len() as f32;
                let variance =
                    nums.iter().fold(0.0, |acc, x| acc + (x - mean).powi(2)) / nums.len() as f32;
                println!("{variance}");
            }
            "stddev" => {
                let sum = nums.iter().fold(0.0, |acc, x| acc + x);
                let mean = sum / nums.len() as f32;
                let variance =
                    nums.iter().fold(0.0, |acc, x| acc + (x - mean).powi(2)) / nums.len() as f32;
                println!("{}", variance.sqrt());
            }
            "sumsq" => {
                let sumsq = nums.iter().fold(0.0, |acc, x| acc + x.powi(2));
                println!("{sumsq}");
            }
            "norm" => {
                let sumsq = nums.iter().fold(0.0, |acc, x| acc + x.powi(2));
                println!("{}", sumsq.sqrt());
            }
            "rms" => {
                let sumsq = nums.iter().fold(0.0, |acc, x| acc + x.powi(2));
                let rms = sumsq.sqrt() / nums.len() as f32;
                println!("{rms}");
            }
            "product" => {
                let product = nums.iter().fold(1.0, |acc, x| acc * x);
                println!("{product}");
            }
            "geomean" => {
                let product = nums.iter().fold(1.0, |acc, x| acc * x);
                let geomean = product.powf(1.0 / nums.len() as f32);
                println!("{geomean}");
            }
            "harmean" => {
                let sum = nums.iter().fold(0.0, |acc, x| acc + 1.0 / x);
                let harmean = nums.len() as f32 / sum;
                println!("{harmean}");
            }
            "mode" => {
                let mut nums = nums;
                nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut mode = nums[0];
                let mut mode_count = 1;
                let mut count = 1;
                for i in 1..nums.len() {
                    if nums[i] == nums[i - 1] {
                        count += 1;
                    } else {
                        if count > mode_count {
                            mode = nums[i - 1];
                            mode_count = count;
                        }
                        count = 1;
                    }
                }
                println!("{mode}");
            }
            "sort" => {
                let mut nums = nums;
                nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                println!("{:?}", nums);
            }
            "sortrev" => {
                let mut nums = nums;
                nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                nums.reverse();
                println!("{:?}", nums);
            }
            "sortabs" => {
                let mut nums = nums;
                nums.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
                println!("{:?}", nums);
            }
            _ => {
                println!("Unknown command, try one of: sum, mean, median, count, min, max, range, variance, stddev, sumsq, norm, rms, product, geomean, harmean, mode, sort, sortrev, sortabs");
                exit(1);
            }
        }
    }
}
