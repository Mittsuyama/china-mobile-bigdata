use std::io::Result;

mod kd;
use kd::Tree;

mod read_file;
use read_file::{read_file_point_list, PointPositionConfig, Point};

fn main() -> Result<()> {
    let poi_list = read_file_point_list("poi.txt", &PointPositionConfig::new(0, -1, -1, 3));
    // let cell_list = read_file_point_list("cell.txt", &PointPositionConfig::new(0, -1, -1, 11));

    Ok(())
}
