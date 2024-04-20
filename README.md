# Sorting Library

This is a Rust library developed by Koshkarbay Yernar (SE-2205), offering a variety of sorting algorithms. The library supports Quick Sort, Merge Sort, Insertion Sort, and Selection Sort.

## Features

- **Quick Sort**
- **Merge Sort**
- **Insertion Sort**
- **Selection Sort**

## Installation

To include this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
sorting_lib = { git = "https://github.com/yourusername/sorting_lib.git" }
```

Usage 
```rust
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
```