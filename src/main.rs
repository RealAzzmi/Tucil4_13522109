use std::io;
use std::process;

#[derive(Clone, Copy)]
struct Path {
    node: usize,
    total_cost: f64,
}

impl Path {
    fn new() -> Self {
        Path {
            node: 0,
            total_cost: f64::INFINITY,
        }
    }

    fn with_cost(node: usize, total_cost: f64) -> Self {
        Path {
            node,
            total_cost,
        }
    }
}
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Please type a number!");

    if n <= 0 {
        println!("The number of nodes must be positive");
        process::exit(0);
    }

    let n: usize = n.try_into().expect("The number of nodes is too large");
    let mut adj = vec![vec![0.0; n]; n];
    for i in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let values: Vec<&str> = input.trim().split_whitespace().collect();

        for j in 0..n {
            if values[j] == "inf" {
                adj[i][j] = f64::INFINITY;
            } else {
                adj[i][j] = values[j]
                    .parse::<f64>()
                    .expect("Please type a valid number or 'inf'");
            }
        }
    }

    let mut dp: Vec<Vec<Path>> = vec![vec![Path::new(); n]; 1 << n];
    dp[1][0] = Path::with_cost(0, 0.0);

    for i in 2..(1 << n) {
        for j in 0..n {
            if (i & (1 << j)) == 0 {
                continue;
            }
            for k in 0..n {
                if (i & (1 << k)) == 0 || j == k {
                    continue;
                }
                if dp[i ^ (1 << j)][k].total_cost + adj[k][j] < dp[i][j].total_cost {
                    dp[i][j].total_cost = dp[i ^ (1 << j)][k].total_cost + adj[k][j];
                    dp[i][j].node = k;
                }
            }
        }
    }

    let mut shortest_path = f64::INFINITY;
    let mut last_node: usize = 0;
    for i in 1..n {
        let cost = dp[(1 << n) - 1][i].total_cost + adj[i][0];
        if cost <= shortest_path {
            shortest_path = cost;
            last_node = i;
        }
    }

    if n == 1 {
        println!("Shortest path length: {}", 0);
        println!("{}", 1);
    } else {
        println!("Shortest path length: {}", shortest_path);
        let mut cur = (1 << n) - 1;
        let mut path_nodes = vec![];
        path_nodes.push(1);
        for _ in 0..n {
            path_nodes.push(last_node + 1);
            let previous_node = last_node;
            last_node = dp[cur][last_node].node;
            cur ^= 1 << previous_node;
        }
        println!("{:?}", path_nodes);
    }
}
