use std::{fs::File, io::{Read, Write}};
use std::io::{BufWriter, BufReader};
use std::io::Result;

// Compressed Sparse Bit Vectors
pub struct CSBV{
    pub bit_blocks: Vec<usize>,
    pub block_ids: Vec<usize>,
    pub ptrs: Vec<usize>
}

impl CSBV{

    pub fn n_nodes(&self) -> usize{
        return self.ptrs.len() - 1;
    }

    pub fn dump(&self, filepath: &str) -> Result<()>{
        let mut bw = BufWriter::new(File::create(filepath)?);

        // write n_nodes+1, n_blocks
        bw.write_all(&self.ptrs.len().to_ne_bytes())?;
        bw.write_all(&self.block_ids.len().to_ne_bytes())?;
        
        // write data
        for ptr in &self.ptrs {
            bw.write_all(&ptr.to_ne_bytes())?;
        }
        for bid in &self.block_ids {
            bw.write_all(&bid.to_ne_bytes())?;
        }
        for bb in &self.bit_blocks {
            bw.write_all(&bb.to_ne_bytes())?;
        }

        bw.flush()?;

        return Ok(());
    }

    pub fn load(filepath: &str) -> Result<CSBV>{

        let mut br = BufReader::new(File::open(filepath)?);
        // read n_nodes+1, n_blocks
        let mut buffer = [0u8; std::mem::size_of::<usize>()];
        br.read_exact(&mut buffer)?;
        let n_ptrs = usize::from_ne_bytes(buffer);
        br.read_exact(&mut buffer)?;
        let n_blocks = usize::from_ne_bytes(buffer);

        let mut csbv = CSBV{
            bit_blocks: vec![0usize; n_blocks],
            block_ids: vec![0usize; n_blocks],
            ptrs: vec![0usize; n_ptrs]
        };

        for i in 0..n_ptrs {
            br.read_exact(&mut buffer)?;
            csbv.ptrs[i] = usize::from_ne_bytes(buffer);
        }

        for i in 0..n_blocks {
            br.read_exact(&mut buffer)?;
            csbv.block_ids[i] = usize::from_ne_bytes(buffer);
        }

        for i in 0..n_blocks {
            br.read_exact(&mut buffer)?;
            csbv.bit_blocks[i] = usize::from_ne_bytes(buffer);
        }

        return Ok(csbv);
    }

    pub fn block_iter(&self, u: usize) -> NeighborBlockIterator{
        return NeighborBlockIterator{
            csbv: self,
            end: self.ptrs[u+1],
            ptr: self.ptrs[u]
        }
    }

    pub fn neighbor_iter(&self, u: usize) -> NeighborIterator{
        let ptr = self.ptrs[u];
        
        return NeighborIterator{
            csbv: self,
            end: self.ptrs[u+1],
            ptr,
            bits: match self.bit_blocks.get(ptr) { Some(x) => *x, None => 0}
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
                csbv.ptrs[*u + 1] += 1; // compute degrees
            }
            else if bl != bl_prev {
                bl_prev = bl;
                csbv.bit_blocks[bi] = 1usize << (*v % block_size);
                csbv.block_ids[bi] = bl;
                bi += 1;
                csbv.ptrs[*u + 1] += 1;
            }
            else{
                csbv.bit_blocks[bi-1] |= 1usize << (*v % block_size);
            }
            
        }

        for i in 1..n_nodes {
            csbv.ptrs[i+1] += csbv.ptrs[i];
        }
        // for i in (0..n_nodes).rev() {
        //     csbv.ptrs[i+1] = csbv.ptrs[i];
        // }
        csbv.ptrs[0] = 0;

        return csbv;
    }
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

        let offset: usize = self.bits.trailing_zeros() as usize;
        self.bits -= 1 << offset;
        
        return Some(self.csbv.block_ids[self.ptr] * BLOCK_SIZE + offset);
    }
}


pub struct NeighborBlockIterator<'a>{
    csbv: &'a CSBV,
    end: usize,
    ptr: usize
}

impl<'a> Iterator for NeighborBlockIterator<'a>{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

        if self.ptr < self.end {
            self.ptr += 1;
            return Some( (self.csbv.block_ids[self.ptr-1], self.csbv.bit_blocks[self.ptr-1]) );
        }
        
        return None;
    }
}