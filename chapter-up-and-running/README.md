# Chapter Up and Running

This project recreates the example in the book [Akka in Action]
with [Bastion-rs].

The original example code is also on [github](https://github.com/RayRoestenburg/akka-in-action/tree/master/chapter-up-and-running).

## Build and run

```sh
cargo run
```

### Test with cURL

```sh
curl -i -X POST -d '{"tickets": 100}' http://localhost:8000/events/RUST
```

[Akka in Action]: https://www.manning.com/books/akka-in-action
[Bastion-rs]: https://bastion.rs/
