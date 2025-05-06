use godot::classes::mesh::ArrayFormat;
use godot::classes::{IMesh, Mesh};
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::meta::{ClassName, PropertyHintInfo};
use godot::prelude::*;
use bitflags::bitflags;
use crate::SpiralHexGrid;

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
    //If a grid is not defined, layers can be set manually. A value of 0 will result in just one single tile.
    layers:u8,
    //flags are used to define what surface layers are rendered.
    flags:RenderFlags,
    //contains data regarding the current animation effect, or lack thereof.
    animate_data:Option<AnimateData>,
    //defines the orientation to render the tiles in, using "north" as Z-axis, has a flat surface towards north if true.
    flat_north:bool,
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
        MeshSize::Fixed(Vector3{x:1.0, y:0.004, z:1.0})
    }
}

///stores data regarding an active animation. This includes a copy of data before animation, the current animation step, what tiles are being moved, origin and destination tiles of the animation, the geometry data of the moving geometry,
/// and what else I can't think of while prototyping this structure right now :) .
struct AnimateData {
    step:AniStep
}
///Each step holds a time value, and an end value, messured in microseconds. These are used to lerp values for animation. Once t exceeds e, the animation is advanced to the next step. The excess value of t above e of the previus step is added to the next value t. This ensures consistant animation length.
/// Lift: the tile(s) lifts from the grid. EndLift: Short pause. Move: the tiles move to their target destination. EndMove: short pause. Drop: Tiles drop down to their destination. EndDrop: short pause. Morph: tile height adjusts to the new geometry as tile data changes End: short pause. None  - No animation is running.
enum AniStep{None, Lift{t:i32, e:i32}, EndLift{t:i32, e:i32}, Move{t:i32, e:i32}, EndMove{t:i32, e:i32}, Drop{t:i32, e:i32}, EndDrop{t:i32, e:i32}, Morph{t:i32, e:i32}, End{i:i32, e:i32}}

bitflags! {
    pub struct RenderFlags: u32 {
        const READY  =      1 << 0;
        const WALL_1 =      1 << 1;
        const WALL_2 =      1 << 2;
        const WALL_3 =      1 << 3;
        const WALL_4 =      1 << 4;
        const WALL_5 =      1 << 5;
        const WALL_6 =      1 << 6;
        const ANIMATING =   1 << 7;

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
            self.regenerate();
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

    fn regenerate(&mut self){
        use kva_hex_core::spiral;
        const VERTS_PER_TILE:usize = 7; // six corners and a center makes 7 vertecies
        const INDICIES_PER_TILE:usize = 6*3; //six triangles make one hexagon, there are 3 vertecies per triangle.
        let grid_vertex_size = self._get_num_tiles() * VERTS_PER_TILE;
        let grid_index_size = self._get_num_tiles()* INDICIES_PER_TILE;

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

        let heightdata: PackedByteArray = self.grid.as_ref().map_or(PackedByteArray::new(), |g|{Some(g.bind().get_heightdata().clone())});
        for i in 0..self._get_num_tiles() {
            let color:f64 = (i % 500) as f64 / 500f64;
            let color = Color::from_hsv(color, 1.0, 1.0);
            let hex = spiral::spiral_index_to_hex(i);
            let height = heightdata.get(i).unwrap_or(0);

            if DEBUG_01{
                if i as i32 >= self.get_num_tiles() - 20 {
                    godot_print!("draw - {}, {}, {}", hex.q, hex.r, hex.s());
                }
            }
        }

        //grid must be dropped so that self can be rused for base.
        drop(grid);
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
            0 => {if self.flags.contains(RenderFlags::READY) { 0 } else { 0 }}
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
			0 //TODO fetch number of vertecies 
		} else {
			0
		}
    }
	///get the full arrays of mesh data. Vertex, UV, indicies, etc. For given surface.
    fn surface_get_arrays(&self, _index: i32,) -> VariantArray {
		//TODO pack relevant data and return copy.
		VariantArray::new()
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
		(ArrayFormat::VERTEX | ArrayFormat::INDEX | ArrayFormat::COLOR).ord() as u32
    }

    fn surface_get_primitive_type(&self, _index: i32,) -> u32 {
        if _index > 0 {0}
        else {0}
		
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

    }
	///Defines a box where the entire mesh resides. Used for automatic culling
    fn get_aabb(&self,) -> Aabb {
		//TODO calculate the Axis-Aligned Bounding Box of the mesh.
		Aabb { position: Vector3::ZERO, size: Vector3::ZERO }
    }
    
    fn init(base: godot::obj::Base < Self::Base >) -> Self {
        
        //TODO: Store base
		Self {base, grid:None, layers:3u8, flags:RenderFlags::empty(), animate_data:None, flat_north:true, grid_verticies:vec!(), grid_colors:vec!(), grid_indicies:vec!(), size: MeshSize::default()}
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
    
    fn set_property(&mut self, _property: StringName, _value: Variant) -> bool {
        //No properties to set for now.
		false
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
        //Nothing needs doing. Maybe. I donno...
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
