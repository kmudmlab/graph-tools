pub mod union_find;
pub mod triangle_counting;
pub mod csr;
pub mod heap;
pub mod slashburn;

#[cfg(test)]
mod test{
    use super::*;

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
        
        let cnt = triangle_counting::count_total(adj);
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