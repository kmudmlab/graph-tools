// Compressed Sparse Bit Vectors
pub struct CSBV{
    pub bit_blocks: Vec<usize>,
    pub block_ids: Vec<usize>,
    pub ptrs: Vec<usize>
}

pub struct NeighborIterator<'a>{
    csbv: &'a CSBV,
    end: usize,
    ptr: usize,
    bits: usize
}

impl<'a> Iterator for NeighborIterator<'a> {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {

        const BLOCK_SIZE: usize = 64usize;
        
        if self.bits == 0 {
            if self.ptr + 1 >= self.end { return None; }

            self.ptr += 1;
            self.bits = self.csbv.bit_blocks[self.ptr];
        }

        let offset: usize = self.bits.trailing_zeros().try_into().unwrap();
        self.bits -= 1 << offset;
        
        return Some(self.csbv.block_ids[self.ptr] * BLOCK_SIZE + offset);
    }
}

impl CSBV{

    pub fn neighbor_iter(&self, u: usize) -> NeighborIterator{
        let ptr = self.ptrs[u];
        return NeighborIterator{
            csbv: self,
            end: self.ptrs[u+1],
            ptr,
            bits: self.bit_blocks[ptr]
        };
    }

    // edges are sorted, and has no duplicate. Nodes in each edge is ordered.
    pub fn from_sorted_edges(edges: &[(usize, usize)], n_nodes: usize) -> CSBV{
        let mut u_prev = usize::MAX;
        let mut bl_prev = usize::MAX;
        let block_size = 64usize;

        let mut n_blocks = 0usize;
        for (u, v) in edges {
            let bl = *v / block_size;
            
            if *u != u_prev {
                u_prev = *u;
                bl_prev = bl;
                n_blocks += 1;
            }
            else if bl != bl_prev {
                bl_prev = bl;
                n_blocks += 1;
            }
        }

        let mut csbv = CSBV{
            bit_blocks: vec![0usize; n_blocks],
            block_ids: vec![0usize; n_blocks],
            ptrs: vec![0usize; n_nodes+1],
        };

        u_prev = usize::MAX;
        bl_prev = usize::MAX;
        
        let mut bi = 0usize;
        for (u, v) in edges {
            let bl = *v / block_size;

            if *u != u_prev {
                u_prev = *u;
                bl_prev = bl;
                csbv.bit_blocks[bi] = 1usize << (*v % block_size);
                csbv.block_ids[bi] = bl;
                bi += 1;
                csbv.ptrs[*u] += 1; // compute degrees
            }
            else if bl != bl_prev {
                bl_prev = bl;
                csbv.bit_blocks[bi] = 1usize << (*v % block_size);
                csbv.block_ids[bi] = bl;
                bi += 1;
                csbv.ptrs[*u] += 1;
            }
            else{
                csbv.bit_blocks[bi-1] |= 1usize << (*v % block_size);
            }
            
        }

        for i in 0..n_nodes {
            csbv.ptrs[i+1] += csbv.ptrs[i];
        }
        for i in (0..n_nodes).rev() {
            csbv.ptrs[i+1] = csbv.ptrs[i];
        }
        csbv.ptrs[0] = 0;

        return csbv;
    }
}