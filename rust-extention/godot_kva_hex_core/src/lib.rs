#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
use std::cmp;
use std::u8;

use godot::classes::image::Format;
use godot::classes::mesh::ArrayType;
use godot::classes::mesh::PrimitiveType;
use godot::classes::Image;
use godot::prelude::*;
use kva_hex_core::direction::get_dir;
use kva_hex_core::Hex;
use kva_hex_core::spiral;
use kva_hex_core::Hex32;

//Number of layers in a spiral grid.
//const NUM_LAYERS:u8 = 255;
//number of tiles in a spiral grid.
//at u8::MAX this would be 195'841 tiles.
//orientation of tiles. Flat means a flat edge towards 'north'. False value means pointy edge towards 'north'.
const FLAT:bool = true; 

//pub mod mesh;
pub mod node;

#[derive(GodotClass)]
#[class(base=Resource)]
struct SpiralHexGrid {
    //data:[HexContent; NUM_TILES],
    data:Vec<HexContent>,
    super_pos:Hex32, //coordinates of this grid within a grid of grids.
    num_layers:u8,
    raw_heightdata:PackedByteArray,
    raw_size:Vector2i
}

#[godot_api]
impl IResource for SpiralHexGrid {
    fn init(_base: godot::obj::Base < Self::Base >) -> Self {
        //godot_print!("Number of layers: {NUM_LAYERS} - Number of tiles: {NUM_TILES}");
        //std::unimplemented !()
        //Self {data:vec![], layers: 0, super_pos:Hex{q:0,r:0}, origin:Hex{q:0,r:0}}
        Self {data:vec![HexContent{height:0}], num_layers: 0, super_pos:Hex{q:0,r:0}, raw_heightdata:PackedByteArray::new(), raw_size:Vector2i::ONE}
        
    }
}



