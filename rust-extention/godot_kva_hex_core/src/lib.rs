use std::cmp;
use std::u8;

use godot::classes::mesh::PrimitiveType;
use godot::classes::Image;
use godot::classes::Mesh;
use godot::prelude::*;
use kva_hex_core::direction::get_dir;
use kva_hex_core::Hex;
use kva_hex_core::spiral;

//Number of layers in a spiral grid.
const NUM_LAYERS:u8 = 3;
//number of tiles in a spiral grid.
//at u8::MAX this would be 195'841 tiles.
const NUM_TILES:usize = 3 * (NUM_LAYERS as usize +1) * NUM_LAYERS as usize + 1;
//orientation of tiles. Flat means a flat edge towards 'north'. False value means pointy edge towards 'north'.
const FLAT:bool = true;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct SpiralHexGrid {
    data:[HexContent; NUM_TILES],
    super_pos:Hex<i32>, //coordinates of this grid within a grid of grids.
}

#[godot_api]
impl IRefCounted for SpiralHexGrid {
    fn init(_base: godot::obj::Base < Self::Base >) -> Self {
        //std::unimplemented !()
        //Self {data:vec![], layers: 0, super_pos:Hex{q:0,r:0}, origin:Hex{q:0,r:0}}
        Self {data:[HexContent::default(); NUM_TILES], /*layers: 0,*/ super_pos:Hex{q:0,r:0}, origin:Hex{q:0,r:0}}
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
    /// This will impact the origin coordinates, which as used to convert local coordinates to global coordinates.
    #[func]
    pub fn set_super_pos(&mut self, super_pos_q:i32, super_pos_r:i32) {
        self.super_pos.q = super_pos_q;
        self.super_pos.r = super_pos_r;
    }
    pub fn calculate_origin(super_pos:Hex<i32>)->Hex<i32>{
        use kva_hex_core::direction::*;
        use spiral::*;

        //gather data to convert super posision to origin.
        let s_spiral:Spiral = Spiral::try_from(super_pos).expect("Spiral::try_from<Hex<i32>> should convert with no issues.");
        let dir_seg: Hex<i32> = get_dir(s_spiral.segment()).into();
        let dir_seg_p2: Hex<i32> = get_dir(s_spiral.segment()+2).into();
        let dir_seg_m2: Hex<i32> = get_dir(s_spiral.segment()-2).into();

        //conversation constants
        let layer_scale = NUM_LAYERS as i32 * 2 + 1;
        let pos_scale = NUM_LAYERS;

        //calculate origin
        dir_seg * s_spiral.layer * layer_scale + 
        dir_seg_p2 * s_spiral.s_posision() * layer_scale + 
        dir_seg_m2 * NUM_LAYERS * s_spiral.layer + 
        dir_seg * s_spiral.s_posision() * pos_scale
    }
    pub fn origin(&self)->Hex<i32>{
        Self::calculate_origin(self.super_pos)
    }
    #[func]
    pub fn origin_packed_array(&self)->PackedInt32Array {PackedInt32Array::from(self.origin().as_array() )}
    #[func]
    pub fn super_pos_packed_array(&self)-> PackedInt32Array {PackedInt32Array::from(self.super_pos.as_array())}
    #[func]
    pub fn from_hightmap(&mut self, /*layers:u8,*/ map:Gd<Image>) {
        //self.layers = layers;
        let num_chan = {
            let ord = map.detect_used_channels().ord();
            if ord == 0 {1}
            else if ord == 1 || ord == 3 {2}
            else if ord == 4 {3}
            else if ord == 5 {4}
            else {3}
        };
            
        let data: PackedByteArray = map.get_data();
        //let size: usize = (3 * (NUM_LAYERS+1) * NUM_LAYERS + 1) as usize;
        let (width, _height) = (map.get_width(), map.get_height());
        //sample sizes. How big a rectangle (by radius) does each tile need to sample? For average height.
        let x_s_size:i32 = map.get_width()  / (NUM_LAYERS*2) as i32;
        let y_s_size:i32 = map.get_height() / (NUM_LAYERS*2) as i32;
        let scale_x: f32 = map.get_width() as f32 / (2 * NUM_LAYERS) as f32;
        let scale_y: f32 = map.get_height() as f32 / (2 * NUM_LAYERS) as f32;
        //godot_print!("size {size}, x_s_size {x_s_size}, y_s_size {y_s_size}");

        //self.data.resize(size, HexContent { height: 0 });
        for i in 0..NUM_TILES{
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
    pub fn to_other_grid_offset(&self, &other:Self)->Hex<i32>{
        other.origin() - self.origin()
    }
    ///returns neighbors of a tile (presumed) in this grid. Returns tile coordinates in local space and optionally direction index of neighboring grid for tiles outside of layer range.
    /// Local coordinates can be converted to global by adding self.origin(). Neighbor direction index can be converted to super posision with kva_hex_core::direction::get_dir().
    /// Local coordinates can be converted to neighboring grid's coordinates by adding self.to_other_grid_offset()
    pub fn get_neighbors_local(&self, target:Hex<i32>) -> Vec<(Hex<i32>, Option<usize>)>
    {
        //defining return variable, and reserving space.
        //index 0 is the neiboring tile's coordinates in local space.
        //Index 1 is None if in the same grid. Else it is the direction index to the neiboring grid where the neiboring tile would be.
        //local coordinates may be converted to be useful. But that's not the job of this method.
        let mut neighbors: Vec<(Hex<i32>, Option<usize>)> = Vec::new();
        neighbors.resize(6,  (Hex{q:0,r:0},None));
        
        for d in 0..6 {
            let mut n:Hex<i32> = get_dir(d).into();
            let layers = NUM_LAYERS as i32;
            
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

    fn getData(&self)->&[HexContent; NUM_TILES]{
        return &self.data;
    }
    #[func]
    pub fn generate_mesh(&self, size:Vector3, grid_neighbours:Vec<Option<Gd<SpiralHexGrid>>>) -> Gd<godot::classes::ArrayMesh> {
        const VERTS_PER_TILE:usize = 7; // six corners and a center makes 7 vertecies
        const INDICIES_PER_TILE:usize = 6*3; //six triangles make one hexagon, there are 3 vertecies per triangle.

        //indicies of mesh arrays according to godot documentation
        const VERTEX_INDEX:usize = 0;
        //const UV1_INDEX:usize = 4;
        const COLOR_INDEX:usize = 3;
        const INDICIES_INDEX:usize = 12;

        let scale = Vector3{x:1.0, y:size.y / 255.0, z:1.0};

        // return value
        let mut mesh = godot::classes::ArrayMesh::new_gd();
        //semi-return value (packed into above return value)
        let mut packed_arrays = VariantArray::new();

        let mut vertecies = PackedVector3Array::new();
        let mut colors = PackedColorArray::new();
        let mut indecies = PackedInt32Array::new();

        //resize arrays to expected sizes.
        vertecies.resize(NUM_TILES * VERTS_PER_TILE);
        colors.resize(NUM_TILES * VERTS_PER_TILE);
        indecies.resize(NUM_TILES* INDICIES_PER_TILE);

        //for each tile. This may take some time...
        for i in 0..NUM_TILES {
            const COLOR_STEPS:f64 = 0.01;
            //set color for the tiles to spiral out as a rainbow.
            let color = Color::from_hsv((i as f64 *  COLOR_STEPS) % 1.0, 1.0, 1.0);
            let hex = spiral::spiral_index_to_hex(i);
            let height = self.data[i].height;
            let center = hex.to_xy(FLAT);
            let center = Vector3{x:center.0, y:height as f32, z:center.1} * scale;
            let center_i = i * VERTS_PER_TILE;
            vertecies[center_i] = center;
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
                        assert!(nindex < NUM_TILES); //If this fails, there is a math error somewhere!
                        grid.bind().get_heightdata_at(nindex as i32) as f32
                    }
                    else {(255/2) as f32}
                }
                else {
                    let nindex = spiral::hex_to_spiral_index(neighbors[n].0);
                    assert!(nindex < NUM_TILES); //If this fails, there is a math error somewhere!
                    self.get_heightdata_at(nindex as i32) as f32
                }
            }

            //mark neighbor heights as immutable from now on.
            let n_heights = n_heights;

            //for each corner
            for c in 0..6 {
                let v1 = center_i + c;
                let v2 = center_i + (c+1)%6;

                let vertex = Vector3{
                    x: {if FLAT {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.0,
                    y: (height as f32 +n_heights[c] + n_heights[(c+1)%6])/3f32,
                    z: {if FLAT {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.1,
                };
                vertecies[center_i+c] = vertex;
                colors[center_i+c] = color;

                //vertecies[center_i + 1 + c]
                indecies[center_i + 1 + c*3 + 0] = v1 as i32;
                indecies[center_i + 1 + c*3 + 1] = center_i as i32;
                indecies[center_i + 1 + c*3 + 2] = v2 as i32;
                
            }
            //todo - get neighbors, handle the literal edge cases, calculate height for each corner of the hex.
            //store vertecies and color into their arrays.
            //compile the polygons by indecies, store those.
        }
        packed_arrays.set(VERTEX_INDEX, &vertecies.to_variant());
        packed_arrays.set(COLOR_INDEX, &colors.to_variant());
        packed_arrays.set(INDICIES_INDEX, &indecies.to_variant());
        mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &packed_arrays);


        //TODO find height of each corner of each hex based on heights of neiboring hex tiles.
        // Since each corner is shared between three tiles, a corner's height will be the average of the three

        
        mesh //returns final mesh
    }
}
const FLAT_UP_CORNERS: [(f32, f32); 6] = [
    (0.500, -0.866),    
    (1.000, 0.000),   
    (0.500, 0.866), 
    (-0.500, 0.866),  
    (-1.000, 0.000), 
    (-0.500, -0.866),   
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
    fn default()->Self{
        Self{height: u8::MAX/2}
    }
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
