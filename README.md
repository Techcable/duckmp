duckmp
======
Fast, liberally licensed, multiple precision arithmetic.

## Features
- Written in Rust
    - Implements [num-traits](https://docs.rs/num-traits/)
    - Assembly implementations for certian algorithims (x86-64 only)
- Liberally licensed (MIT OR Apache 2.0)
- Complete rewrite, avoiding taking anything from [GMP](https://gmplib.org/)
    - I'm not even allowing myself to look at GMP, to avoid any copyright issues
    - However, I am allowing myself to (sometimes) look at [libtommath](https://github.com/libtom/libtommath)
      since that is liberally licensed
    - Of course, I can also look at Wikipedia ;)
