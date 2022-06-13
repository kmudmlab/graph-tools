use std::cmp::Ordering;

use crate::csbv::CSBV;

pub fn count_from_csbv(graph: &CSBV) -> usize{

    let mut cnt = 0usize;

    for u in 0..graph.n_nodes() {
        println!("u = {}", u);
        for v in graph.neighbor_iter(u) {
            cnt += count_intersect(u, v, graph);
        }
    }
    return cnt;
}

pub fn count_intersect(u: usize, v: usize, graph: &CSBV) -> usize{
    let mut cnt = 0usize;
    
    let mut uiter = graph.block_iter(u);
    let mut viter = graph.block_iter(v);

    let (mut un, mut un_bits) = match uiter.next() {
        Some(x) => x,
        None => return cnt
    };

    let (mut vn, mut vn_bits) = match viter.next() {
        Some(x) => x,
        None => return cnt
    };

    loop{
        if un < vn {
            (un, un_bits) = match uiter.next() {
                Some(x) => x,
                None => return cnt
            }
        }
        else if un > vn {
            (vn, vn_bits) = match viter.next() {
                Some(x) => x,
                None => return cnt
            }
        }
        else {
            cnt += (un_bits & vn_bits).count_ones() as usize;

                (un, un_bits) = match uiter.next() {
                    Some(x) => x,
                    None => return cnt
                };
                (vn, vn_bits) = match viter.next() {
                    Some(x) => x,
                    None => return cnt
                };
        }
    }

}

pub fn count_total(adj: Vec<Vec<usize>>) -> usize{
    let mut cnt = 0usize;
    for adj_u in adj.iter(){
        for v in adj_u{
            cnt += count_intersect_btw_sorted(adj_u, &adj[*v]);
        }
    }
    return cnt;
}

fn count_intersect_btw_sorted(adj_u: &Vec<usize>, adj_v: &Vec<usize>) -> usize{
    let mut iter_u = adj_u.iter();
    let mut iter_v = adj_v.iter();
    let mut un = match iter_u.next() { Some(un) => un, None => return 0 };
    let mut vn = match iter_v.next() { Some(vn) => vn, None => return 0 };
    
    let mut cnt = 0usize;

    loop {
        match un.cmp(vn){
            Ordering::Less => {
                un = match iter_u.next() { Some(un) => un, None => return cnt };
            }
            Ordering::Greater => {
                vn = match iter_v.next() { Some(vn) => vn, None => return cnt };
            }
            Ordering::Equal => {
                cnt += 1;
                un = match iter_u.next() { Some(un) => un, None => return cnt };
                vn = match iter_v.next() { Some(vn) => vn, None => return cnt };
            }
        };
    }
}