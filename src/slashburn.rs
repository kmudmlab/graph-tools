use super::union_find::UnionFind;
use super::heap;
use std::collections::{HashSet, HashMap};

pub fn slashburn(edges: &[(usize, usize)], n_nodes: usize, k: usize) -> Vec<usize>{

    let mut remain_edges : Vec<(usize, usize)> = edges.iter().copied().collect();
    let mut remain_nodes : HashSet<usize> = (0..n_nodes).collect();
    let mut hubs = Vec::<usize>::new();
    let mut spokes = Vec::<usize>::new();

    let mut degrees = vec![0; n_nodes].into_boxed_slice();

    let mut uf = UnionFind::new(n_nodes);

    for (u, v) in edges{
        degrees[*u] += 1;
        degrees[*v] += 1;
    }

    loop{
        eprintln!("remain edges: {}, remain_nodes: {}", remain_edges.len(), remain_nodes.len());

        let hubs_remain = topk(k, &remain_nodes, &degrees);
        let hubset_remain : HashSet<usize> = hubs_remain.iter().copied().collect();

        remain_nodes.retain(|u| !hubset_remain.contains(u));
        
        remain_edges.retain(|(u, v)| {
            if remain_nodes.contains(u) && remain_nodes.contains(v) {
                return true;
            }
            else{
                degrees[*u] -= 1;
                degrees[*v] -= 1;
                return false;
            }
        });

        hubs.extend(hubs_remain.iter());

        if remain_nodes.is_empty() {
            break;
        }
        
        let spokes_remain = find_and_remove_spokes(&remain_edges, &mut remain_nodes, &mut uf);
        
        spokes.extend(spokes_remain.iter().rev());

        if remain_nodes.is_empty() {
            break;
        }
    }

    hubs.extend(spokes.into_iter().rev());

    return hubs;

}

fn find_and_remove_spokes(edges: &[(usize, usize)], nodes: &mut HashSet<usize>, uf: &mut UnionFind) -> Vec<usize>{
    
    uf.reset(nodes.iter());

    for (u, v) in edges.iter() {
        uf.union(*u, *v);
    }

    let cc : Vec<usize> = nodes.iter().map(|u| uf.find(*u)).collect();
    
    let mut cc_sizes : HashMap<usize, usize> = HashMap::new();
    for c in &cc{
        *cc_sizes.entry(*c).or_insert(0) += 1;
    }

    let (gccid, gcccnt) = cc_sizes.into_iter().max_by(|(_, x), (_, y)| x.cmp(y)).expect("What the hell?");

    let mut spokes = Vec::with_capacity(nodes.len() - gcccnt);
    
    
    nodes.retain(|u| {
        if gccid == uf.find(*u) {
            true
        }
        else{
            spokes.push(*u);
            false
        }
    });

    spokes.sort_by(|a,b| uf.find(*a).cmp(&uf.find(*b)));
    
    return spokes;
}

fn topk(k: usize, keys: &HashSet<usize>, values: &[usize]) -> Vec<usize> {
    
    let k = if k < keys.len() { k } else { keys.len() }; 

    let mut key_iter = keys.iter();

    let mut heap : Vec<usize> = vec![0;k];
    for i in heap.iter_mut(){
        *i = *key_iter.next().unwrap();
    }

    heap::heapify_indices(&mut heap, &values);

    for x in key_iter{
        heap::heap_push_indices(*x, &mut heap, &values);
    }

    heap::sort_heap_indices(&mut heap, &values);

    return heap;
}