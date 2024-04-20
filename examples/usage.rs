use sorting_lib::sorting::quick_sort;
use sorting_lib::sorting::selection_sort;
use sorting_lib::sorting::insertion_sort;   
use sorting_lib::sorting::merge_sort;

fn main() {
    let mut vec = vec![2, 5, 8, 14, 666, 234, 1113, 13, 3, 4];
    quick_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 5, 8, 14, 666, 234, 1113, 13, 3, 4];
    selection_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 5, 8, 14, 666, 234, 1113, 13, 3, 4];
    insertion_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 5, 8, 14, 666, 234, 1113, 13, 3, 4];
    merge_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

}
