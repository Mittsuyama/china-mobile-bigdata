pub mod read_file;
use read_file::Point;
use rand::random;
use std::cmp::Ordering;

pub struct Tree {
    l: Option<Box<Tree>>,
    r: Option<Box<Tree>>,
    p: Point,
}

impl Tree {
    fn get_value_at_dimesion(&self, d: bool) -> f64 {
        self.p.get_value_at_dimesion(d)
    }

    fn recursion_search_tree<'a>(
        &self,
        p: &Point,
        d: bool,
        k: usize,
        res: &mut Vec<ResultPoint>,
    ) {
        if p.get_value_at_dimesion(d) < self.get_value_at_dimesion(d)  {
            if let Some(l) = &self.l {
                l.recursion_search_tree(p, !d, k, res);
            }
        } else {
            if let Some(r) = &self.r {
                r.recursion_search_tree(p, !d, k, res);
            }
        };

        let dis = p.get_distance(&self.p);

        if res.len() < k {
            res.push(ResultPoint { dis, p: self.p.clone() } )
        } else {
            replace_result_vec(res, p, dis);
        }

        let mut flag = false;
        if res.len() < k {
            flag = true;
        } else if let Some(point) = res.iter().max_by_key(|&item| item) {
            if point.dis > p.get_dis_to_axis(d, self.get_value_at_dimesion(d)) {
                flag = true
            }
        }
        if flag {
            if p.get_value_at_dimesion(d) > self.get_value_at_dimesion(d)  {
                if let Some(l) = &self.l {
                    l.recursion_search_tree(p, !d, k, res);
                }
            } else {
                if let Some(r) = &self.r {
                    r.recursion_search_tree(p, !d, k, res);
                }
            };
        }
    }

    pub fn search_tree(&self, p: &Point, k: usize) -> Vec<ResultPoint> {
        let mut res: Vec<ResultPoint> = vec![];
        self.recursion_search_tree(p, false, k, &mut res);
        res
    }
}

#[derive(Debug)]
pub struct ResultPoint {
    dis: f64,
    p: Point,
}

impl Ord for ResultPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(o) = self.dis.partial_cmp(&other.dis) {
            o
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for ResultPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for ResultPoint {
    fn eq(&self, other: &Self) -> bool {
        &self.dis == &other.dis
    }
}

impl Eq for ResultPoint {}

fn replace_result_vec(v: &mut Vec<ResultPoint>, p: &Point, dis: f64) {
    if let Some((key, res_p)) = v.iter().enumerate().max_by_key(|&(_, item)| item) {
        if res_p.dis > dis {
            v[key] = ResultPoint {
                dis,
                p: p.clone(),
            }
        }
    }
}


fn recursion_build_tree(
    point: &mut [Point],
    d: bool,
) -> Option<Box<Tree>> {
    let len = point.len();
    
    if len <= 0 {
        return None;
    }

    if len == 1 {
        return Some(Box::new(Tree {
            l: None,
            r: None,
            p: point[0].clone(),
        }));
    }

    let mut i = 0;
    let mut j = len - 1;
    let mid_value = point[random::<usize>() % len].get_value_at_dimesion(d);
    while i < j {
        while point[i].get_value_at_dimesion(d) < mid_value { i += 1; }
        while point[j].get_value_at_dimesion(d) > mid_value { j -= 1; }
        if point[i].get_value_at_dimesion(d) == point[j].get_value_at_dimesion(d) {
            i += 1;
        }
        point.swap(i, j)
    }

    let p = point[i].clone();

    let (left_include, right) = point.split_at_mut(i);
    let l = if i > 1 {
        let (left, _) = left_include.split_at_mut(i - 1);
        recursion_build_tree(left, !d)
    } else {
        None
    };
    let r = recursion_build_tree(right, !d);

    Some(Box::new(Tree { l, r, p }))
}

pub fn build_kd_tree(point: &mut Vec<Point>) -> Option<Box<Tree>> {
    recursion_build_tree(&mut point[..], false)
}
