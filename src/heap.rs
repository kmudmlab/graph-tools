pub fn sort_heap_indices(heap : &mut [usize], values : &[usize]) {
    
    for size in (1..heap.len()).rev() {
        (heap[0], heap[size]) = (heap[size], heap[0]);
        siftdown_indices(0, heap, values, size);
    }
}

pub fn heapify_indices(heap : &mut[usize], values : &[usize]){
    for idx in (0..heap.len()/2).rev() {
        siftdown_indices(idx, heap, values, heap.len());
    }
}

pub fn heap_push_indices(val: usize, heap : &mut[usize], values : &[usize]){
    if values[heap[0]] < values[val] {
        heap[0] = val;
        siftdown_indices(0, heap, values, heap.len());
    }
}

#[inline]
fn siftdown_indices(idx: usize, heap: &mut[usize], values: &[usize], size : usize){
    let mut idx = idx;
    loop {
        let left_idx = idx * 2 + 1;
        let right_idx = left_idx + 1;

        if left_idx >= size { break; }
        else if right_idx >= size || values[heap[left_idx]] <= values[heap[right_idx]] {
            if values[heap[left_idx]] < values[heap[idx]] {
                (heap[idx], heap[left_idx]) = (heap[left_idx], heap[idx]);
                idx = left_idx;
            }
            else { break; }
        }
        else {
            if values[heap[right_idx]] < values[heap[idx]] {
                (heap[idx], heap[right_idx]) = (heap[right_idx], heap[idx]);
                idx = right_idx;
            }
            else { break; }
        }
    }
}






/* The code below has not been tested */

pub fn heap_push(val: usize, heap : &mut[usize]){
    if heap[0] < val{
        heap[0] = val;
        siftdown(0, heap);
    }
}

pub fn heapify(heap : &mut[usize]){
    let size = heap.len();
    for idx in (0..size/2).rev() {
        siftdown(idx, heap);
    }
}

#[inline]
fn siftdown(idx: usize, heap: &mut[usize]){
    let size = heap.len();
    let mut idx = idx;
    loop {
        let left_idx = idx * 2 + 1;
        let right_idx = left_idx + 1;
        let p = heap[idx];

        if left_idx >= size { break; }
        else if right_idx >= size || heap[left_idx] <= heap[right_idx] {
            if heap[left_idx] < p {
                heap[idx] = heap[left_idx];
                heap[left_idx] = p;
                idx = left_idx;
            }
            else { break; }
        }
        else {
            if heap[right_idx] < p {
                heap[idx] = heap[right_idx];
                heap[right_idx] = p;
                idx = right_idx;
            }
            else { break; }
        }
    }
}