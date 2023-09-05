use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Graph = Vec<Vec<usize>>;

pub fn part1(input: String) {
    let (_, graph, start, end) = process_input(input);

    let distance_map = dijkstra(&graph, start);
    println!("{}", distance_map[end]);
}

pub fn part2(input: String) {
    let (grid, graph, _, end) = process_input(input);

    let flat_grid: Vec<u8> = grid.into_iter().flatten().collect();

    let a_coords: Vec<_> = flat_grid
        .iter()
        .enumerate()
        .filter(|(_, &cell)| cell == b'a')
        .map(|(i, _)| i)
        .collect();

    let shortest_distance = a_coords
        .iter()
        .map(|&coord| dijkstra(&graph, coord)[end])
        .min()
        .unwrap();
    println!("{}", shortest_distance);
}

fn process_input(input: String) -> (Vec<Vec<u8>>, Graph, usize, usize) {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let (start, end) = {
        let mut start = 0;
        let mut end = 0;
        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &cell)| match cell {
                b'S' => start = i * row.len() + j,
                b'E' => end = i * row.len() + j,
                _ => (),
            })
        });
        (start, end)
    };

    grid[start / width][start % width] = b'a';
    grid[end / width][end % width] = b'z';

    let mut graph: Graph = vec![vec![]; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut neighbors = vec![];
            let current_height = grid[y][x];

            if y > 0 && grid[y - 1][x] <= current_height + 1 {
                neighbors.push((x, y - 1));
            }
            if y < height - 1 && grid[y + 1][x] <= current_height + 1 {
                neighbors.push((x, y + 1));
            }
            if x > 0 && grid[y][x - 1] <= current_height + 1 {
                neighbors.push((x - 1, y));
            }
            if x < width - 1 && grid[y][x + 1] <= current_height + 1 {
                neighbors.push((x + 1, y));
            }
            for (nx, ny) in neighbors {
                graph[y * width + x].push(ny * width + nx);
            }
        }
    }

    (grid, graph, start, end)
}

fn dijkstra(graph: &Graph, start: usize) -> Vec<usize> {
    let mut to_visit = BinaryHeap::new();
    let mut visited = vec![false; graph.len()];
    let mut distance = vec![usize::MAX; graph.len()];

    to_visit.push((Reverse(0), start));
    distance[start] = 0;

    while let Some((dist, node)) = to_visit.pop() {
        let Reverse(dist) = dist;

        if visited[node] {
            continue;
        }
        visited[node] = true;

        for &neighbor in &graph[node] {
            let new_dist = dist + 1;
            if new_dist < distance[neighbor] {
                distance[neighbor] = new_dist;
                if !visited[neighbor] {
                    to_visit.push((Reverse(new_dist), neighbor));
                }
            }
        }
    }

    distance
}
