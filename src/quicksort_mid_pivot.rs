fn arrange_in_place(data: &mut [i32], _pivot: &mut usize) {
    let ref_d = data;
    let p_idx = ref_d.len() / 2;
    let p = ref_d[p_idx];
    //
    // u32 vs usize, i32 vs isize
    //
    let mut i: usize = 0;

    for j in 0 .. ref_d.len() {
        if ref_d[j] >= p {
            continue;
        }
        
        ref_d.swap(i, j);
        i += 1; 
        if i == p_idx && p_idx < (ref_d.len() - 1) {
            i += 1;
        }
    }
    //
    // If final i is after (pivot + 1), that
    //
    if i != 0 && (i - 1) == p_idx {
    } else if  i > (p_idx + 1){
        i -= 1;
        ref_d.swap(i, p_idx);
    } else {
        ref_d.swap(i, p_idx);
    }
    *_pivot = i;
}

fn quicksort (data: &mut [i32]) {
    let ref_d = data;
    let size  = ref_d.len();
    let mut p_idx = 0;
    if size == 0 {
        return;
    }
    arrange_in_place(ref_d, &mut p_idx);
    //
    // ending_idex of slice is one more than the last position in the slice.
    //
    quicksort(&mut ref_d[0 .. p_idx]);
    if (p_idx + 1) < size {
        quicksort(&mut ref_d[(p_idx + 1) .. size]);
    }
}

pub fn run () {
    let mut data = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    quicksort (&mut data);
    println!("{:?}", data);
}