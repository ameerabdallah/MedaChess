<div align="center">
  <h3>MedaChess (Not yet functional)</h3>
</div>

## Overview

MedaChess is a chess engine that doesn't like to lose but feels bad winning. It will try to force draws even if it detects winning positions. It accomplishes this by following the path that keeps the position as equal as possible.

## Compiling MedaChess
* To build a release version of MedaChess, run,
`cargo build --release`

* If you would just like to run it then run,
`cargo run`

## Tests
* To run tests, run,
`cargo test`