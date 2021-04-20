use std::io::Result;

mod kd;
use kd::read_file::{ read_file_point_list, PointPositionConfig, Point };

fn main() -> Result<()> {
    if let Ok(mut poi_list) = read_file_point_list("data/poi.txt", &PointPositionConfig::new(0, -1, -1, 5)) {
        if let Some(tree) = kd::build_kd_tree(&mut poi_list) {
            for item in tree.search_tree(&Point::default(), 5) {
                println!("result: {:?}", item);
            }
        } else {
            println!("build kd tree failed!");
        }
    } else {
        println!("No poi information!");
    }

    Ok(())
}
