# echo-server-rust
Echo Server implementation in Rust

Inspired by [Coding Challenge #101](https://codingchallenges.substack.com/p/coding-challenge-101-echo-server) by John Crickett

## Setup
1. Clone the repository: `git clone https://github.com/yourusername/echo-server-rust.git`
2. Navigate to the project directory: `cd echo-server-rust`
3. Install dependencies: `cargo build`
4. Run the server: 
  - TCP mode: `cargo run` 
  - UDP mode: `cargo run -- --udp`
5. Test the server using `netcat`: `nc localhost 7`

# References
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [RFC 862: Echo Protocol](https://www.rfc-editor.org/rfc/rfc862)