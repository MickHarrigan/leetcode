struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let (mut starts, mut ends): (Vec<i32>, Vec<i32>) =
            flowers.into_iter().map(|each| (each[0], each[1])).unzip();
        starts.sort_by(|a, b| a.cmp(&b));
        ends.sort_by(|a, b| a.cmp(&b));
        let mut out = Vec::new();
        for person in people.iter() {
            let start = starts.partition_point(|val| val <= person);
            let end = ends.partition_point(|val| val < person);
            out.push((start - end) as i32);
        }
        out
    }
}

fn main() {
    let sol = Solution::full_bloom_flowers(
        vec![(1, 6), (3, 7), (9, 12), (4, 13)]
            .iter()
            .map(|each| vec![each.0, each.1])
            .collect(),
        vec![2, 3, 7, 11],
    );
    println!("{:?}", sol);
    let sol = Solution::full_bloom_flowers(
        vec![(1, 10), (3, 3)]
            .iter()
            .map(|each| vec![each.0, each.1])
            .collect(),
        vec![3, 3, 2],
    );
    println!("{:?}", sol);
}

