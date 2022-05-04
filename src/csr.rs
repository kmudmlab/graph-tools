pub struct CSR{
    nodes: Box<[usize]>,
    edges: Box<[usize]>
}

impl CSR{
    pub fn from_edges(edges: &[(usize, usize)], n_nodes: usize) -> CSR{
        let n_edges = edges.len();
        let mut csr = CSR{
            edges: vec![0; n_edges*2].into_boxed_slice(),
            nodes: vec![0; n_nodes + 1].into_boxed_slice(),
        };

        for (u, v) in edges {
            csr.nodes[*u+1] += 1;
            csr.nodes[*v+1] += 1;
        }

        for i in 0..n_nodes {
            csr.nodes[i+1] += csr.nodes[i];
        }
        
        for (u, v) in edges {
            csr.edges[csr.nodes[*u]] = *v;
            csr.nodes[*u] += 1;
            csr.edges[csr.nodes[*v]] = *u;
            csr.nodes[*v] += 1;
        }
        
        for i in (0..n_nodes).rev() {
            csr.nodes[i+1] = csr.nodes[i];
        }
        csr.nodes[0] = 0;

        return csr;
    }

    pub fn degree(&self, u: usize) -> usize {
        return self.nodes[u+1] - self.nodes[u];
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        return &self.edges[self.nodes[u]..self.nodes[u+1]];
    }
}
