#![feature(extern_item_impls)]
#![feature(on_broken_pipe)]

#[std::io::on_broken_pipe]
fn on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}

fn main() {
    loop {
        println!("Hello, world!");
    }
}
