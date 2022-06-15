pub struct CSR{
    nodes: Vec<usize>,
    edges: Vec<usize>
}

impl CSR{

    pub fn from_sorted_edges(edges: &[(usize, usize)], n_nodes: usize) -> CSR{
        let n_edges = edges.len();

        
        let mut csr = CSR{
            edges: vec![0; n_edges],
            nodes: vec![0; n_nodes + 1]
        };
        
        for (i, (u, v)) in edges.iter().enumerate() {
            csr.edges[i] = *v;
            csr.nodes[*u + 1] += 1;
        }
        
        for i in 1..n_nodes {
            csr.nodes[i+1] += csr.nodes[i];
        }

        return csr;
    }

    pub fn from_edges(edges: &[(usize, usize)], n_nodes: usize) -> CSR{
        let n_edges = edges.len();
        let mut csr = CSR{
            edges: vec![0; n_edges*2],
            nodes: vec![0; n_nodes + 1],
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

    pub fn iter_edges(&self) -> CSREdgeIterator{
        return CSREdgeIterator{
            csr: self,
            n_cur: 0,
            e_cur: 0
        }
    }

    pub fn n_nodes(&self) -> usize{
        return self.nodes.len() - 1;
    }
}

pub struct CSREdgeIterator<'a>{
    csr: &'a CSR,
    n_cur: usize,
    e_cur: usize
}

impl<'a> Iterator for CSREdgeIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.e_cur < self.csr.edges.len() {
            
            while self.csr.nodes[self.n_cur+1] <= self.e_cur {
                self.n_cur += 1;
            }

            self.e_cur += 1;

            return Some((self.n_cur, self.csr.edges[self.e_cur-1]));
        }

        return None;
    }
}