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
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn ReverseIP(ip: &str) -> Vec<String> {
    let url = format!("https://rapiddns.io/sameip/{}?full=1", ip);
    let response = get(url).unwrap().text().unwrap();
    let docs = Html::parse_document(&response);
    let all_td = Selector::parse("table > tbody > tr > td:nth-child(2)").unwrap();
    let mut head:Vec<String> = Vec::new();

    for x in docs.select(&all_td) {
        for j in x.text() {
            head.push(j.to_string());
        }
    }

    if head.len() != 0 {
        println!("\x1b[93m-->\x1b[0m \x1b[94m{}\x1b[0m \x1b[97mGrabbed \x1b[0m\x1b[92m{} \x1b \x1b[97mDomain\x1b", ip, head.len());
    } else {
        println!("\x1b[93m-->\x1b[0m \x1b[94m{}\x1b[0m \x1b[97mBad IP!\x1b[0m", ip);
    }

    return head;
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
    print!("Enter number of threads you want to use: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut thread_count).unwrap();
    let num_of_thread = thread_count.trim().parse::<usize>().unwrap();

    
    
    let mut files = String::new();
    print!("\x1b[93m---> IP LIST : \x1b[0m");
    io::stdout().flush().unwrap();
    let b1 = std::io::stdin().read_line(&mut files).unwrap();   
    let lines = read_lines(files.trim()).unwrap();
    let mut threads = Vec::with_capacity(num_of_thread as usize);
    
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("res.txt")
        .unwrap();
    let sharefiles = Arc::new(Mutex::new(file));

    let sharelines = Arc::new(Mutex::new(lines));

    for i in 0..num_of_thread{
        let share = Arc::clone(&sharelines);
        let file = sharefiles.clone();
        threads.push(
            thread::spawn(
                move || {
                    loop {
                        let line;
                        {
                            line = share.lock().unwrap().next();
                        }
                        if line.is_none() {
                            break;
                        } else {
                            if let Ok(l) =  line.as_ref().unwrap() {
                                let domains = ReverseIP(l);
                                if domains.len() == 0{
                                } else {
                                    let mut f = file.lock().unwrap();
                                    
                                    for d in domains {
                                        if let Err(_) = writeln!(
                                            f, "{}", d
                                        ){}
                                    }
                                }
                            }
                        }
                    }
                }
            )
        )
    }

    for t in threads {
        t.join().unwrap();
    }
}