use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq)]
struct Point {
    dist: i32,
    val: Vec<i32>,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Point {
    fn new(point: Vec<i32>) -> Self {
        let dist = point[0] * point[0] + point[1] * point[1];
        Self { dist, val: point }
    }

    fn to_vec(self) -> Vec<i32> {
        return self.val;
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for p in points {
            heap.push(Point::new(p));
            if heap.len() == k as usize + 1 {
                heap.pop();
            }
        }

        heap.into_iter().map(|entry| entry.to_vec()).collect()
    }
}
