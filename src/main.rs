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

    if args.len() == 3 {
        let v: f32 = parse(args[2].as_bytes())
            .expect(&format!("Failed to parse second argument: \"{}\"", args[2]));
        match args[1].as_str() {
            "p" => {
                nums.iter().for_each(|n| println!("{}", n + v));
            }
            "s" => {
                nums.iter().for_each(|n| println!("{}", n - v));
            }
            "m" => {
                nums.iter().for_each(|n| println!("{}", n * v));
            }
            "d" => {
                nums.iter().for_each(|n| println!("{}", n / v));
            }
            "pow" => {
                nums.iter().for_each(|n| println!("{}", n.powf(v)));
            }
            _ => {
                println!("Unknown operation: {}", args[1]);
                exit(1);
            }
        }
    }

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
            "exp" => {
                nums.iter().for_each(|n| println!("{}", n.exp()));
            }
            "ln" => {
                nums.iter().for_each(|n| println!("{}", n.ln()));
            }
            "log10" => {
                nums.iter().for_each(|n| println!("{}", n.log10()));
            }
            "log2" => {
                nums.iter().for_each(|n| println!("{}", n.log2()));
            }
            "sqrt" => {
                nums.iter().for_each(|n| println!("{}", n.sqrt()));
            }
            "cbrt" => {
                nums.iter().for_each(|n| println!("{}", n.cbrt()));
            }
            "sin" => {
                nums.iter().for_each(|n| println!("{}", n.sin()));
            }
            "cos" => {
                nums.iter().for_each(|n| println!("{}", n.cos()));
            }
            "tan" => {
                nums.iter().for_each(|n| println!("{}", n.tan()));
            }
            "asin" => {
                nums.iter().for_each(|n| println!("{}", n.asin()));
            }
            "acos" => {
                nums.iter().for_each(|n| println!("{}", n.acos()));
            }
            "atan" => {
                nums.iter().for_each(|n| println!("{}", n.atan()));
            }
            "sinh" => {
                nums.iter().for_each(|n| println!("{}", n.sinh()));
            }
            "cosh" => {
                nums.iter().for_each(|n| println!("{}", n.cosh()));
            }
            "tanh" => {
                nums.iter().for_each(|n| println!("{}", n.tanh()));
            }
            "asinh" => {
                nums.iter().for_each(|n| println!("{}", n.asinh()));
            }
            "acosh" => {
                nums.iter().for_each(|n| println!("{}", n.acosh()));
            }
            "atanh" => {
                nums.iter().for_each(|n| println!("{}", n.atanh()));
            }
            "ceil" => {
                nums.iter().for_each(|n| println!("{}", n.ceil()));
            }
            "floor" => {
                nums.iter().for_each(|n| println!("{}", n.floor()));
            }
            "round" => {
                nums.iter().for_each(|n| println!("{}", n.round()));
            }
            _ => {
                println!("Unknown operation. Command must be one of the following: sum, mean, median, min, max, range, variance, stddev, sumsq, norm, rms, product, geomean, harmean, mode, sort, sortrev, sortabs, exp, ln, log10, log2, sqrt, cbrt, sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, ceil, floor, round, p<v>, s<v>, m<v>, d<v>, pow<v>");
                exit(1);
            }
        }
    }
}
