// https://leetcode.com/problems/walking-robot-simulation/

use std::collections::HashSet;

struct Solution;

#[derive(Clone,Copy)]
enum Dir {
    North,
    West,
    South,
    East,
}

impl Dir {
    fn rotate_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West  => Self::South,
            Self::South => Self::East,
            Self::East  => Self::North,
        }
    }

    fn rotate_right(self) -> Dir {
        self.rotate_left().rotate_left().rotate_left()
    }
}

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

impl Point {
    fn forward(self, offset: i32, dir: Dir, obs: &HashSet<(i32, i32)>) -> Point {
        let o = match dir {
            Dir::North => (0, 1),
            Dir::West  => (-1, 0),
            Dir::South => (0, -1),
            Dir::East  => (1, 0),
        };

        let Point(mut x, mut y) = self;

        for _ in 1..=offset {
            if obs.contains(&(x+o.0, y+o.1)) {
                return Point(x, y)
            }
            println!("x: {}, y: {}", x, y);

            x += o.0;
            y += o.1;
        }

        Point(x, y)
    }
}

enum Cmd {
    Left,
    Right,
    Forward(i32),
}

impl Cmd {
    fn from_i32(i: i32) -> Self {
        match i {
            -2    => Self::Left,
            -1    => Self::Right,
            1..=9 => Self::Forward(i),
            _     => panic!("Invalid command"),
        }
    }
}

struct Robot {
    dir: Dir,
    pos: Point,
    max: i32,
}

impl Robot {
    fn new() -> Self {
        Robot {
            dir: Dir::North,
            pos: Point(0, 0),
            max: 0,
        }
    }

    fn execute(&mut self, cmd: Cmd, obs: &HashSet<(i32, i32)>) {
        match cmd {
            Cmd::Left       => self.dir = self.dir.rotate_left(),
            Cmd::Right      => self.dir = self.dir.rotate_right(),
            Cmd::Forward(i) => self.pos = self.pos.forward(i, self.dir, obs),
        }

        self.max = self.max.max(self.pos.0.pow(2) + self.pos.1.pow(2));
    }
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obs: HashSet<(i32, i32)> = obstacles.iter().fold(HashSet::new(), |mut acc, p| {
            acc.insert((p[0], p[1]));
            acc
        });

        let mut r = Robot::new();

        for i in commands.iter() {
            r.execute(Cmd::from_i32(*i), &obs); 
        }

        r.max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(25, Solution::robot_sim(vec![4, -1, 3], vec![]));
    }

    #[test]
    fn example2() {
        assert_eq!(
            65,
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
        );
    }
}
