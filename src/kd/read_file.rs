use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const LNG_TO_METER: f64 = 85390.0;
const LAT_TO_METER: f64 = 111000.0;

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

// 0-5: NO,ID(唯一),POI名称,分类,具体地址
// 5-10: 经度,纬度,5级评分,人均消费（元）,所属POI分类代码
// 10-15: 所属POI大类,所属POI中类,所属POI小类,地区代码,地区名称

#[derive(Debug, Clone)]
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
    fn new_from_str(line: &str, ord: u32, config: &PointPositionConfig) -> Option<Self> {
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

    pub fn default() -> Self {
        Point {
            name: None,
            point_type: None,
            id: None,
            ord: 0,
            lng: 0.0,
            lat: 0.0,
        }
    }

    pub fn get_value_at_dimesion(&self, d: bool) -> f64 {
        if !d {
            self.lng
        } else {
            self.lat
        }
    }

    pub fn get_distance(&self, other: &Self) -> f64 {
        let x = (self.lng - other.lng) * LNG_TO_METER;
        let y = (self.lat - other.lat) * LAT_TO_METER;
        (x * x + y * y).sqrt()
    }

    pub fn get_dis_to_axis(&self, d: bool, value: f64) -> f64 {
        if !d {
            (self.lng - value).abs() * LNG_TO_METER
        } else {
            (self.lat - value).abs() * LAT_TO_METER
        }
    }
}

pub fn read_file_point_list(file_path: &str, config: &PointPositionConfig) -> Result<Vec<Point>> {
    let mut result: Vec<Point> = vec![];
    let mut ord: u32 = 0;

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let s = line?;
        if let Some(p) = Point::new_from_str(&s, ord, config) {
            result.push(p);
            ord += 1;
        }
    }

    Ok(result)
}
