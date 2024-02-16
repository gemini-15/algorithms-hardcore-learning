# Rust tips & tricks

## Basic operations

- `(x >> n) & 1` retrieves the `n`th bit 
- `()
- euclidian division can be done with `div_rem` function from `num_integer`

## Array operations
- `let v: Vec<i32> = vec![1; N];` initializes a vector of 1 and length N  
- Instanciating a range array vector : `let range: Vec<u32> = (A..B).collect();`

## Structures usage
- Hashset: [Hashset doc](https://doc.rust-lang.org/std/collections/struct.HashSet.html). Hashset can be used similarly to the `set()` function in python. Defining the unicity of an array. 
- We can transform a vector to a hashset by : `let hashset_vec : HashSet<i32> = HashSet::from_iter(&vector[..]).iter().cloned()`.

## Best pratices 
- Casting from `usize` (which can be a return to the len method) is best used with `try_into()`.
