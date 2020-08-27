pub mod buffer;
pub mod color;
pub mod device;
pub mod shader_resources;

fn wrong_backend() -> ! {
    panic!("render backend invariant broken, you have multiple render backend activated!!!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
