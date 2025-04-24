use godot::classes::{IMesh, Mesh};
use godot::prelude::*;
#[derive(GodotClass)]
#[class(base=Mesh)]
struct SpiralHexMesh {

}
#[godot_api]
impl IMesh for SpiralHexMesh {
	///Number of contigues surfaces in the mesh.
	fn get_surface_count(&self,) -> i32 {
        1
    }
	///Number of vertecies per surface
    fn surface_get_array_len(&self, index: i32,) -> i32 {
        if index == 0{
			0 //TODO fetch number of vertecies 
		} else {
			0
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
		Mesh::ARRAY_FORMAT_VERTEX | Mesh::ARRAY_FORMAT_INDEX | Mesh::ARRAY_FORMAT_COLOR
    }

    fn surface_get_primitive_type(&self, _index: i32,) -> u32 {
        //NOTE triangle strips may be a thing to try.
		
    }

    fn surface_set_material(&mut self, _index: i32, material: Option< Gd< godot::classes::Material > >,) {
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

    fn set_blend_shape_name(&mut self, _index: i32, name: StringName,) {
        //NOTE: consider if blend shapes may be of any use to this mesh. doupt it.

    }
	///Defines a box where the entire mesh resides. Used for automatic culling
    fn get_aabb(&self,) -> Aabb {
		//TODO calculate the Axis-Aligned Bounding Box of the mesh.
		Aabb { position: Vector3::ZERO, size: Vector3::ZERO }
    }
    
    fn init(base: godot::obj::Base < Self::Base >) -> Self {
        //TODO: Store base
		Self {  }
    }
    
    fn to_string(&self) -> godot::builtin::GString {
        //TODO: describe mesh
		GString::from("Spiral Hex Mesh")

    }
    
    fn on_notification(&mut self, what: godot::classes::notify::ObjectNotification) {
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
        //No properties to list for now.
		vec![]
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
