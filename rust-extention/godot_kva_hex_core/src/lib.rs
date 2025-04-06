use std::cmp;
use std::u8;

use godot::classes::Image;
use godot::classes::Mesh;
use godot::prelude::*;
use kva_hex_core::Hex;
use kva_hex_core::spiral;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct SpiralHexGrid {
    data:Vec<HexContent>,
    layers:u8, //With a u8 restriction, there may be up to 195'841 tiles in a 255 layered spiral hex grid
    super_pos:Hex<i32>,
    origin:Hex<i32>
}

#[godot_api]
impl IRefCounted for SpiralHexGrid {
    fn init(_base: godot::obj::Base < Self::Base >) -> Self {
        //std::unimplemented !()
        Self {data:vec![], layers: 0, super_pos:Hex{q:0,r:0}, origin:Hex{q:0,r:0}}
    }
}



fn get_height_by_sample(x0:i32, y0:i32, x1:i32, y1:i32, width:i32, num_chan:i32, data: &PackedByteArray) ->u8 {
    let mut index:usize = 0;
    if y1-y0 < 1 || x1-x0 < 1 {
        index = ((y0 * width + x0) * num_chan) as usize;
        return data.get(index).unwrap_or(0);
    }
    else {
        let mut sum:u32 = 0;//map.get_pixel(x0, y0).r8().into();
        let count:u32 = ((x1-x0) * (y1-y0)).try_into().unwrap_or(u32::MAX);
        for y in y0..y1{
            index = ((y * width + x0) * num_chan) as usize;
            for _ in x0..x1 {
                sum += data.get(index).unwrap_or(0) as u32;
                index += num_chan as usize;
            }
        }
        (sum/count).try_into().unwrap_or(u8::MAX)
    }
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
    /// sets the posision of the super-hexagon, in relation to other super hexagons.
    /// This also updates the origin coordinates, which as used to convert local coordinates to global coordinates.
    #[func]
    pub fn set_super_pos(&mut self, super_pos_q:i32, super_pos_r:i32) {
        use kva_hex_core::direction::*;
        use spiral::*;

        self.super_pos.q = super_pos_q;
        self.super_pos.r = super_pos_r;

        let s_spiral:Spiral = Spiral::try_from(self.super_pos).expect("Spiral::try_from<Hex<i32>> should convert with no issues.");
        let dir_seg: Hex<i32> = get_dir(s_spiral.segment()).into();
        let dir_seg_p2: Hex<i32> = get_dir(s_spiral.segment()+2).into();
        let dir_seg_m2: Hex<i32> = get_dir(s_spiral.segment()-2).into();

        let layer_scale = self.layers as i32 * 2 + 1;
        let pos_scale = self.layers;

        self.origin = dir_seg * s_spiral.layer * layer_scale + 
        dir_seg_p2 * s_spiral.s_posision() * layer_scale + 
        dir_seg_m2 * self.layers * s_spiral.layer + 
        dir_seg * s_spiral.s_posision() * pos_scale;
    }
    #[func]
    pub fn from_hightmap(&mut self, layers:u8, map:Gd<Image>) {
        self.layers = layers;
        let num_chan = {
            let ord = map.detect_used_channels().ord();
            if ord == 0 {1}
            else if ord == 1 || ord == 3 {2}
            else if ord == 4 {3}
            else if ord == 5 {4}
            else {3}
        };
            
        let data: PackedByteArray = map.get_data();
        let size: usize = (3 * (layers+1) * layers + 1) as usize;
        let (width, _height) = (map.get_width(), map.get_height());
        //sample sizes. How big a rectangle (by radius) does each tile need to sample? For average height.
        let x_s_size:i32 = map.get_width()  / (layers*2) as i32;
        let y_s_size:i32 = map.get_height() / (layers*2) as i32;
        let scale_x: f32 = map.get_width() as f32 / (2 * layers) as f32;
        let scale_y: f32 = map.get_height() as f32 / (2 * layers) as f32;
        //godot_print!("size {size}, x_s_size {x_s_size}, y_s_size {y_s_size}");

        self.data.resize(size, HexContent { height: 0 });
        for i in 0..size{
            let h: Hex<i32> = spiral::spiral_index_to_hex(i);
            let mut x: f32;
            let mut y: f32;
            (x, y) = h.to_xy(true);
            x = x * scale_x + (map.get_width()/2) as f32;
            y = y * scale_y + (map.get_height()/2) as f32;
            let (x, y) = (x.round() as i32, y.round() as i32);
            //godot_print!("hex at x{x}, y{y}");
            let (x0, y0) = (cmp::max(x-x_s_size, 0), cmp::max(y-y_s_size, 0));
            let (x1, y1) = (cmp::min(x+x_s_size, map.get_width()), cmp::min(y+y_s_size, map.get_height()));
            //self.data[i].height = get_height_by_sample2(x0, y0, x1, y1, &map);
            self.data[i].height = get_height_by_sample(x0, y0, x1, y1, width, num_chan, &data);
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
    pub fn get_neighbors(target:Hex<i32>) -> Vec<Hex<i32>>
    {
        let mut neighbors: Vec<Option<Hex<i32>>> = Vec::new();
        neighbors.resize(6, None);
        return neighbors;

        /* func checkNeighbors(tile_layers:int, chunk_q:int, chunk_r:int, q:int, r:int):
	var chunk_s = -chunk_q - chunk_r
	var s = -q-r
	var neighbors:Array = []

	for d in 6:
		var n =HexUtil.get_dir(d).duplicate()
		var nq = n[0]+q
		var nr = n[1]+r
		var ns = n[2]+s
		var ncq = chunk_q
		var ncr = chunk_r
		var ncs = chunk_s
		##test out of bounds
		#1 find tile's global coords.
		#2 find tile's local coords in adjacent chunk
		#3 apply direction to get neighbour

		#If neighbor is within chunk borders, skip to next.
		if (abs(nq)>tile_layers or abs(nr)>tile_layers or abs(ns)>tile_layers):
			var n_global := HexUtil.chunk_to_global(PackedInt32Array([chunk_q, chunk_r, chunk_s]), tile_layers)
			n_global[0]+=nq; n_global[1]+=nr; n_global[2]+=ns
			var n_dir:int = -1
			match(d):
				0: n_dir= d if q <= 0 else d+1
				1: n_dir= d if s >= 0 else d+1
				2: n_dir= d if r <= 0 else d+1
				3: n_dir= d if q >= 0 else d+1
				4: n_dir= d if s <= 0 else d+1
				5: n_dir= d if r >= 0 else d+1
			ncq += HexUtil.get_dir(n_dir)[0]
			ncr += HexUtil.get_dir(n_dir)[1]
			ncs += HexUtil.get_dir(n_dir)[2]
			if(ncq + ncr + ncs) != 0:
				printerr("Bad chunk coodinates")
			var n_qrs = HexUtil.tile_to_other_chunk(PackedInt32Array([nq, nr, ns]),PackedInt32Array([chunk_q, chunk_r, chunk_s]),PackedInt32Array([ncq, ncr, ncs]), tile_layers)
			nq = n_qrs[0]
			nr = n_qrs[1]
			ns = n_qrs[2]
		neighbors.append([nq, nr, ns, ncq, ncr, ncs])
		print(neighbors[d])
	pass
	print() */

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
    #[func]
    pub fn generate_mesh(&self, size:Vector3, neighbours:Option<Gd<SpiralHexGrid>>) -> Gd<godot::classes::ArrayMesh> {
        let mut mesh = godot::classes::ArrayMesh::new_gd();
        let mut vertecies = PackedVector3Array::new();
        vertecies.resize(self.data.len()*6);

        //TODO find height of each corner of each hex based on heights of neiboring hex tiles.
        // Since each corner is shared between three tiles, a corner's height will be the average of the three

        
        mesh //returns final mesh
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
