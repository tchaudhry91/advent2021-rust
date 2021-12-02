use eyre::Result;
use std::fs;
extern crate eyre;

#[derive(Debug)]
struct Submarine {
    ypos: i64,
    xpos: i64,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            ypos: 0,
            xpos: 0,
            aim: 0,
        }
    }

    fn move_x(&mut self, delta: i64) {
        self.xpos += delta;
    }

    fn move_up(&mut self, delta: i64) {
        self.ypos -= delta;
    }
    fn move_down(&mut self, delta: i64) {
        self.ypos += delta;
    }

    fn follow_instruction(&mut self, command: String) {
        let command_split: Vec<&str> = command.split(' ').collect();
        let command = &command_split[0];
        let delta = &command_split[1];

        if *command == "forward" {
            self.move_x(delta.parse::<i64>().unwrap());
        }
        if *command == "up" {
            self.move_up(delta.parse::<i64>().unwrap());
        }
        if *command == "down" {
            self.move_down(delta.parse::<i64>().unwrap());
        }
    }
}

#[derive(Debug)]
struct SubmarineMod {
    ypos: i64,
    xpos: i64,
    aim: i64,
}

impl SubmarineMod {
    fn new() -> SubmarineMod {
        SubmarineMod {
            ypos: 0,
            xpos: 0,
            aim: 0,
        }
    }

    fn forward(&mut self, delta: i64) {
        self.xpos += delta;
        self.ypos += self.aim * delta;
    }

    fn move_up(&mut self, delta: i64) {
        self.aim -= delta;
    }
    fn move_down(&mut self, delta: i64) {
        self.aim += delta;
    }

    fn follow_instruction(&mut self, command: String) {
        let command_split: Vec<&str> = command.split(' ').collect();
        let command = &command_split[0];
        let delta = &command_split[1];

        if *command == "forward" {
            self.forward(delta.parse::<i64>().unwrap());
        }
        if *command == "up" {
            self.move_up(delta.parse::<i64>().unwrap());
        }
        if *command == "down" {
            self.move_down(delta.parse::<i64>().unwrap());
        }
    }
}

fn read_input(fname: &str) -> Result<Vec<String>> {
    let contents = fs::read_to_string(fname)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

fn main() {
    let input = read_input("input.txt").unwrap();

    let mut sub = Submarine::new();
    let mut sub2 = SubmarineMod::new();

    for line in input {
        sub.follow_instruction(line.clone());
        sub2.follow_instruction(line);
    }

    println!("Submarine Normal");
    println!("X-Pos:{}, Y-Pos:{}", sub.xpos.abs(), sub.ypos.abs());
    println!("Multiplication:{}", sub.xpos * sub.ypos);

    println!("Submarine Modded");
    println!("X-Pos:{}, Y-Pos:{}", sub2.xpos.abs(), sub2.ypos.abs());
    println!("Multiplication:{}", sub2.xpos * sub2.ypos);
}