fn get_height_by_sample(x0:i32, y0:i32, x1:i32, y1:i32, width:i32, num_chan:i32, data: &PackedByteArray) ->u8 {
    let mut index:usize;
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
/* fn get_height_by_sample2(x0:i32, y0:i32, x1:i32, y1:i32, map: &Image) ->u8 {
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
 */
#[godot_api]
impl SpiralHexGrid {
    #[func]
    pub fn get_layers(&self)->i32{
        return self.num_layers as i32;
    }
    pub fn _get_layers(&self)->u8{
        return self.num_layers;
    }
    #[func]
    pub fn set_layers(&mut self, layers:i32){self._set_layers(layers as u8)}
    pub fn _set_layers(&mut self, layers:u8){
        self.num_layers = layers;
        self.data.resize(self._get_tile_count(), HexContent { height: 0 });
    }
    #[func]
    pub fn get_tile_count(&self)->i32{self._get_tile_count() as i32}
    pub fn _get_tile_count(&self)->usize{
        return 3 * (self.num_layers as usize +1) * self.num_layers as usize + 1;
    }

    /// sets the posision of the super-hexagon, in relation to other super hexagons.
    /// This will impact the origin coordinates, which as used to convert local coordinates to global coordinates.
    #[func]
    pub fn set_super_pos(&mut self, super_pos_q:i32, super_pos_r:i32) {
        self.super_pos.q = super_pos_q;
        self.super_pos.r = super_pos_r;
    }
    pub fn calculate_origin(&self, super_pos:Hex32)->Hex32{
        use kva_hex_core::direction::*;
        use spiral::*;

        //gather data to convert super posision to origin.
        let s_spiral:Spiral = Spiral::try_from(super_pos).expect("Spiral::try_from<Hex32> should convert with no issues.");
        let dir_seg: Hex32 = get_dir(s_spiral.segment()).into();
        let dir_seg_p2: Hex32 = get_dir(s_spiral.segment()+2).into();
        let dir_seg_m2: Hex32 = get_dir(s_spiral.segment()-2).into();

        //conversation constants
        let layer_scale = self.get_layers() * 2 + 1;
        let pos_scale = self.get_layers();

        //calculate origin
        dir_seg * s_spiral.layer * layer_scale + 
        dir_seg_p2 * s_spiral.s_posision() * layer_scale + 
        dir_seg_m2 * self.get_layers() * s_spiral.layer + 
        dir_seg * s_spiral.s_posision() * pos_scale
    }
    pub fn origin(&self)->Hex32{
        self.calculate_origin(self.super_pos)
    }
    #[func]
    pub fn origin_packed_array(&self)->PackedInt32Array {PackedInt32Array::from(self.origin().as_array() )}
    #[func]
    pub fn super_pos_packed_array(&self)-> PackedInt32Array {PackedInt32Array::from(self.super_pos.as_array())}

    #[func]
    fn load_image(&mut self, mut img:Gd<Image>) {
        img.convert(Format::RG8);
        self.raw_heightdata = img.get_data();
        self.raw_size = img.get_size();
    }

    #[func]
    pub fn regenerate(&mut self) {
        let num_tiles = self._get_tile_count();
        //self.layers = layers;
        //format is set to RGB (3 bytes per pixel) in load_image.
        let num_chan = 3;
        //We use red channel for heightdata
        let channel = 0;
        let num_layers = self._get_layers();
            
        let data: PackedByteArray = self.raw_heightdata.clone();

        //sample sizes. How many pixels per hexagon.
        let scale:Vector2 = self.raw_size.into() / (num_layers * 2 +1);
        let sample:Vector2i = Vector2i { x: (scale.x.round() as i32).max(1), y: (scale.y.round() as i32).max(1) };
        //godot_print!("size {size}, x_s_size {x_s_size}, y_s_size {y_s_size}");

        //for each hexagon tile from center, spiraling out layer by layer
        for i in 0..num_tiles{
            //get hex coordinates by layer index
            let hex: Hex32 = spiral::spiral_index_to_hex(i);
            //generate local x and y coordinates of hexagon, where tile 0 is center of the map
            let mut x: f32;
            let mut y: f32;
            (x, y) = hex.to_xy(true);
            //apply scale and offset
            x = x * scale.x + self.raw_size.x as f32 /2f32;
            y = y * scale.y + self.raw_size.y as f32 /2f32;
            //Get area of pixels to sample for hexagon

            let x_min = (x as i32 - sample.x/2).clamp(0, self.raw_size.x-2);
            let x_max = (x as i32 + sample.x/2).clamp(1, self.raw_size.x-1);
            let y_min = (y as i32 - sample.y/2).clamp(0, self.raw_size.y-2);
            let y_max = (y as i32 + sample.y/2).clamp(1, self.raw_size.y-1);

            self.data[i].height = get_height_by_sample(x_min, y_min, x_max, y_max, self.raw_size.x, num_chan, &data);
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
    ///returns neighbors of a tile (presumed) in this grid. Returns tile coordinates in local space and optionally direction index of neighboring grid for tiles outside of layer range.
    /// Local coordinates can be converted to global by adding self.origin(). Neighbor direction index can be converted to super posision with kva_hex_core::direction::get_dir().
    /// Local coordinates can be converted to neighboring grid's coordinates by adding self.to_other_grid_offset()
    pub fn get_neighbors_local(&self, target:Hex32) -> Vec<(Hex32, Option<usize>)>
    {
        let num_layers = self._get_layers();
        let num_tiles = self._get_tile_count();
        //defining return variable, and reserving space.
        //index 0 is the neiboring tile's coordinates in local space.
        //Index 1 is None if in the same grid. Else it is the direction index to the neiboring grid where the neiboring tile would be.
        //local coordinates may be converted to be useful. But that's not the job of this method.
        let mut neighbors: Vec<(Hex32, Option<usize>)> = Vec::new();
        neighbors.resize(6,  (Hex{q:0,r:0},None));
        
        for d in 0..6 {
            let n:Hex32 = target + get_dir(d).into();
            let layers = num_layers as i32;
            
            if n.q.abs() > layers || n.r.abs() >layers || n.s().abs() >layers {
                //determine direction to grid hosting the neighboring tile
                //For each direction, there are two possible neighboring grids, depending on the tile's location.
                let n_dir =  match d{
                    0 => {if target.q   <= 0 {d} else {d+1}}
                    1 => {if target.s() >= 0 {d} else {d+1}}
                    2 => {if target.r   <= 0 {d} else {d+1}}
                    3 => {if target.q   >= 0 {d} else {d+1}}
                    4 => {if target.s() <= 0 {d} else {d+1}}
                    5 => {if target.r   >= 0 {d} else {d+1}}
                    _ => unreachable!("Value of d is ranged 0..6")
                };
                neighbors[d as usize].1 = Some((n_dir%6) as usize);
            }
            neighbors[d as usize].0 = n;
        }
        return neighbors;
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
            let hex: Hex32 = spiral::spiral_index_to_hex(i as usize);
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
    pub fn generate_mesh(&self, size:Vector3, grid_neighbours:Vec<Option<Gd<SpiralHexGrid>>>) -> Gd<godot::classes::ArrayMesh> {
        const VERTS_PER_TILE:usize = 7; // six corners and a center makes 7 vertecies
        const INDICIES_PER_TILE:usize = 6*3; //six triangles make one hexagon, there are 3 vertecies per triangle.

        let num_tiles = self._get_tile_count();

        //structe conststs of mesh arrays according to godot documentation
        let vertex_index:usize = ArrayType::VERTEX.ord() as usize;
        let color_index:usize = ArrayType::COLOR.ord() as usize;
        let indicies_index:usize = ArrayType::INDEX.ord() as usize;
        let packed_array_size:usize = ArrayType::MAX.ord() as usize;

        let scale = Vector3{x:1.0, y:size.y / 255.0, z:1.0};

        // return value
        let mut mesh = godot::classes::ArrayMesh::new_gd();
        //semi-return value (packed into above return value)
        let mut packed_arrays = VariantArray::new();

/*         let mut vertecies = PackedVector3Array::new();
        let mut colors = PackedColorArray::new();
        let mut indecies = PackedInt32Array::new(); */

        let mut vertecies = vec![]; //[Vector3::ZERO; num_tiles * VERTS_PER_TILE]; 
        let mut colors = vec![]; //[Color::BLACK; num_tiles * VERTS_PER_TILE];
        let mut indecies = vec![]; //[0i32; num_tiles* INDICIES_PER_TILE];
        vertecies.resize(num_tiles * VERTS_PER_TILE, Vector3::ZERO);
        colors.resize(num_tiles * VERTS_PER_TILE, Color::BLACK);
        indecies.resize(num_tiles * INDICIES_PER_TILE, 0i32);

        //resize arrays to expected sizes.
/*         vertecies.resize(NUM_TILES * VERTS_PER_TILE);
        colors.resize(NUM_TILES * VERTS_PER_TILE);
        indecies.resize(NUM_TILES* INDICIES_PER_TILE); */

        //test
        //godot_print!("Heights!");
/*         for t in 0..NUM_TILES{
            let h = self.data[t].height;
            godot_print!("tile {t} is {h}");
        } */
        //for each tile. This may take some time...
        for i in 0..num_tiles {
            //godot_print!("Drawing tile {i}");

            //godot_print!("drawing mesh for tile index {i} of {NUM_TILES}");
            const COLOR_STEPS:f64 = 0.01;
            //set color for the tiles to spiral out as a rainbow.
            let color = Color::from_hsv((i as f64 *  COLOR_STEPS) % 1.0, 1.0, 1.0);
            let hex = spiral::spiral_index_to_hex(i);

            //TEST - remove me
            // if i as i32 >= NUM_TILES as i32 - 20 {
            //     godot_print!("draw - {}, {}, {}", hex.q, hex.r, hex.s());
            // }

            let height = self.data[i].height as f32;
            let center_raw = hex.to_xy(FLAT);
            let center = Vector3{x:center_raw.0, y:height, z:center_raw.1} * scale;
            
            let vertex_index_start = i * VERTS_PER_TILE;
            let indicies_index_start = i * INDICIES_PER_TILE;


            vertecies[vertex_index_start] = center;
            colors[vertex_index_start] = Color::BLACK;
            let neighbors = self.get_neighbors_local(hex);
            let mut n_heights = [(255/2) as f32;6];
            for n in 0..6 {
                n_heights[n] = 
                if neighbors[n].1.is_some() {
                    if grid_neighbours[neighbors[n].1.unwrap()].is_some(){
                        let grid_index = neighbors[n].1.unwrap();
                        let grid = grid_neighbours[grid_index].as_ref().unwrap();
                        //convert coodinates from local grid's space to the other grid's space
                        let nhex = neighbors[n].0 + self.origin() - grid.bind().origin();
                        let nindex = spiral::hex_to_spiral_index(nhex);
                        assert!(nindex < num_tiles); //If this fails, there is a math error somewhere!
                        let h = grid.bind().get_heightdata_at(nindex as i32) as f32;
                        //godot_print!("neighbor {n} is in another grid. value set to {h}.");
                        h
                    }
                    else {
                        //godot_print!("neighbor {n} is out of bounds. value set to {}.", 0f32);
                        0f32
                    }
                }
                else {
                    let nindex = spiral::hex_to_spiral_index(neighbors[n].0);
                    assert!(nindex < num_tiles); //If this fails, there is a math error somewhere!
                    let h = self.get_heightdata_at(nindex as i32) as f32;
                    //godot_print!("neighbor {n} was found as index {nindex}. value set to {h}.");
                    h
                }
            }

            //mark neighbor heights as immutable from now on.
            let n_heights = n_heights;

            //for each corner
            for c in 0..6 {
                //godot_print!("Printing corner {c}");
                let v1 = vertex_index_start + 1 + c;
                let v2 = vertex_index_start + 1 + (c+1)%6;

                let h1 = n_heights[c];
                let h2 = n_heights[(c+1)%6];
                let h = (height as f32 + h1 + h2) / 3.0;
                //godot_print!("height = ({height} + {h1} + {h2}) / 3 =  {h}");

                let vertex = Vector3{
                    x: {if FLAT {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.0
                    + center_raw.0,
                    y: h,
                    //y: 0.0,
                    z: {if FLAT {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.1
                    + center_raw.1,
                } * scale;
                vertecies[vertex_index_start + c + 1] = vertex;
                colors[vertex_index_start + c + 1] = color;

                //vertecies[center_i + 1 + c]
                indecies[indicies_index_start + c*3 + 0] = vertex_index_start as i32;
                indecies[indicies_index_start + c*3 + 1] = v1 as i32;
                indecies[indicies_index_start + c*3 + 2] = v2 as i32;
                
            }
            //test
            colors[vertex_index_start+1] = Color::WHITE;
            //colors[vertex_index_start+2] = Color::BLUE;
            //colors[vertex_index_start+3] = Color::GREEN;
        }
        /*         godot_print!("vertecies");
        for v in vertecies{
            godot_print!("{v}")
        }
        godot_print!("colors");
        for c in colors{
            godot_print!("{c}")
        }
        godot_print!("indecies");
        for i in indecies{
            godot_print!("{i}")
        } */

        //convert data to Godot varaints.
        let vertecies = PackedVector3Array::from(vertecies).to_variant();
        let colors = PackedColorArray::from(colors).to_variant();
        let indecies = PackedInt32Array::from(indecies).to_variant();
        
        packed_arrays.resize(packed_array_size, &Variant::nil());
        packed_arrays.set(vertex_index, &vertecies);
        packed_arrays.set(color_index, &colors);
        packed_arrays.set(indicies_index, &indecies);
        mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &packed_arrays);

        mesh //returns final mesh
    }

    #[func]
    fn generate_flat_mesh(layers:i32) -> PackedVector3Array{
        unimplemented!()
    }
    #[func]
    fn generate_base_indicies(layers:i32) -> PackedInt32Array {
        unimplemented!()
    }
    #[func]
    fn generate_vertex_color(layers:i32) -> PackedColorArray {
        unimplemented!()
    }

    fn apply_height_to_mesh(&self, layers:i32, mesh:PackedVector3Array) -> PackedVector3Array {
        unimplemented!()

    }
}
const FLAT_UP_CORNERS: [(f32, f32); 6] = [
    (0.500, -0.866),     // 300°: (0.5, -√3/2) 
    (1.000, 0.000),      // 0°: (1, 0)
    (0.500, 0.866),      // 60°: (0.5, √3/2)
    (-0.500, 0.866),     // 120°: (-0.5, √3/2)
    (-1.000, 0.000),     // 180°: (-1, 0)
    (-0.500, -0.866),    // 240°: (-0.5, -√3/2)
];
const POINTY_UP_CORNERS: [(f32, f32); 6] = [
    (0.866, -0.500), 
    (0.866, 0.500), 
    (0.000, 1.000), 
    (-0.866, 0.500), 
    (-0.866, -0.500),  
    (0.000, -1.000), 
];

#[derive(Clone, Copy)]
struct HexContent{
    height:u8
}
impl HexContent {
    // fn default()->Self{
    //     Self{height: u8::MAX/2}
    // }
}

#[gdextension]
unsafe impl ExtensionLibrary for SpiralHexGrid {}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */