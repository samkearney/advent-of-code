use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}

#[derive(Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn new(p1: Point, p2: Point) -> Self {
        Segment { p1: p1, p2: p2 }
    }

    fn contains_x(&self, x: i32) -> bool {
        if self.p1.x < self.p2.x {
            return self.p1.x <= x && self.p2.x >= x;
        } else {
            return self.p1.x >= x && self.p2.x <= x;
        }
    }

    fn contains_y(&self, y: i32) -> bool {
        if self.p1.y < self.p2.y {
            return self.p1.y <= y && self.p2.y >= y;
        } else {
            return self.p1.y >= y && self.p2.y <= y;
        }
    }
}

fn get_segments(line: &str) -> (Vec<Segment>, Vec<Segment>) {
    let mut current_location = Point::new(0, 0);
    let mut horizontal_segments = Vec::new();
    let mut vertical_segments = Vec::new();

    for move_instruction in line.split(',') {
        let direction = move_instruction.chars().next().unwrap();
        let distance = &move_instruction[1..].parse::<i32>().unwrap();
        match direction { 
            'L' => {
                horizontal_segments.push(
                        Segment::new(current_location.clone(),
                                     Point::new(current_location.x - distance, current_location.y)));
                current_location.x -= distance;
             },
            'R' => {
                horizontal_segments.push(
                        Segment::new(current_location.clone(),
                                     Point::new(current_location.x + distance, current_location.y)));
                current_location.x += distance;
             },
            'U' => {
                vertical_segments.push(
                        Segment::new(current_location.clone(),
                                     Point::new(current_location.x, current_location.y + distance)));
                current_location.y += distance;
            },
            'D' => {
                vertical_segments.push(
                        Segment::new(current_location.clone(),
                                     Point::new(current_location.x, current_location.y - distance)));
                current_location.y -= distance;
            },
             _ => panic!("AAA")
        };
    }

    (horizontal_segments, vertical_segments)
}

fn get_intersection_distances(horizontal_segments: &Vec<Segment>, vertical_segments: &Vec<Segment>) -> Vec<u32> {
    let mut distances = Vec::new();

    for horizontal_seg in horizontal_segments {
        for vertical_seg in vertical_segments {
            if horizontal_seg.contains_x(vertical_seg.p1.x) && vertical_seg.contains_y(horizontal_seg.p1.y) {
                let x = i32::abs(vertical_seg.p1.x) as u32;
                let y = i32::abs(horizontal_seg.p1.y) as u32;
                distances.push(x + y);
            }
        }
    }
    distances
}

fn main() {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open("input.txt") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open input.txt: {}", why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    assert!(lines.len() == 2, "Invalid input!");

    let (line_1_horizonal, line_1_vertical) = get_segments(lines[0]);
    let (line_2_horizonal, line_2_vertical) = get_segments(lines[1]);

    let mut intersections = get_intersection_distances(&line_1_horizonal, &line_2_vertical);
    intersections.append(&mut get_intersection_distances(&line_2_horizonal, &line_1_vertical));

    intersections.sort();
    println!("The shortest distance is {}", intersections[0]);
}
