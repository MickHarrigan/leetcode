struct Solution;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut counts = vec![true; n as usize];
        let mut vis = Vec::new();
        let mut stack = Vec::new();

        // find the root
        // root has no parents
        (0..n as usize).for_each(|i| {
            if left_child[i] != -1 {
                counts[left_child[i] as usize] = false;
            }
            if right_child[i] != -1 {
                counts[right_child[i] as usize] = false;
            }
        });

        let mut curr = match counts.iter().position(|item| item == &true) {
            Some(a) => a as i32,
            None => return false,
        };

        // this is just a local function that simplifies the adding of non -1 values
        let fnc = |s: &mut Vec<i32>, &val: &i32| match val {
            -1 => {}
            n => s.push(n),
        };
        vis.push(curr);
        fnc(&mut stack, &left_child[curr as usize]);
        fnc(&mut stack, &right_child[curr as usize]);
        while vis.len() < n as usize {
            match stack.pop() {
                Some(item) => {
                    curr = item;
                    vis.push(item);
                    fnc(&mut stack, &left_child[curr as usize]);
                    fnc(&mut stack, &right_child[curr as usize]);
                }
                None => return false,
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let sol = Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]);
    println!("{:?}", sol);
    let sol = Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]);
    println!("{:?}", sol);
    let sol = Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]);
    println!("{:?}", sol);

    let sol = Solution::validate_binary_tree_nodes(
        6,
        vec![1, -1, -1, 4, -1, -1],
        vec![2, -1, -1, 5, -1, -1],
    );
    println!("{:?}", sol);

    let sol = Solution::validate_binary_tree_nodes(4, vec![3, -1, 1, -1], vec![-1, -1, 0, -1]);
    println!("{:?}", sol);
}
