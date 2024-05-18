use std::collections::HashMap;
use std::fs::File;
use rand::Rng;
use std::io::prelude::*;
use std::{env, vec};
use std::io::BufReader;



pub fn generate_random_undirected_graph(num_nodes:usize,  min_degree:usize,max_degree: usize,directed:bool, output_file:&str) -> std::io::Result<()> {
    assert!(min_degree<=max_degree,"min node degree must be <= then max node degree");
    let mut edges_nodes:HashMap<usize,Vec<usize>>  =   HashMap::with_capacity(num_nodes);
    let mut rng = rand::thread_rng();
    for n in 0..num_nodes {
        let mut num_edges = rng.gen_range(0..max_degree)+1;
        if min_degree>num_edges {
            num_edges = min_degree;
        }
        let  edges:Vec<usize> = Vec::new();
        edges_nodes.insert(n,edges);
        for _ in 0..num_edges {
            let mut edge_to =   rng.gen_range(0..num_nodes);
                while edges_nodes[&n].contains(&edge_to) {
                    edge_to = rng.gen_range(0..num_nodes);
                if directed {
                    if let successor = Some(edges_nodes.get(&edge_to).unwrap()) {
                        if !successor.unwrap().contains(&n) {
                            successor.unwrap().push(n)
                        }
                    } else {
                        edges_nodes.insert(edge_to,vec![n]);
                    }
                }
            }
            edges_nodes.get_mut(&n).unwrap().push(edge_to);




        }
    }

    write_graph(&edges_nodes, output_file)
}
fn write_graph(nodes_edges: &HashMap<usize, Vec<usize>>, file:&str) -> std::io::Result<()> {
    let batch_size = 200;
    let mut file = File::create(file)?;
    let mut str_buffer  = "".to_string();
    for (_l, (k,v)) in nodes_edges.iter().enumerate() {
        let mut edges = "".to_string();
        for i in 0..v.len()  {

            let separator = if v.len()>1 && i<v.len()-1 {
                ","
            } else {
                ""
            }.to_string();
            edges.push_str(format!("{}{}",v[i],separator).as_str())
        }

        str_buffer.push_str(format!("{k}:{edges}\n").as_str());
        if _l % batch_size == 0 {
            file.write(str_buffer.as_ref()).expect("TODO: panic message");
            str_buffer.clear();
        }
    }
    file.write(str_buffer.as_ref()).expect("TODO: panic message");

    Ok(())
}
pub fn brute_force_hamilton_path(nodes_edges:&HashMap<usize,Vec<usize>>){
 let graph:&mut Vec<Vec<bool>> = &mut vec![vec![];nodes_edges.len()];
    init_bit_graph(graph, nodes_edges);
   let mut  permutation =Vec::with_capacity(nodes_edges.len());
    for _ in 0..nodes_edges.len() {
        permutation.push(false);
    }
    permutation[0] = true;

    let has_hp =  brute_force_hp_rec(graph,permutation,0);




}
pub fn dyn_subset(graph:&Vec<Vec<bool>>)->bool{
    let n = graph.len();
    let node_subsets:&mut Vec<Vec<bool>> = &mut vec![vec![];n];
    for i in 0..n{
        node_subsets[i] = Vec::with_capacity(n^2);
        node_subsets[i][i^2] = true
    }
    for i in 0..n^2  {
        for j in 0..n{
            if j^2 % i ==0 {
                for k in 0..n{
                    if i %k^2 == 0 && graph[k][j] && node_subsets[k][i ^ (1 << j)]{
                        node_subsets[j][i] = true;
                    }
                }
            }
        }
    }

return false
}
pub fn init_bit_graph(graph:&mut Vec<Vec<bool>>, nodes_edges:&HashMap<usize,Vec<usize>>) {
    for (node,edges) in nodes_edges{
        let mut bin_edges:Vec<bool> = Vec::with_capacity(nodes_edges.len());
        for i in 0..nodes_edges.len(){
            if edges.contains(&i) {
                bin_edges.insert(i,true);
            } else {
                bin_edges.insert(i,false);
            }
        }
        graph[*node] = bin_edges;
    }
}

pub fn brute_force_hp_rec(graph:&Vec<Vec<bool>>,mut permutation : Vec<bool>,node:usize)->bool{
    return if node == graph.len() {
        println!("HAS HP ");
        true
    } else {
        for i in 0..graph.len() {
            if graph[node][i] && !permutation[i] && node !=i && i<=graph.len()-1{
                permutation[i] = true;
                return brute_force_hp_rec(graph, permutation, i+1);
            }
        }
        false
    }
}

fn perform_hp_search(filepath:&str, alg:usize) ->Result<(), Box<dyn std::error::Error>> {

        let mut edges_nodes: HashMap<usize, Vec<usize>> = HashMap::new();
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let parsed_n_e = line.unwrap().split(":").map(|x| x.to_string()).collect::<Vec<String>>();
            let node = parsed_n_e[0].parse::<usize>().unwrap();
            let edges = if !parsed_n_e[1].is_empty(){
                parsed_n_e[1].split(",").map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>()
            } else {
                Vec::new()
            };
            edges_nodes.insert(node,edges);
        }
    if alg == 0 {  brute_force_hamilton_path(&edges_nodes);}
        Ok(())
    }


