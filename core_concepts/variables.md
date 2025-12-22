# Variables
1. By default, variables are immutable.
```
fn main() {
    let x = 5;
    x = 6; // This is a compile error.
           // Cannot assign another value.
}
```
<br />
 
2. Using the keyword ```mut``` makes the variable mutable.
```
fn main() {
    let mut x = 5;
    x = 6;
}
```
