pub mod union_find;

#[cfg(test)]
mod test{
    use super::*;

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