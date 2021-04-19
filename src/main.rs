use std::io::Result;

mod kd;
use kd::{ Tree, build_kd_tree };
use kd::read_file::{ read_file_point_list, PointPositionConfig };

fn main() -> Result<()> {
    // let cell_list = read_file_point_list("cell.txt", &PointPositionConfig::new(0, -1, -1, 11));

    if let Ok(mut poi_list) = read_file_point_list("poi.txt", &PointPositionConfig::new(0, -1, -1, 3)) {
        let kd_tree = build_kd_tree(&mut poi_list);
    }

    Ok(())
}
