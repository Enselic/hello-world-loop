//! Instead of using an external crate we could manually override `on_broken_pipe`:
//!
//! ```
//! #![feature(on_broken_pipe)]
//!
//! #[std::io::on_broken_pipe]
//! fn on_broken_pipe() -> std::io::OnBrokenPipe {
//!     std::io::OnBrokenPipe::Kill
//! }
//! ```

use broken_pipe_kills;

fn main() {
    loop {
        println!("Hello, world!");
    }
}
