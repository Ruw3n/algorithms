mod string_algorithms;
mod graph_algorithms;

use crate::graph_algorithms::hp_alg::generate_random_undirected_graph;

fn main() {
    generate_random_undirected_graph(8000 ,5,100,false,"out/test").expect("error");
    //perform_hp_search("out/test",1).expect("error");
}