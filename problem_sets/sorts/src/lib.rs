use std::collections::{BTreeMap, HashMap};

// When the function does not mutate the original input array, it's considered good form to use
// the slice as the input signature type, as this is more generic than using a vector type.
pub fn bubble_sort<T: Ord>(vec: &mut [T]) {
    let mut unsorted_len = vec.len();
    while unsorted_len > 1 {
        let mut last_swap = 0;
        for i in 1..unsorted_len {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                last_swap = i;
            }
        }
        unsorted_len = last_swap;
    }

    // Old implementation.
    // if vec.is_empty() {
    //     return;
    // }

    // let mut swap_seen = true;
    // while swap_seen {
    //     swap_seen = false;
    //     for mut i in 0..(vec.len() - 1) {
    //         while (i < (vec.len() - 1)) && (vec[i] > vec[i + 1]) {
    //             vec.swap(i, i + 1);
    //             swap_seen = true;
    //             i += 1;
    //         }
    //     }
    // }
}

pub fn selection_sort<T: Ord>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut smallest_idx = i;
        for j in (i + 1)..(vec.len()) {
            if vec[j] < vec[smallest_idx] {
                smallest_idx = j;
            }
        }
        vec.swap(i, smallest_idx);
    }
}

pub fn counting_sort(vec: &[u32]) -> Vec<u32> {
    // A type alias! Done to "create semantic difference in type" between u32 values that are
    // counts and u32 values that are map keys.
    type Count = u32;
    
    let mut counts: BTreeMap<_, Count> = BTreeMap::new();
    for val in vec {
        *counts.entry(*val).or_default() += 1;
    }

    let mut sorted: Vec<Count> = vec![];
    for (digit, count) in counts {
        sorted.extend(std::iter::repeat(digit).take(count as _));
    }
    sorted
}

pub fn counting_sort_old(vec: &[u32]) -> Vec<u32> {
    // The type matters. HashMap implements its methods using traits, and if you don't pick
    // the right types the traits won't apply and the methods won't show up.
    let mut counts: HashMap<&u32, u32> = HashMap::new();

    for val in vec {
        // Interesting usage note. This doesn't work:
        //
        // match counts.get(val) {
        //     Some(n_val) => counts.insert(val, n_val + 1),
        //     None => counts.insert(val, 1),
        // };
        //
        // Why not?
        // counts.get(val) is an immutable borrow of the counts hashmap.
        // counts.insert(val) is a mutable borrow of the counts hashmap.
        // This violates the constraint that only one mutable or any number of immutable
        // borrows may be live at a time. However, it only throws a warning, not an error, for
        // some reason. Ref:
        // https://discord.com/channels/442252698964721669/448238009733742612/763950019681583152
        //
        // That brings us to this working code. The "entry API" is specifically designed to avoid
        // this problem. In general, many APIs in Rust are designed around such concerns.
        *counts.entry(val).or_default() += 1;
    }

    let mut sorted: Vec<u32> = vec![];

    for digit in 0..=9u32 {
        let digit_ref = &digit;
        let digit_count = counts.get(digit_ref);

        match digit_count {
            Some(count) => {
                // Formerly:
                //
                // for _ in 0..(*count as i32) {
                //     sorted.push(digit);
                // }
                //
                // This repeat is :eyes:.
                sorted.extend(std::iter::repeat(digit).take(*count as usize));
            }
            None => (),
        }
    }

    sorted
}

pub fn insertion_sort(vec: &mut [u32]) {
    // usize is Rust's "architecture-dependent integer size". It is u32 on 32-bit systems and
    // u64 on 64-bit systems. usize is used in certain places in Rust lang where this low-level
    // detail matters, e.g. if indexing into memory. It's used to represent array sizes I guess
    // because array length maximum is the architecture's word size.
    for i in 1..(vec.len()) {
        for j in 0..i {
            if vec[j] > vec[i] {
                let (a, b) = (vec[i], vec[j]);
                vec[j] = a;
                vec[i] = b;
            }
        }
    }
}

pub fn quicksort(vec: Vec<u32>) -> Vec<u32> {
    if vec.len() <= 1 {
        return vec;
    }

    let pivot_idx = vec.len() / 2 as usize;
    let pivot_val = vec[pivot_idx];
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for val in [&vec[..pivot_idx], &vec[(pivot_idx + 1)..]].concat() {
        if val < pivot_val {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    let mut result = quicksort(left);
    result.push(pivot_val);
    let mut right = quicksort(right);
    result.append(&mut right);
    result
}

pub fn mergesort(vec: Vec<u32>) -> Vec<u32> {
    // A comment was made that you can leave the type on left and right out. However, without the
    // type information it throws a cannot infer type error, so you have to leave it in (at least
    // as written).
    let join = |left: Vec<u32>, right: Vec<u32>| -> Vec<u32> {
        let (mut j, mut k) = (0, 0);
        let mut result: Vec<u32> = vec![];
        while j < left.len() && k < right.len() {
            if left[j] < right[k] {
                result.push(left[j]);
                j += 1;
            } else {
                result.push(right[k]);
                k += 1;
            }
        }
        while j < left.len() {
            result.push(left[j]);
            j += 1;
        }
        while k < right.len() {
            result.push(right[k]);
            k += 1;
        }

        result
    };

    if vec.len() <= 1 {
        return vec;
    }
    // Nit: eliminate this additional base case.
    if vec.len() == 2 {
        if vec[0] < vec[1] {
            return vec;
        } else {
            return vec![vec[1], vec[0]];
        }
    }

    // TODO: how do I eliminate this copy without changing the function signature from Vec<u32>
    // to &[u32]?
    let pivot = vec.len() / 2 as usize;
    let mut left: Vec<u32> = Vec::new();
    left.extend_from_slice(&vec[..pivot]);
    let mut right: Vec<u32> = Vec::new();
    right.extend_from_slice(&vec[pivot..]);

    let left = mergesort(left);
    let right = mergesort(right);

    join(left, right)
}

// TODO: radix sort
