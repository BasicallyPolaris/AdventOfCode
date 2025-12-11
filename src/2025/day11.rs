use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::env;
use std::fs;

const YOU_NODE: &str = "you";
const END_NODE: &str = "out";
const SVR_NODE: &str = "svr";
const FFT_NODE: &str = "fft";
const DAC_NODE: &str = "dac";

const YOU_NODE_IDX: u32 = 0;
const END_IDX: u32 = 1;
const SVR_NODE_IDX: u32 = 2;
const FFT_NODE_IDX: u32 = 3;
const DAC_NODE_IDX: u32 = 4;

const INPUT_BASE: &str = "src/2025/input/";
const CURR_DAY: u8 = 11;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.iter().any(|arg| arg == "--test");

    let filename = if is_test_mode {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, "_test.input")
    } else {
        format!("{}{}{}{}", INPUT_BASE, "day", CURR_DAY, ".input")
    };

    println!(
        "📆 Day {}{}:",
        CURR_DAY,
        if is_test_mode { " (DEBUG)" } else { "" }
    );
    println!("------------");

    let file_content = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    let edges_raw: Vec<(&str, &str)> = file_content
        .lines()
        .flat_map(|line| {
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            assert!(tokens.len() >= 2);

            let start_node = tokens[0].trim_end_matches(':');

            tokens[1..]
                .iter()
                .map(move |&target| (start_node, target))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();

    let mut graph = DiGraph::<&str, ()>::new();
    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

    let s_idx = graph.add_node(YOU_NODE);
    let e_idx = graph.add_node(END_NODE);
    let srv_idx = graph.add_node(SVR_NODE);
    let fft_idx = graph.add_node(FFT_NODE);
    let dac_idx = graph.add_node(DAC_NODE);

    assert_eq!(s_idx.index() as u32, YOU_NODE_IDX);
    assert_eq!(e_idx.index() as u32, END_IDX);
    assert_eq!(srv_idx.index() as u32, SVR_NODE_IDX);
    assert_eq!(fft_idx.index() as u32, FFT_NODE_IDX);
    assert_eq!(dac_idx.index() as u32, DAC_NODE_IDX);

    node_map.insert(YOU_NODE, s_idx);
    node_map.insert(END_NODE, e_idx);
    node_map.insert(SVR_NODE, srv_idx);
    node_map.insert(FFT_NODE, fft_idx);
    node_map.insert(DAC_NODE, dac_idx);

    for (source, target) in edges_raw {
        let source_idx = *node_map
            .entry(source)
            .or_insert_with(|| graph.add_node(source));

        let target_idx = *node_map
            .entry(target)
            .or_insert_with(|| graph.add_node(target));

        graph.add_edge(source_idx, target_idx, ());
    }

    task_one(&graph);
    task_two(&graph);

    println!();
}

fn task_one(graph: &DiGraph<&str, ()>) {
    let start_idx = NodeIndex::new(YOU_NODE_IDX as usize);
    let end_idx = NodeIndex::new(END_IDX as usize);

    let path_count =
        all_simple_paths::<Vec<_>, _, RandomState>(graph, start_idx, end_idx, 0, None).count();
    println!(
        "Task 1: Amount of different paths between you and out: {}",
        path_count
    );
}

fn count_paths_dag(
    graph: &DiGraph<&str, ()>,
    start: NodeIndex,
    end: NodeIndex,
    cache: &mut HashMap<NodeIndex, usize>,
) -> usize {
    if start == end {
        return 1;
    }

    if let Some(&count) = cache.get(&start) {
        return count;
    }

    let mut total_paths = 0;
    for neighbor in graph.neighbors(start) {
        total_paths += count_paths_dag(graph, neighbor, end, cache);
    }

    cache.insert(start, total_paths);
    total_paths
}

fn task_two(graph: &DiGraph<&str, ()>) {
    let svr_idx = NodeIndex::new(SVR_NODE_IDX as usize);
    let end_idx = NodeIndex::new(END_IDX as usize);
    let dac_idx = NodeIndex::new(DAC_NODE_IDX as usize);
    let fft_idx = NodeIndex::new(FFT_NODE_IDX as usize);

    let get_count = |from, to| {
        let mut cache = HashMap::new();
        count_paths_dag(graph, from, to, &mut cache)
    };

    let paths_svr_fft = get_count(svr_idx, fft_idx);
    let paths_fft_dac = get_count(fft_idx, dac_idx);
    let paths_dac_out = get_count(dac_idx, end_idx);
    let route_svr_fft_dac_out = paths_svr_fft * paths_fft_dac * paths_dac_out;

    let paths_svr_dac = get_count(svr_idx, dac_idx);
    let paths_dac_fft = get_count(dac_idx, fft_idx);
    let paths_fft_out = get_count(fft_idx, end_idx);
    let oute_svr_dac_fft_out = paths_svr_dac * paths_dac_fft * paths_fft_out;

    let total_paths = route_svr_fft_dac_out + oute_svr_dac_fft_out;

    println!(
        "Task 2: Amount of different paths between Server and Out over fft and dac: {}",
        total_paths
    );
}
