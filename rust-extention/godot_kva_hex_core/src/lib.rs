use godot::prelude::*;
use kva_hex_core::Hex;
use kva_hex_core::spiral;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct SpiralHexGrid {
    data:Vec<HexContent>
}

#[godot_api]
impl IRefCounted for SpiralHexGrid {
    fn init(base: godot::obj::Base < Self::Base >) -> Self {
        //std::unimplemented !()
        Self {data:vec![]}
    }

    fn to_string(&self) -> godot::builtin::GString {
        std::unimplemented !()
    }

    fn on_notification(&mut self, what: godot::classes::notify::ObjectNotification) {
        std::unimplemented !()
    }

    fn get_property(&self, property: StringName) -> Option< Variant > {
        std::unimplemented !()
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        std::unimplemented !()
    }

    fn get_property_list(&mut self) -> Vec< godot::meta::PropertyInfo > {
        std::unimplemented !()
    }

    fn validate_property(&self, property: &mut godot::meta::PropertyInfo) {
        std::unimplemented !()
    }

    fn property_get_revert(&self, property: StringName) -> Option< Variant > {
        std::unimplemented !()
    }
}
#[godot_api]
impl SpiralHexGrid {
    #[func]
    pub fn testDrawHex(&mut self, count:i32, flat:bool) ->PackedVector3Array {
        if count <= 0 {
            panic!("Can't draw imaginary hexagons");
        }
        const FLAT_UP_CORNERS: [(f32, f32); 6] = [
            (1.000, 0.000),      // 0°: (1, 0)
            (0.500, 0.866),      // 60°: (0.5, √3/2)
            (-0.500, 0.866),     // 120°: (-0.5, √3/2)
            (-1.000, 0.000),     // 180°: (-1, 0)
            (-0.500, -0.866),    // 240°: (-0.5, -√3/2)
            (0.500, -0.866),     // 300°: (0.5, -√3/2)
        ];
        const POINTY_UP_CORNERS: [(f32, f32); 6] = [
            (0.866, 0.500),      // 30°: (√3/2, 1/2)
            (0.000, 1.000),      // 90°: (0, 1)
            (-0.866, 0.500),     // 150°: (-√3/2, 1/2)
            (-0.866, -0.500),    // 210°: (-√3/2, -1/2)
            (0.000, -1.000),     // 270°: (0, -1)
            (0.866, -0.500),     // 330°: (√3/2, -1/2)
        ];
        let mut ret = PackedVector3Array::new();
        ret.resize((7 * count) as usize);
        for i in 0..count {
            let hex: Hex<i32> = spiral::spiral_index_to_hex(i as usize);
            let (x, y) = hex.to_xy(flat);
            ret[i as usize * 7] = Vector3::new(x, 0.0, y);
            for j in 0..6 { 
                let j = j as usize;
                let i = i as usize;
                ret[i*7+j+1] = if flat {
                    Vector3::new(
                        FLAT_UP_CORNERS[j].0 + x,
                        0.0,
                        FLAT_UP_CORNERS[j].1 + y
                    )
                } else {
                    Vector3::new(
                        POINTY_UP_CORNERS[j].0 + x,
                        0.0,
                        POINTY_UP_CORNERS[j].1 + y
                    )
                }
            }
        }


        return ret;

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
