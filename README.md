# Type-level bubble sort algorithm in Rust 

Read the article about the implementation: [link](https://dev.to/thedenisnikulin/type-level-bubble-sort-in-rust-part-1-3mcb)!

```rust
assert_type_eq!(
	BubbleSort<Cons<N3, Cons<N1, Cons<N2, Nil>>>>,
	Cons<N1, Cons<N2, Cons<N3, Nil>>>);
```
