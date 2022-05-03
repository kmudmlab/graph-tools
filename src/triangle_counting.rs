use std::cmp::Ordering;

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