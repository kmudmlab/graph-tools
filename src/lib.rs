pub mod union_find;
pub mod triangle_counting;
pub mod csr;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_csr(){
        let edges = vec![(1usize, 3usize), (3, 7), (3, 8),
                         (8, 9), (0, 2), (0, 4),
                         (0, 5), (2, 4), (4, 5), (5, 6)];
        
        let n_nodes = 10;
        let n_edges = edges.len();
        let graph = csr::CSR::from_edges(&edges, n_nodes, n_edges);
        for n in 0..n_nodes{
            println!("{}({}): {:?}", n, graph.degree(n), graph.neighbors(n));
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