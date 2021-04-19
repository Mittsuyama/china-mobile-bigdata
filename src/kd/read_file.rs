use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub struct PointPositionConfig {
    name: i32,
    id: i32,
    point_type: i32,
    lng: i32,
}

impl PointPositionConfig {
    pub fn new(name: i32, id: i32, point_type: i32, lng: i32) -> Self {
        PointPositionConfig {
            name,
            id,
            point_type,
            lng,
        }
    }
}

pub struct Point {
    name: Option<String>,
    id: Option<String>,
    point_type: Option<String>,
    ord: u32,
    lng: f64,
    lat: f64,
}

fn find_str_at_position(s: &Vec<&str>, p: i32) -> Option<String> {
    if p < 0 {
        None
    } else {
        Some(s[p as usize].to_owned())
    }
}

impl Point {
    fn new(line: &str, ord: u32, config: &PointPositionConfig) -> Option<Self> {
        let lng = config.lng as usize;
        let s: Vec<&str> = line.split(',').collect();

        if let (Ok(lng), Ok(lat)) = (s[lng].parse::<f64>(), s[lng + 1].parse::<f64>()) {
            let name = find_str_at_position(&s, config.name);
            let point_type = find_str_at_position(&s, config.point_type);
            let id = find_str_at_position(&s, config.id);

            Some(Point {
                name,
                point_type,
                id,
                ord,
                lng,
                lat,
            })
        } else {
            None
        }
    }
}

pub fn read_file_point_list(file_path: &str, config: &PointPositionConfig) -> Result<Vec<Point>> {
    let mut result: Vec<Point> = vec![];
    let mut ord: u32 = 0;

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        if let Some(p) = Point::new(&line?, ord, config) {
            result.push(p);
            ord += 1;
        }
    }

    Ok(result)
}
