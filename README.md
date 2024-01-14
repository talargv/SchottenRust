# UCT for Schotten Totten _(in Rust)_
Similarly to the [other Python project](https://github.com/talargv/Schotten-Totten-UCT.git), this project aims to implement (more efficiently) the UCT algorithm for the Schotten Totten game.

## Features
- [x] A basic CLI impelemntation of the game of a manually controlled player vs a random player.
       _Can be played by running ```cargo run```_.
- [ ] A more efficient implementation of the game by reducing computations needed to determine legal stone claims - _in progress, by the ```jobs``` module_.
- [ ] A basic implementation of a UCT - based bot against a random player. _Currently testing different tree structures suitable for the algorithm (trees are a _very_ non-trivial structure in Rust. check out this [book teaching _all of Rust_ by implementing Linked Lists](https://rust-unofficial.github.io/too-many-lists/) to understand why)_.
- [ ] Better simulations and opponent modeling by eliminating some _bad_ moves.
- [ ] Comparing between opponent modeling and determinization when implementing the algorithm.
