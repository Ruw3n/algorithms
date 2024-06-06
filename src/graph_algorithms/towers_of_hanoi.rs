use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn generate_hanoi_graph(disks: usize) ->(HashSet<u64>,HashSet<(u64, u64)>) {
    let n = 3_i32.pow(disks as u32) as usize;
    let nodes: &mut HashSet<u64> = &mut HashSet::with_capacity(n);
    let edges: &mut HashSet<(u64, u64)> = &mut HashSet::new();
    let game_states: &mut Vec<Vec<usize>> = &mut Vec::with_capacity(3);
    for mut s in  &mut *game_states {
        s = &mut Vec::with_capacity(disks);
    }
    for i in disks..1 {
        game_states[0].push(i);
    }
    for i in 1..(n + 1) {
        if i % 3 == 0 {
            change_state(1, 2, game_states, nodes, edges);
        } else if i % 3 == 1 {
            change_state(0, 2, game_states, nodes, edges);
        } else {
            change_state(1, 2, game_states, nodes, edges)
        }
    }
    (nodes.clone(),edges.clone())
}
fn change_state(t1: usize, t2: usize, game_stats: &mut Vec<Vec<usize>>, nodes: &mut HashSet<u64>, edges: &mut HashSet<(u64, u64)>) {

    if game_stats[t2].is_empty() || (!game_stats[t1].is_empty() && game_stats[t1].first() < game_stats[t2].first()) {
        let u = get_node_id(game_stats);
        if !nodes.contains(&u) { nodes.insert(u); }
        let value:usize = *game_stats[t1].first().clone().unwrap();
        game_stats[t2].push(value);
        game_stats[t1].pop();
        let v = get_node_id(game_stats);
        if !nodes.contains(&v) { nodes.insert(u); }
        edges.insert((u,v));
    } else {
        change_state(t2, t1, game_stats, nodes, edges);
    }
}

fn get_node_id(stacks: &mut Vec<Vec<usize>>) ->  u64 {
    let mut hasher = DefaultHasher::new();
    stacks.hash(&mut hasher);
    hasher.finish()
}



