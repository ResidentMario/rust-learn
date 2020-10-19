use sorts;

fn main() {
    let vec = &mut vec![5, 4, 3, 2, 1];
    // sorts::bubble_sort(vec);
    // println!("{:?}", sorts::bubble_sort(&mut vec![5, 4, 2, 1, 3]));
    // println!("{:?}", sorts::bubble_sort(&mut vec![1, 2, 3, 4, 5]));
    // println!("{:?}", sorts::bubble_sort(&mut vec![]));
    // sorts::selection_sort(vec);
    // println!("{:?}", sorts::selection_sort(&mut vec![5, 4, 2, 1, 3]));
    // println!("{:?}", sorts::selection_sort(&mut vec![1, 2, 3, 4, 5]));
    // println!("{:?}", sorts::selection_sort(&mut vec![]));
    let vec = sorts::counting_sort(vec);
    // sorts::insertion_sort(vec);
    // println!("{:?}", sorts::quicksort(vec![5, 4, 3, 2, 1]));
    // println!("{:?}", sorts::mergesort(vec![5, 4, 3, 2, 1]));
    println!("{:?}", vec);
}
