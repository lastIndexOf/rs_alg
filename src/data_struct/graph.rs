// // ad
// pub struct NodeNotInGraph {}

// pub struct UndirectedGraph {}

// pub struct DirectedGraph {}

// trait Graph {}

// #[cfg(test)]
// mod test_undirected_graph {
//     use super::*;

//     #[test]
//     fn test_add_edge() {
//         let mut graph = UndirectedGraph::new();

//         graph.add_edge(("a", "b", 5));
//         graph.add_edge(("b", "c", 10));
//         graph.add_edge(("c", "a", 7));

//         let expected_edges = [
//             (&String::from("a"), &String::from("b"), 5),
//             (&String::from("b"), &String::from("a"), 5),
//             (&String::from("c"), &String::from("a"), 7),
//             (&String::from("a"), &String::from("c"), 7),
//             (&String::from("b"), &String::from("c"), 10),
//             (&String::from("c"), &String::from("b"), 10),
//         ];
//         for edge in expected_edges.iter() {
//             assert!(graph.edges().contains(edge));
//         }
//     }

//     #[test]
//     fn test_neighbours() {
//         let mut graph = UndirectedGraph::new();

//         graph.add_edge(("a", "b", 5));
//         graph.add_edge(("b", "c", 10));
//         graph.add_edge(("c", "a", 7));

//         assert_eq!(
//             graph.neighbours("a").unwrap(),
//             &vec![(String::from("b"), 5), (String::from("c"), 7)]
//         );
//     }
// }

// #[cfg(test)]
// mod test_directed_graph {
//     use super::*;

//     #[test]
//     fn test_add_node() {
//         let mut graph = DirectedGraph::new();
//         graph.add_node("a");
//         graph.add_node("b");
//         graph.add_node("c");
//         assert_eq!(
//             graph.nodes(),
//             [&String::from("a"), &String::from("b"), &String::from("c")]
//                 .iter()
//                 .cloned()
//                 .collect()
//         );
//     }

//     #[test]
//     fn test_add_edge() {
//         let mut graph = DirectedGraph::new();

//         graph.add_edge(("a", "b", 5));
//         graph.add_edge(("c", "a", 7));
//         graph.add_edge(("b", "c", 10));

//         let expected_edges = [
//             (&String::from("a"), &String::from("b"), 5),
//             (&String::from("c"), &String::from("a"), 7),
//             (&String::from("b"), &String::from("c"), 10),
//         ];
//         for edge in expected_edges.iter() {
//             assert!(graph.edges().contains(edge));
//         }
//     }

//     #[test]
//     fn test_neighbours() {
//         let mut graph = DirectedGraph::new();

//         graph.add_edge(("a", "b", 5));
//         graph.add_edge(("b", "c", 10));
//         graph.add_edge(("c", "a", 7));

//         assert_eq!(
//             graph.neighbours("a").unwrap(),
//             &vec![(String::from("b"), 5)]
//         );
//     }

//     #[test]
//     fn test_contains() {
//         let mut graph = DirectedGraph::new();
//         graph.add_node("a");
//         graph.add_node("b");
//         graph.add_node("c");
//         assert!(graph.contains("a"));
//         assert!(graph.contains("b"));
//         assert!(graph.contains("c"));
//         assert!(!graph.contains("d"));
//     }
// }
