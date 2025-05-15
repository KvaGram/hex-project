use godot::classes::mesh::ArrayFormat;
use godot::classes::{IMesh, Mesh};
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::meta::{ClassName, PropertyHintInfo};
use godot::prelude::*;
use bitflags::bitflags;
use crate::SpiralHexGrid;
use std::collections::HashMap;

const DEBUG_01:bool = false;

#[derive(GodotClass)]
#[class(base=Mesh)]
struct SpiralHexMesh {
    base:Base<Mesh>,
    grid_verticies:Vec<Vector3>,
    grid_indicies:Vec<i32>,
    grid_colors:Vec<Color>,
    //if grid is not defined, layers will used without pulling this from grid.
    //This will result in a mesh without heightdata, meaning it's flat
    grid:Option<Gd<SpiralHexGrid>>,
    grid_n:[Option<Gd<SpiralHexGrid>>;6],
    //If a grid is not defined, layers can be set manually. A value of 0 will result in just one single tile.
    layers:u8,
    //flags are used to define what surface layers are rendered.
    flags:RenderFlags,
    //contains data regarding the current animation effect, or lack thereof.
//    animate_data:Option<AnimateData>,
    size:MeshSize,

}

enum MeshSize{
    Scaled(Vector3),
    Fixed(Vector3)
}
impl Default for MeshSize {
    fn default() -> Self {
        //height-values are in u8 integers between 0 and 255.
        //They should be well scaled to fit the situation.
        MeshSize::Scaled(Vector3{x:1.0, y:0.004, z:1.0})
    }
}

///stores data regarding an active animation. This includes a copy of data before animation, the current animation step, what tiles are being moved, origin and destination tiles of the animation, the geometry data of the moving geometry,
/// and what else I can't think of while prototyping this structure right now :) .
// struct AnimateData {
//     step:AniStep
// }
///Each step holds a time value, and an end value, messured in microseconds. These are used to lerp values for animation. Once t exceeds e, the animation is advanced to the next step. The excess value of t above e of the previus step is added to the next value t. This ensures consistant animation length.
/// Lift: the tile(s) lifts from the grid. EndLift: Short pause. Move: the tiles move to their target destination. EndMove: short pause. Drop: Tiles drop down to their destination. EndDrop: short pause. Morph: tile height adjusts to the new geometry as tile data changes End: short pause. None  - No animation is running.
//enum AniStep{None, Lift{t:i32, e:i32}, EndLift{t:i32, e:i32}, Move{t:i32, e:i32}, EndMove{t:i32, e:i32}, Drop{t:i32, e:i32}, EndDrop{t:i32, e:i32}, Morph{t:i32, e:i32}, End{i:i32, e:i32}}

bitflags! {
    pub struct RenderFlags: u32 {
        const REFRESH =      1 << 0; //set when the mesh needs to regenerate due to changed data.
        const WALL_1 =      1 << 1; // whatever to render walls for each directions, adding depth to the hexgrid.
        const WALL_2 =      1 << 2;
        const WALL_3 =      1 << 3;
        const WALL_4 =      1 << 4;
        const WALL_5 =      1 << 5;
        const WALL_6 =      1 << 6;
        const FLAT_NORTH =  1 << 8; //defines the orientation to render the tiles in, using "north" as Z-axis, has a flat surface towards north if true.
        const ANIMATING =   1 << 9;

        // The source may set any bits
        const _ = !0;
    }
}

