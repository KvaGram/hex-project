use std::cmp;
use std::u8;

use godot::classes::audio_stream_wav::Format;
use godot::classes::Image;
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
}

fn get_height_by_sample(x0:i32, y0:i32, x1:i32, y1:i32, w:i32, data: &PackedByteArray) ->u8 {
    let mut index:usize = 0;
    if y1-y0 < 1 || x1-x0 < 1 {
        index = ((y0 * w + x0) * 3) as usize;
        return data.get(index).unwrap_or(0);
    }
    let mut sum:u32 = 0;//map.get_pixel(x0, y0).r8().into();
    let count:u32 = ((x1-x0) * (y1-y0)+1).try_into().unwrap_or(u32::MAX);
    for y in y0..y1{
        index = ((y * w + x0) * 3) as usize;
        for _ in x0..x1 {
            sum += data.get(index).unwrap_or(0) as u32;
            index += 3;
        }
    }
    (sum/count).try_into().unwrap_or(u8::MAX)
}
fn get_height_by_sample2(x0:i32, y0:i32, x1:i32, y1:i32, map: &Image) ->u8 {
    //godot_print!("get_height_by_sample {x0}, {y0}, {x1}, {y1}", );
    let mut sum:u32 = map.get_pixel(x0, y0).r8().into();
    let count:u32 = ((x1-x0) * (y1-y0)+1).try_into().unwrap_or(u32::MAX);
    for x in x0..x1 {
        for y in y0..y1 {
            sum += u32::from(map.get_pixel(x, y).r8());
        }
    }
    (sum/count).try_into().unwrap_or(u8::MAX)
}
#[godot_api]
impl SpiralHexGrid {
    #[func]
    pub fn from_hightmap(&mut self, layers:i32, map:Gd<Image>) {
        let num_chan = {
            let ord = map.detect_used_channels().ord();
            if ord == 0 {1}
            else if ord == 1 || ord == 3 {2}
            else if ord == 4 {3}
            else if ord == 5 {4}
            else {3}
        };
            
        let data: PackedByteArray = map.get_data();
        let size = (3 * (layers+1) * layers + 1) as usize;
        let (width, height) = (map.get_width(), map.get_height());
        //sample sizes. How big a rectangle (by radius) does each tile need to sample? For average height.
        let x_s_size:i32 = map.get_width()  / (layers*2);
        let y_s_size:i32 = map.get_height() / (layers*2);
        let scale_x = map.get_width() as f32 / (2 * layers) as f32;
        let scale_y = map.get_height() as f32 / (2 * layers) as f32;
        //godot_print!("size {size}, x_s_size {x_s_size}, y_s_size {y_s_size}");

        self.data.resize(size, HexContent { height: 0 });
        for i in 0..size{
            let h = spiral::spiral_index_to_hex(i);
            let mut x;
            let mut y;
            (x, y) = h.to_xy(true);
            x = x * scale_x + (map.get_width()/2) as f32;
            y = y * scale_y + (map.get_height()/2) as f32;
            let (x, y) = (x.round() as i32, y.round() as i32);
            //godot_print!("hex at x{x}, y{y}");
            let (x0, y0) = (cmp::max(x-x_s_size, 0), cmp::max(y-y_s_size, 0));
            let (x1, y1) = (cmp::min(x+x_s_size, map.get_width()), cmp::min(y+y_s_size, map.get_height()));
            self.data[i].height = get_height_by_sample(x0, y0, x1, y1, width, &data);
            //self.data[i].height = get_height_by_sample2(x0, y0, x1, y1, &map);
        }
    }
    #[func]
    pub fn get_heightdata_at(&self, index:i32)->u8{
        if index < 0 || index >= self.data.len() as i32 {
            return 0;
        }
        self.data.get(index as usize).unwrap().height
    }
    #[func]
    pub fn get_heightdata(&self)->PackedByteArray{
        let mut ret = PackedByteArray::new();
        ret.resize(self.data.len());
        for i in 0..self.data.len(){
            ret[i] = self.data[i].height;
        }
        return ret;
    }


    #[func]
    pub fn test_draw_hex(&self, flat:bool) -> PackedVector3Array {
        let count = self.data.len();
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
            let h = self.data[i].height;
            let hex: Hex<i32> = spiral::spiral_index_to_hex(i as usize);
            let (x, y) = hex.to_xy(flat);
            ret[i as usize * 7] = Vector3::new(x, h as f32 / 100f32, y);
            for j in 0..6 { 
                let j = j as usize;
                let i = i as usize;
                ret[i*7+j+1] = if flat {
                    Vector3::new(
                        FLAT_UP_CORNERS[j].0 + x,
                        h as f32 / 100f32,
                        FLAT_UP_CORNERS[j].1 + y
                    )
                } else {
                    Vector3::new(
                        POINTY_UP_CORNERS[j].0 + x,
                        h as f32 / 100f32,
                        POINTY_UP_CORNERS[j].1 + y
                    )
                }
            }
        }
        return ret;

    }
}

#[derive(Clone)]
struct HexContent{
    height:u8
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
