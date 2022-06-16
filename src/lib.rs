pub mod union_find;
pub mod tricnt;
pub mod csr;
pub mod heap;
pub mod slashburn;
pub mod csbv;


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_csr_dump_load(){
        let edges = vec![(1, 37), (1,40), (1,68), (37, 40), (37,68), (37,75), (40, 68), (40, 75)];
        let n_nodes = 76;

        let original = csr::CSR::from_sorted_edges(&edges, n_nodes);

        let filepath = "test_csr_dump_load.bin";

        original.dump(filepath).unwrap();
        
        let dump_loaded = csr::CSR::load(filepath).unwrap();

        assert_eq!(original.nodes, dump_loaded.nodes);
        assert_eq!(original.edges, dump_loaded.edges);

        std::fs::remove_file(filepath).unwrap();
    }

    #[test]
    fn test_csbv_dump_load(){
        let edges = vec![(1, 37), (1,40), (1,68), (37, 40), (37,68), (37,75), (40, 68), (40, 75)];
        let n_nodes = 76;

        let original = csbv::CSBV::from_sorted_edges(&edges, n_nodes);

        let filepath = "test_csbv_dump_load.bin";

        original.dump(filepath).unwrap();
        
        let dump_loaded = csbv::CSBV::load(filepath).unwrap();

        assert_eq!(original.bit_blocks, dump_loaded.bit_blocks);
        assert_eq!(original.block_ids, dump_loaded.block_ids);
        assert_eq!(original.ptrs, dump_loaded.ptrs);

        std::fs::remove_file(filepath).unwrap();
    }

    #[test]
    fn test_tri_csbv(){
        let edges = vec![(1, 37), (1,40), (1,68), (37, 40), (37,68), (37,75), (40, 68), (40, 75)];
        let n_nodes = 76;

        let graph = csbv::CSBV::from_sorted_edges(&edges, n_nodes);

        let cnt = tricnt::csbv::count(&graph);
        assert_eq!(cnt, 5);
    }

    #[test]
    fn test_tri_csr(){
        let edges = vec![(1, 37), (1,40), (1,68), (37, 40), (37,68), (37,75), (40, 68), (40, 75)];
        let n_nodes = 76;

        let graph = csr::CSR::from_sorted_edges(&edges, n_nodes);

        let cnt = tricnt::csr::count(&graph);
        assert_eq!(cnt, 5);
    }

    
    #[test]
    fn test_csbv(){

        let edges = vec![(0,1), (0, 10), (0, 50), (0, 64), (0, 127), (3, 64), (3, 127)];
        let n_nodes = 128;

        let graph = csbv::CSBV::from_sorted_edges(&edges, n_nodes);

        assert_eq!(graph.bit_blocks, vec![1125899906843650, 9223372036854775809, 9223372036854775809]);
        assert_eq!(graph.block_ids, vec![0,1,1]);
        assert_eq!(graph.ptrs, vec![0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);

        assert_eq!(graph.neighbor_iter(0).collect::<Vec<usize>>(), vec![1,10,50,64,127]);
        assert_eq!(graph.neighbor_iter(3).collect::<Vec<usize>>(), vec![64,127]);

        assert_eq!(graph.block_iter(0).collect::<Vec<(usize, usize)>>(), vec![(0, 1125899906843650), (1, 9223372036854775809)]);
        assert_eq!(graph.block_iter(3).collect::<Vec<(usize, usize)>>(), vec![(1, 9223372036854775809)]);

    }

    #[test]
    fn test_heap(){
        let priorities = vec![6usize,3,7,1,9,5,4,8];
        let mut indices : Vec<usize> = (0..priorities.len()).collect();

        heap::heapify_indices(&mut indices, &priorities);
        heap::sort_heap_indices(&mut indices, &priorities);
        
        let sorted_priorities : Vec<usize> = indices.iter().map(|x| priorities[*x]).collect();
        let mut true_sorted_priorities = priorities.clone();
        true_sorted_priorities.sort_by(|a, b| b.cmp(a));
        
        assert_eq!(sorted_priorities, true_sorted_priorities);
    }

    #[test]
    fn test_csr_edge_iterator(){
        let edges = vec![(1usize,3usize), (1,4), (1,5), (4,5)];
        let n_nodes = 6;
        let graph = csr::CSR::from_edges(&edges, n_nodes);
        
        let mut true_edges = vec![];
        for (u, v) in &edges {
            true_edges.push( (*u, *v) );
            true_edges.push( (*v, *u) );
        }

        let mut edges_from_iter : Vec<(usize, usize)> = graph.iter_edges().collect();

        true_edges.sort();
        edges_from_iter.sort();

        assert_eq!(true_edges, edges_from_iter);
    }

    #[test]
    fn test_csr(){
        let edges = vec![(1usize, 3usize), (3, 8), (3, 7),
                         (8, 9), (0, 4), (0, 2),
                         (0, 5), (2, 4), (4, 5), (5, 6)];
        
        let n_nodes = 10;
        let graph = csr::CSR::from_edges(&edges, n_nodes);
        
        //check degrees
        let true_degrees = [3,1,2,3,3,3,1,1,2,1];

        for n in 0..n_nodes{
            assert_eq!(graph.degree(n), true_degrees[n]);
        }

        //check neighbors
        let true_neighbors = [vec![2,4,5],vec![3],vec![0,4],vec![1,7,8],
                            vec![0,2,5],vec![0,4,6],vec![5],vec![3],vec![3,9],vec![8]];
        
        for n in 0..n_nodes{
            let mut neighbors : Vec<usize> = vec![0; graph.degree(n)];
            neighbors.clone_from_slice(graph.neighbors(n));
            neighbors.sort();
            assert_eq!(neighbors, true_neighbors[n]);
        }
    }

    #[test]
    fn test_tri(){
        let adj = vec![
            vec![2,4,5],
            vec![3],
            vec![4,5],
            vec![7,8],
            vec![5],
            vec![6],
            vec![],
            vec![],
            vec![9],
            vec![]
            ];
        
        let cnt = tricnt::count_total(adj);
        assert_eq!(cnt, 4);
    }

    #[test]
    fn test_uf(){
        let mut uf = union_find::UnionFind::new(10);
        
        let edges = vec![(1usize, 3usize), (3, 7), (3, 8),
                         (8, 9), (0, 2), (0, 4),
                         (0, 5), (2, 4), (4, 5), (5, 6)];
        
        for (u, v) in &edges{
            uf.union(*u, *v);
        }
        
        assert_eq!(uf.n_components(), 2);
        assert_eq!(uf.n_nodes(), 10);
        
        let true_ccs = vec![vec![1usize,3,7,8,9], vec![0usize,2,4,5,6]];

        // test if ccs are correctly found
        for cc in &true_ccs {
            cc.iter().reduce(|accum, item| {
                assert_eq!(uf.find(*accum), uf.find(*item));
                return accum;
            });
        }

        // test if nodes in different ccs are identified.
        for u in &true_ccs[0]{
            for v in &true_ccs[1]{
                assert_ne!(uf.find(*u), uf.find(*v));
            }
        }
    }
}