#[godot_api]
impl SpiralHexMesh
{
    #[func]
    fn set_layers(&mut self, new_layers:i32)->bool {
        let gridlength = {if self.grid.is_none(){None} else {
            let layers:i32 = self.grid.as_ref().unwrap().bind().get_layers();
            Some(layers)
        }};
        if new_layers < 0{
            godot_error!("SpiralHexMesh cannot have a negative layer length. Change discarded");
            return false;
        }
        else if new_layers > u8::MAX as i32 {
            godot_error!("SpiralHexMesh cannot have a layer length greater than {}. Change discarded.", u8::MAX);
            return false;
        }
        else if gridlength.is_some() && gridlength.unwrap() < new_layers {
            godot_error!("SpiralHexMesh already have a defined grid with a length of {}. New length cannot be greater than that. Change discarded.", gridlength.unwrap());
            return false;
        }
        else {
            self.layers = new_layers as u8;
            if gridlength.is_some() && gridlength.unwrap() != new_layers {
                godot_warn!("SpiralHexMesh already have a defined grid with a length of {}. Setting layers manually means less of the grid is rendered in the mesh.", gridlength.unwrap());
            }
            self.need_refresh();
            return true;
        }
    }
    #[func]
    fn set_grid(&mut self, new_grid:Option<Gd<SpiralHexGrid>>) -> bool {
        self.grid = new_grid;
        if self.grid.is_some(){
            self.layers = self.grid.as_ref().unwrap().bind().get_layers() as u8;
        }
        else {
            self.layers = 0;
        }
        self.regenerate();
        return true;
    }
    #[func]
    fn get_num_tiles(&self) -> i32 {self._get_num_tiles() as i32}
    fn _get_num_tiles(&self) -> usize {
        3 * (self.layers as usize +1) * self.layers as usize + 1
    }
    /// sets the refresh flag, and calls regenerate_deferred, deferred. 
    fn need_refresh(&mut self){
        self.flags.set(RenderFlags::REFRESH, true);
        self.base_mut().call_deferred("regenerate_deferred", &[]);
        
    }
    ///runs regenerate if flag REFRESH is set.
    ///Only meant to be called deferred. Do not call directly.
    /// Shields regenerate from being called multible times per frame.
    #[func]
    fn regenerate_deferred(&mut self)
    {
        if self.flags.contains(RenderFlags::REFRESH) {
            self.regenerate();
        }
    }
    /// Regenerate the base grid-mesh
    /// Potentially expensive, only call when needed.
    /// Do not call deferred. Use regenerate_deferred instead for deferred calls.
    #[func]
    fn regenerate(&mut self){
        use kva_hex_core::spiral;
        const VERTS_PER_TILE:usize = 7; // six corners and a center makes 7 vertecies
        const INDICIES_PER_TILE:usize = 6*3; //six triangles make one hexagon, there are 3 vertecies per triangle.
        let num_tiles = self._get_num_tiles();
        let grid_vertex_size = num_tiles * VERTS_PER_TILE;
        let grid_index_size = num_tiles * INDICIES_PER_TILE;

        self.grid_verticies.resize(grid_vertex_size, Vector3::ZERO);
        self.grid_colors.resize(grid_vertex_size, Color::BLACK);
        self.grid_indicies.resize(grid_index_size, 0i32);

        let scale = match self.size {
            MeshSize::Scaled(s) => s,
            MeshSize::Fixed(s) => {
                let length = self.layers as f32 * 2f32+ 1f32;
                //TODO: softcode to allow for alternate different types of height data input
                Vector3{x:s.x / length, y:s.y / 255f32, z:s.z / length}
            }
        };
        let heightdata: PackedByteArray = self.grid.as_ref().map_or(PackedByteArray::new(), |g|{g.bind().get_heightdata()});
        for i in 0..num_tiles {

            let verticies = &mut self.grid_verticies;
            let colors = &mut self.grid_colors;
            let indicies = &mut self.grid_indicies;

            let color:f64 = (i % 500) as f64 / 500f64;
            let color = Color::from_hsv(color, 1.0, 1.0);
            let hex = spiral::spiral_index_to_hex(i);
            let height = heightdata.get(i).unwrap_or(0) as f32;

            if DEBUG_01{
                if i as i32 >= num_tiles as i32 - 20 {
                    godot_print!("draw - {}, {}, {}", hex.q, hex.r, hex.s());
                }
            }
            let center_raw = hex.to_xy(self.flags.contains(RenderFlags::FLAT_NORTH));
            let center = Vector3{x:center_raw.0, y:height, z:center_raw.1} * scale;
            let vi_start = i * VERTS_PER_TILE;
            let ii_start = i * INDICIES_PER_TILE;

            verticies[vi_start] = center;
            colors[vi_start] = Color::BLACK;

            let mut n_heights = [(255/2) as f32;6];
            if  self.grid.as_ref().is_some_and(|g| {g.bind().get_layers() <= hex.get_layer()}) {
                let neighbors = self.grid.as_ref().unwrap().bind().get_neighbors_local(hex);
                for n in 0..6 {
                    n_heights[n] = 
                    if neighbors[n].1.is_some() {
                        if self.grid_n[neighbors[n].1.unwrap()].is_some(){
                            let grid_index = neighbors[n].1.unwrap();
                            let grid = self.grid_n[grid_index].as_ref().unwrap();
                            //convert coodinates from local grid's space to the other grid's space
                            let nhex = neighbors[n].0 + self.grid.as_ref().unwrap().bind().origin() - grid.bind().origin();
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
                        let h = self.grid.as_ref().unwrap().bind().get_heightdata_at(nindex as i32) as f32;
                        //godot_print!("neighbor {n} was found as index {nindex}. value set to {h}.");
                        h
                    }
                }
                //mark neighbor heights as immutable from now on.
                let n_heights = n_heights;
                //for each corner
                for c in 0..6 {
                    //godot_print!("Printing corner {c}");
                    let v1 = vi_start + 1 + c;
                    let v2 = vi_start + 1 + (c+1)%6;

                    let h1 = n_heights[c];
                    let h2 = n_heights[(c+1)%6];
                    let h = (height as f32 + h1 + h2) / 3.0;
                    //godot_print!("height = ({height} + {h1} + {h2}) / 3 =  {h}");

                    let vertex = Vector3{
                        x: {if self.flags.contains(RenderFlags::FLAT_NORTH) {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.0
                        + center_raw.0,
                        y: h,
                        //y: 0.0,
                        z: {if self.flags.contains(RenderFlags::FLAT_NORTH) {FLAT_UP_CORNERS[c]} else {POINTY_UP_CORNERS[c]}}.1
                        + center_raw.1,
                    } * scale;
                    verticies[vi_start + c + 1] = vertex;
                    colors[vi_start + c + 1] = color;

                    //vertecies[center_i + 1 + c]
                    indicies[ii_start + c*3 + 0] = vi_start as i32;
                    indicies[ii_start + c*3 + 1] = v1 as i32;
                    indicies[ii_start + c*3 + 2] = v2 as i32;
                    
                }
                //test coloring first corner as white
                colors[vi_start+1] = Color::WHITE;
            }

        }
        self.flags.set(RenderFlags::REFRESH,false);
        //NOTE: drop any variables that may lock self.base or self.base_mut
        self.base_mut().emit_changed();
    }
}

/* ideas
Layer 0 is the grid itself
Layer 1-6 are toggelable. They are the walls outside of the grid. Each layer is their own side of the grid.
Layer 7 and 8 are reserved for animated geometry.
layers 9 and 10 are reserved for walls for animated geometry
layers 11 and 12 are reserved for walls covering "gaps" of animated geometry.
layers 13 and 15 are reserved for ground convering undearneath animated geometry
layers 15 and 16 are reserved for bedrock/floors covering "gaps" from aniamted geometry.
*/

#[godot_api]
impl IMesh for SpiralHexMesh {
    ///Number of contigues surfaces in the mesh.
	fn get_surface_count(&self,) -> i32 {
        1
    }
	///Number of vertecies per surface
    fn surface_get_array_len(&self, index: i32,) -> i32 {
        match index {
            //TODO: get number of vertecies in base grid
            0 => self.grid_verticies.len() as i32,
            1 => {if self.flags.contains(RenderFlags::WALL_1){ 0 } else { 0 }}
            2 => {if self.flags.contains(RenderFlags::WALL_2){ 0 } else { 0 }}
            3 => {if self.flags.contains(RenderFlags::WALL_3){ 0 } else { 0 }}
            4 => {if self.flags.contains(RenderFlags::WALL_4){ 0 } else { 0 }}
            5 => {if self.flags.contains(RenderFlags::WALL_5){ 0 } else { 0 }}
            6 => {if self.flags.contains(RenderFlags::WALL_6){ 0 } else { 0 }}

            _ => 0
            
        }	
    }

	///Number of indecies per surface
    fn surface_get_array_index_len(&self, index: i32,) -> i32 {
        if index == 0{
			self.grid_indicies.len() as i32
		} else {
			0
		}
    }
	///get the full arrays of mesh data. Vertex, UV, indicies, etc. For given surface.
    fn surface_get_arrays(&self, index: i32,) -> VariantArray {
        use godot::classes::mesh::ArrayType;
        //structe conststs of mesh arrays according to godot documentation
        let vertex_index:usize = ArrayType::VERTEX.ord() as usize;
        let color_index:usize = ArrayType::COLOR.ord() as usize;
        let indicies_index:usize = ArrayType::INDEX.ord() as usize;
        let packed_array_size:usize = ArrayType::MAX.ord() as usize;

        //TODO pack relevant data and return copy.
		let mut ret = VariantArray::new();
        ret.resize(packed_array_size, &Variant::nil());
        ret.set(vertex_index, &PackedVector3Array::from(self.grid_verticies.clone()).to_variant());
        match index{
            0 => {
                ret.set(color_index, &PackedColorArray::from(self.grid_colors.clone()).to_variant());
                ret.set(indicies_index, &PackedInt32Array::from(self.grid_indicies.clone()).to_variant());
            }
            _ => {
                ret.set(color_index, &PackedColorArray::new().to_variant());
                ret.set(indicies_index, &PackedVector3Array::new().to_variant());
            }
        }
        ret
    }

	///gets data relevant to animations. This likely does not apply for this use.
    fn surface_get_blend_shape_arrays(&self, _index: i32,) -> Array< VariantArray > {
        // NOTE: consider if this could be used for something.
		Array::<VariantArray>::new()
    }
	///gets the level of detail mesh data
    fn surface_get_lods(&self, _index: i32,) -> Dictionary {
		// TODO define simplefied LOD meshes
		return Dictionary::new()
    }
	///defines the format of the arrays, utalizing bitflags to set properties.
    fn surface_get_format(&self, _index: i32,) -> u32 {
		//NOTE indexed arrays, using vertex colors instead of UVs.
		(ArrayFormat::VERTEX.ord() | ArrayFormat::INDEX.ord() | ArrayFormat::COLOR.ord()) as u32
    }

    fn surface_get_primitive_type(&self, _index: i32,) -> u32 {
        //use godot::classes::mesh::PrimitiveType;
        //PrimitiveType::TRIANGLES.ord() as u32
        3u32
    }

    fn surface_set_material(&mut self, _index: i32, _material: Option< Gd< godot::classes::Material > >,) {
        //TODO: Store material
    }

    fn surface_get_material(&self, _index: i32,) -> Option< Gd< godot::classes::Material > > {
        None
    }

    fn get_blend_shape_count(&self,) -> i32 {
        //NOTE: consider if blend shapes may be of any use to this mesh. doupt it.
		0
    }

    fn get_blend_shape_name(&self, _index: i32,) -> StringName {
        //NOTE: consider if blend shapes may be of any use to this mesh. doupt it.
		StringName::from("nope")
    }

    fn set_blend_shape_name(&mut self, _index: i32, _name: StringName,) {
        //NOTE: consider if blend shapes may be of any use to this mesh. doupt it.
        //self.need_refresh();

    }
	///Defines a box where the entire mesh resides. Used for automatic culling
    fn get_aabb(&self,) -> Aabb {
        match self.size{
            MeshSize::Fixed(s) => Aabb { position: Vector3 { x: -s.x/2f32, y: -s.y/2f32, z: -s.z/2f32 }, size:s},
            MeshSize::Scaled(s) =>{
                let max = Vector3{x:(self.layers as f32)*s.x, y:255f32*s.y, z:(self.layers as f32)*s.z};
                let min = Vector3{x:-(self.layers as f32)*s.x, y:0f32, z:-(self.layers as f32)*s.z};
                Aabb::from_corners(max, min)
                //TODO calculate the Aabb
                //Aabb{position: Vector3::ZERO, size: Vector3::ONE}
            }
        }

    }
    
    fn init(base: godot::obj::Base < Self::Base >) -> Self {
        
        //TODO: Store base
		Self {base, grid:None, grid_n:[None, None, None, None, None, None], layers:3u8, flags:RenderFlags::empty(), /*animate_data:None,*/ grid_verticies:vec!(), grid_colors:vec!(), grid_indicies:vec!(), size: MeshSize::default()}
    }
    
    fn to_string(&self) -> godot::builtin::GString {
        //TODO: describe mesh
		GString::from("Spiral Hex Mesh")

    }
    
    fn on_notification(&mut self, _what: godot::classes::notify::ObjectNotification) {
        //do nothing for now.
    }
    
    fn get_property(&self, _property: StringName) -> Option< Variant > {
        //No properties to fetch for now.
		None
    }
    
    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        if property == StringName::from("layers"){
            //attempt to get i32 from Variant
            let value = i32::try_from_variant(&value);
            //attempt to set layers from value (set_layers may trigger need_refresh)
            value.is_ok_and(|value| self.set_layers(value))
        }
        else {
            false
        }
    }
    
    fn get_property_list(&mut self) -> Vec< godot::meta::PropertyInfo > {
        use godot::meta::PropertyInfo as Info;
        vec![
            Info{property_name:StringName::from("layers"), variant_type:VariantType::INT, class_name:ClassName::none(), usage: PropertyUsageFlags::DEFAULT,
                hint_info:PropertyHintInfo{hint_string:GString::from("minimum 0, maximum 255. Restricted by unsigned 8 bit integer. This is the number of layers to render. Automatically set by grid(if defined), can be overwritten."), hint: PropertyHint::RANGE}}
            
            ]
        //TODO: list all properties that I may want to view or edit from inspector.
    }
    
    fn validate_property(&self, _property: &mut godot::meta::PropertyInfo) {
        // TODO use this to validate data set by the inspector. Currently none.
    }
    
    fn property_get_revert(&self, _property: StringName) -> Option< Variant > {
        // TODO use this to reset varaible values from the inspector. Currently no such variables exist
		None
    }
    
    fn setup_local_to_scene(&mut self,) {
        //Nothing needs doing. Maybe. I donno... Maybe clone material(s)?
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
