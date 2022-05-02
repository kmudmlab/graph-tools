pub struct UnionFind{
    n_components: usize,
    pub parent: Vec<usize>
}

impl UnionFind{
    pub fn new(n : usize) -> Self {
        Self {
            n_components: n,
            parent: (0..n).collect(),
        }
    }

    pub fn union(&mut self, mut u: usize, mut v: usize){
        let p = &mut self.parent;
        
        while p[u] != p[v] {
            if p[u] > p[v] {
                if u == p[u] {
                    p[u] = p[v];
                    self.n_components-=1;
                    return;
                }
                let z = p[u];
                p[u] = p[v];
                u = z;
            }
            else {
                if v == p[v] {
                    p[v] = p[u];
                    self.n_components-=1;
                    return;
                }
                let z = p[v];
                p[v] = p[u];
                v = z;
            }
        }
    }

    pub fn find(&mut self, mut u: usize) -> usize {
        let mut p = self.parent[u];
        while p != u {
            self.parent[u] = self.parent[p];
            u = p;
            p = self.parent[p];
        }

        return u;
    }

    pub fn n_components(& self) -> usize {
        return self.n_components;
    }

    pub fn n_nodes(&self) -> usize{
        return self.parent.len();
    }
}