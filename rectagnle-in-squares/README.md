The drawing below gives an idea of how to cut a given "true" rectangle into squares ("true" rectangle meaning that the two dimensions are different).

![alternative text](https://i.imgur.com/lk5vJ7sm.jpg)

Can you translate this drawing into an algorithm?

You will be given two dimensions

1.  a positive integer length (parameter named `lng`)
2.  a positive integer width (parameter named `wdth`)

You will return an array or a string with the size of each of the squares.

```rust
sq_in_rect(5, 3) // should return Some(vec![3, 2, 1, 1])
sq_in_rect(3, 5) // should return Some(vec![3, 2, 1, 1])
sq_in_rect(1, 1) // should return None
```

When the initial parameters are so that `lng` == `wdth`, the solution `[lng]` would be the most obvious but in our case, return `None`.
