pub mod read_file;
use read_file::Point;

pub struct Tree {
    id: u32,
    l: u32,
    r: u32,
    d: u32,
    value: f64,
}

fn recursion_build_tree(id: &mut u32, list: &mut Vec<Point>, l: u32, r: u32) {

}

pub fn build_kd_tree(list: &mut Vec<Point>) -> Vec<Tree> {
    let mut tree: Vec<Tree> = vec![];
    let mut root = 0;

    tree
}