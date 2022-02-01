This crate help you to measure time taken by a function or a block of code for "print" debugging purpose. It output in stdout (using `println`) the elapsed time.


# usage
```rust
#[macro_use]
extern crate elapsed_time;

fn main() {
  // measure time taken by a block
  #[elapsed_time::elapsed_block(block_name_1)]
  {
    // some piece of code in a function
  }
  my_func();
}

// measure time taken by a function
#[elapsed_time::elapsed]
fn my_func() -> String { }
```
Produce following logs:
```
block_name_1 tooks 793.515574ms
my_func tooks 28.855µs
```
