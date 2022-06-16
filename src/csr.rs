use std::fs::File;
use std::io::{Result, BufWriter, BufReader, Write, Read};

pub struct CSR{
    pub nodes: Vec<usize>,
    pub edges: Vec<usize>
}

impl CSR{

    pub fn dump(&self, filepath: &str) -> Result<()>{
        let mut bw = BufWriter::new(File::create(filepath)?);

        // write n_nodes, n_edges
        bw.write_all(&self.nodes.len().to_ne_bytes())?;
        bw.write_all(&self.edges.len().to_ne_bytes())?;
        
        // write data
        for n in &self.nodes {
            bw.write_all(&n.to_ne_bytes())?;
        }
        for n in &self.edges {
            bw.write_all(&n.to_ne_bytes())?;
        }

        bw.flush()?;

        return Ok(());
    }

    pub fn load(filepath: &str) -> Result<CSR>{
        let mut br = BufReader::new(File::open(filepath)?);

        let mut buffer = [0u8; std::mem::size_of::<usize>()];

        // read n_nodes, n_edges
        br.read_exact(&mut buffer)?;
        let n_nodes = usize::from_ne_bytes(buffer);
        br.read_exact(&mut buffer)?;
        let n_edges = usize::from_ne_bytes(buffer);

        let mut csr = CSR{
            edges: vec![0usize; n_edges],
            nodes: vec![0usize; n_nodes]
        };

        for i in 0..n_nodes {
            br.read_exact(&mut buffer)?;
            csr.nodes[i] = usize::from_ne_bytes(buffer);
        }

        for i in 0..n_edges {
            br.read_exact(&mut buffer)?;
            csr.edges[i] = usize::from_ne_bytes(buffer);
        }

        return Ok(csr);
    }


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