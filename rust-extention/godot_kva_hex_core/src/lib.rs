use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct SpiralHexGrid {
    data:Vec<HexContent>
}

impl SpiralHexGrid {
    fn new() -> Self {
        Self {  }
    }
}
struct HexContent{

}

#[gdextension]
unsafe impl ExtensionLibrary for SpiralHexGrid {}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
