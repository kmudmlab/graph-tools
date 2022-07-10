use std::collections::HashMap;

fn gcd(mut a: usize,mut b: usize) -> usize{
    if a < b {
        (a, b) = (b, a);
    }
    
    if b == 0 {
        return a;
    }
    
    return gcd(b, a % b);

}

fn mul_mod(mut x: usize, mut y: usize, m: usize) -> usize {

    let mut res = 0usize;
    x = x % m;

    while y > 0 {
        if y % 2 == 1 {
            res = (res + x) % m;
        }
        x = (x << 1) % m;
        y = y >> 1;
    }

    return res % m;
}

pub fn scaling(edges: &[(usize, usize)], n_nodes: usize, n_pixels: usize) -> Vec<(usize, usize, usize)>{
    let div = gcd(n_nodes, n_pixels);
    let n = n_nodes / div;
    let p = n_pixels / div;
    
    let mut edges_cnt: HashMap<(usize, usize), usize> = HashMap::new();
    
    for (u, v) in edges {
        let new_u = *u - mul_mod(*u, p, n) / p;
        let new_v = *v - mul_mod(*v, p, n) / p;
        let new_edge = (new_u, new_v);
        let cnt = edges_cnt.entry(new_edge).or_insert(0);
        *cnt += 1;
    }

    let new_edges: Vec<(usize, usize, usize)> = edges_cnt.iter().map(|((u, v), c)| (*u, *v, *c)).collect();
    
    return new_edges;
}