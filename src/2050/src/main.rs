struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        // start from the last class and work backwards with DFS
        let relations: Vec<(i32, i32)> =
            relations.into_iter().map(|rel| (rel[0], rel[1])).collect();

        let mut paths = Vec::new();

        for i in 1..=n {
            paths.extend(Self::dfs(&relations, vec![i], vec![]));
        }

        println!("{:?}", paths);

        Self::longest_path(paths, time)
    }

    pub fn longest_path(paths: Vec<Vec<i32>>, times: Vec<i32>) -> i32 {
        // make this more functional and with folds or something
        let mut time = 0;
        let mut temp = 0;
        for path in paths.iter() {
            for course in path.iter() {
                temp += times[(course - 1) as usize];
            }
            time = time.max(temp);
            temp = 0;
        }
        time
    }

    pub fn dfs(relations: &Vec<(i32, i32)>, path: Vec<i32>, paths: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut path = path;
        let mut paths = paths;
        let curr = *path.last().unwrap();
        if relations.iter().any(|(a, _b)| a == &curr) {
            // push all the b's of relations that have curr as the a
            for (start, end) in relations.iter() {
                if start == &curr {
                    let old = path.clone();
                    path.push(*end);
                    paths = Self::dfs(relations, path.clone(), paths);
                    path = old;
                }
            }
        } else {
            paths.push(path);
        }
        paths
    }
}

fn main() {
    let sol = Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![2, 3, 5]);
    println!("{:?}", sol);
    let sol = Solution::minimum_time(
        5,
        vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
        vec![1, 2, 3, 4, 5],
    );
    println!("{:?}", sol);
}


// solved on the website with 
// I ran out of time today and my solution *should* work, its just slow lol
// use std::collections::VecDeque;

// impl Solution {
//     pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
//         let mut indeg = vec![0; n as usize];
//         let mut adj = vec![vec![]; n as usize];
//         relations.into_iter().for_each(|e| {
//             adj[e[0] as usize - 1].push(e[1] as usize - 1);
//             indeg[e[1] as usize - 1] += 1;
//         });

//         let mut q = indeg.iter().enumerate().filter(|t| t.1 == &0).map(|t| t.0).collect::<VecDeque<usize>>();
//         let mut ans = time.clone();
//         while let Some(node) = q.pop_front() {
//             adj[node].iter().for_each(|&next| {
//                 indeg[next] -= 1;
//                 if indeg[next] == 0 { q.push_back(next) }
//                 ans[next] = ans[next].max(ans[node] + time[next]);
//             });
//         }

//         ans.into_iter().max().unwrap()
//     }
// }
