#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cfonts;

use reqwest::blocking::get;
use scraper::{Html, Selector};
use cfonts::{ say, Options, Colors, Align };
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::thread;

fn ReverseIP(ip: &str) {
    let url = format!("https://rapiddns.io/sameip/{}?full=1", ip);
    let response = get(url).unwrap().text().unwrap();
    let docs = Html::parse_document(&response);
    let all_td = Selector::parse("table > tbody > tr > td:nth-child(2)").unwrap();
    let mut head:Vec<String> = Vec::new();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("res.txt")
        .unwrap();

    for x in docs.select(&all_td) {
        for j in x.text() {
            head.push(j.to_string());
        }
    }

    if head.len() != 0 {
        println!("\x1b[93m-->\x1b[0m \x1b[94m{}\x1b[0m \x1b[97mGrabbed \x1b[0m\x1b[92m{} \x1b \x1b[97mDomain\x1b", ip, head.len());
        for m in head {
            if let Err(e) = writeln!(file, "{}", m) {
            }
        }
    } else {
        println!("\x1b[93m-->\x1b[0m \x1b[94m{}\x1b[0m \x1b[97mBad IP!\x1b[0m", ip);
    }
}
fn Banner() {
    print!("\x1B[2J\x1B[1;1H");
    say(Options {
        text: String::from("Reverse IP"),
        colors: vec![Colors::Red],
        align: Align::Left,
        ..Options::default()
    });
    println!("\x1b[93m Created By {}\x1b[0m", "\x1b[97mX - MrG3P5\x1b[0m");
    println!("\x1b[93m Made With Rust {}\x1b[0m", "\x1b[97mv1.0\x1b[0m");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    Banner();
    println!("");

    let mut thread_count = String::new();
    let mut no_ips = String::new();
    print!("Enter number of threads you want to use: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut thread_count).unwrap();
    let num_of_thread = thread_count.trim().parse::<u8>().unwrap();
    print!("Enter number of ips: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut no_ips).unwrap();
    let ips: usize = no_ips.trim().parse().unwrap();

    
    let mut files = String::new();
    print!("\x1b[93m---> IP LIST : \x1b[0m");
    io::stdout().flush().unwrap();
    let b1 = std::io::stdin().read_line(&mut files).unwrap();   
    let mut threads = Vec::with_capacity(num_of_thread as usize); 
    if let Ok(lines) = read_lines(files.trim()) {
        let num_of_lines_for_each_thread = (ips/num_of_thread as usize) + 1;
        let mut lines_done = 0;
        let mut lines_for_each_thread: Vec<String> = Vec::new();
        for line in lines {
            if lines_done == num_of_lines_for_each_thread {
                let lines_for_thread = lines_for_each_thread.drain(..).collect::<Vec<_>>();
                threads.push(thread::spawn(
                    move || {
                        for l in lines_for_thread {
                            ReverseIP(&l)
                        }
                    }
                ));
                lines_done = 0;
            }
            if let Ok(s) = line {
                lines_for_each_thread.push(s.clone());
                lines_done += 1;
            }
        }
    }
    for t in threads {
        t.join().unwrap();
    }
}