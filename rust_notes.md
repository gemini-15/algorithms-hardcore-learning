# Rust tips & tricks

## Basic operations

- `(x >> n) & 1` retrieves the `n`th bit 
- `()
- euclidian division can be done with `div_rem` function from `num_integer`

## Array operations
- `let v: Vec<i32> = vec![1; N];` initializes a vector of 1 and length N  
- Instanciating a range array vector : `let range: Vec<u32> = (A..B).collect();`
- we can retrieve the index and value with enumeration with a slice : `for (i, &radius) in a.iter().enumerate()` with `a : &[i32]`


## Structures usage
- Hashset: [Hashset doc](https://doc.rust-lang.org/std/collections/struct.HashSet.html). Hashset can be used similarly to the `set()` function in python. Defining the unicity of an array. 
- We can transform a vector to a hashset by : `let hashset_vec : HashSet<i32> = HashSet::from_iter(&vector[..]).iter().cloned()`.

## String manipulation
- Strings [https://doc.rust-lang.org/rust-by-example/std/str.html?highlight=strings#strings](https://doc.rust-lang.org/rust-by-example/std/str.html?highlight=strings#strings). 
There is two types of strings in Rust: `String` and `&str`. 
`String` and `&str` are vector of bytes (`Vec[u8]`) and a slice (`&[u8]`) that are valid UTF-8 sequences. 

## Best pratices 
- Casting from `usize` (which can be a return to the len method) is best used with `try_into()`.
