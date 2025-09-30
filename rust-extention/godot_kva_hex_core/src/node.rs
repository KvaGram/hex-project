//TODO disabled checks for development. Remove when cleaning.
#![allow(unused_variables)]
#![allow(unused_mut)]
// NOTE to self. Use this as an example/guide for what to implement https://github.com/nicholas-maltbie/Godot-OpenKCC/blob/main/demo-project/scripts/stairs.gd

use std::{array, f32::consts::TAU, ops::Deref};

use godot::{ classes::{base_material_3d::TextureParam, class_macros::{private::callbacks::free, sys::godot_virtual_consts::{AnimationNode::get_child_nodes, EditorPlugin::build}}, mesh::PrimitiveType, CollisionShape3D, ConvexPolygonShape3D, Mesh, MeshInstance3D, StandardMaterial3D, StaticBody3D, SurfaceTool, Texture2D}, meta::{AsObjectArg, ParamType}, obj::WithBaseField, prelude::*};
use kva_hex_core::{direction, spiral::{self}};

use crate::SpiralHexGrid;

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
struct HexGridNode3D {
    base:Base<Node3D>,
	#[export]
	texture:Option<Gd<Texture2D>>,
	grid_mesh:Option<Gd<MeshInstance3D>>,

    #[var(
        set = set_grid,
		get = get_grid
    )]
    #[export]
    grid:Option<Gd<SpiralHexGrid>>,

    #[var(
        set = set_layers,
		get = get_layers,
    )]
    #[export(range = (0f64, 255f64))]
    layers:u8,

	// #[export(
	// 	usage = PropertyUsage::EDITOR,
	// 	hint = PropertyHint::TOOL_BUTTON,
	// 	hint_string = "Update Grid"
	// )]
	//#[export]
	#[var(
		usage_flags = [EDITOR],
		hint = TOOL_BUTTON,
		hint_string = "Update Grid"
	)]
	do_update_grid:Callable,


	// #[var(
	// set = set_grid
    // )]
    // #[export]
}

#[godot_api]
impl INode3D for HexGridNode3D {
	fn init(base: godot::obj::Base < Self::Base >) -> Self {

		HexGridNode3D { base:base,
		texture: None,
		grid_mesh: None,
		grid:None,
		layers:2,
		do_update_grid:Callable::invalid(),


		}
	}
	fn enter_tree(&mut self,) {
		self.do_update_grid = self.base().callable("update_grid");
	}
	fn ready(&mut self,) {
		for c in self.base().get_children().iter_shared(){
			if let Ok(mi) = c.try_cast::<MeshInstance3D>(){
				self.grid_mesh = Some(mi);
				break;
			}

		}
		if self.grid_mesh.is_none() {
			let mi = MeshInstance3D::new_alloc();
			self.base_mut().add_child(&mi);
			self.grid_mesh = Some (mi);
		}
		self.regenerate();
	}
}
#[godot_api]
impl HexGridNode3D {
	#[func]
	pub fn regen_base_grid(&mut self){
		//planned upgrade, splitting regenerate in two.
		//this part planned to run only when shape, tile-size or orientation changes.
		//Not updated on changes in scale or heightdata
		//TODO

	}


	#[func]
	pub fn regenerate(&mut self){
		let tile_count = 0; //todo!!!

		//constants for array sizes, and array indexing
		const VERTS_PER_TILE:usize = 7; // six corners and a center makes 7 vertecies
		const INDICIES_PER_TILE:usize = 6*3; //six triangles make one hexagon, there are 3 vertecies per triangle.
		//size of arrays
		let vertex_count = tile_count * VERTS_PER_TILE;
		let index_count = tile_count * INDICIES_PER_TILE;

		//Tool used to construct the mesh
		let mut builder:Gd<SurfaceTool> = SurfaceTool::new_gd();
		let mut mat:Gd<StandardMaterial3D> = StandardMaterial3D::new_gd();
		mat.set_texture(TextureParam::ALBEDO, self.texture.as_ref());
		mat.set_albedo(Color::GREEN);
		builder.begin(PrimitiveType::TRIANGLES);
		builder.set_material(&mat);

		//variables to be set by object settings
		let tile_count = 7;
		//let layer_count = 1;
		let flat_north = true;
		//heightdata copied from grid object, or a placeholder of size equal to 'tile_count'.
		let heightdata = [5,4,4,2,2,2,2];
		//scale of tiles. Defaults to (1,0.1,1).
		let scale = Vector3{x:1f32, y:0.1f32, z:1f32};

		//arrays with data
		//let mut vertecies = PackedVector3Array::new();
		//vertecies.resize(vertex_count);
		//let mut indicies = PackedInt32Array::new();
		//indicies.resize(index_count);
		
		//process each tile
		for i in 0..tile_count {
			//tile start index of vertex and indicies
			let vi_start = i * VERTS_PER_TILE;
			//becouse of change to use the builder directly, and becouse the builder's arrays are hidden, vertecies have to be added linerarly. Center vertex have been moved to last in a hex.
			let vi_center = vi_start + 6;
            let ii_start = i * INDICIES_PER_TILE;

			//Current hex, from index in a spiral
			let hex = spiral::spiral_index_to_hex(i);

			let height:f32 = heightdata[i] as f32;
			//getting heighdata relevant to this tile.
			//defaults to 0 for out of bounds.
			//TODO: implement support for other grids
			let n_heights: [f32; 6] = array::from_fn(|d| {
				let d = d as i32;
				let neighbor = hex + direction::get_dir(d).into();
				let index = spiral::hex_to_spiral_index(neighbor);
				heightdata.get(index).map(|&h| h as f32).unwrap_or(0.0)
			});
			//vertex for center of tile.
			let center = hex.to_xy(flat_north);
			let mut center = Vector3{x:center.0, y:0f32, z: center.1};

			//For each corner (and each polygon/triangle)
			for c in 0..6 {
				//vertex indicies
				let v1 = vi_start + c;
                let v2 = vi_start + (c+1)%6;
				
				//calculate height for this corner. Average of corner's adjacent tiles.
                let h1 = n_heights[c];
                let h2 = n_heights[(c+1)%6];
                let h = (height as f32 + h1 + h2) / 3.0;

				//calculate vertex of corner
				let vi_1 = vi_start + 1 + c;
                let vi_2 = vi_start + 1 + (c+1)%6;
				let mut v = Self::get_corner(c as i32, flat_north) + center;
				builder.set_uv(Vector2 { x: v.x, y: v.z });
				//set height
				v.y = h;

				builder.set_uv(Vector2{x:v.x, y:v.z});

				//scale
				v *= scale;
				//Store vertex and indicies
				builder.add_vertex(v);

				builder.add_index(vi_center as i32);
				builder.add_index(v1 as i32);
				builder.add_index(v2 as i32);
				//vertecies[vi_start + c + 1] = v;
				/*
				indicies[ii_start + c*3 + 0] = vi_center as i32;
				indicies[ii_start + c*3 + 1] = v1 as i32;
				indicies[ii_start + c*3 + 2] = v2 as i32;
				*/
				//wip-todo...

				//NOTE: look into reuseing previus layer's vertecies. Will save a lot on data.
			}
			// set center vertex height and scale, then store it.
			builder.set_uv(Vector2 { x: center.x, y: center.z });
			center.y = height;
			center *= scale;
			builder.add_vertex(center);
			//vertecies[vi_start] = center;
		}
		builder.generate_normals();
		if self.grid_mesh.is_none(){
			self._autoset_grid_mesh();
		}
		if let Some(grid_mesh) = &mut self.grid_mesh {
			grid_mesh.set_mesh(builder.commit().as_ref());
			for c in grid_mesh.get_children().iter_shared(){
				//clean earlier child nodes
				grid_mesh.remove_child(&c);
				c.free();
			}
			//Automatically create collision shapes. NOTE: Potentially expensive.
			grid_mesh.create_multiple_convex_collisions();
		}
		//NOTE consider implementing a custom shape for collision.
		

		/*
		
if Engine.is_editor_hint():
	if save_mesh and is_inside_tree():
		var root = get_tree().edited_scene_root
		mesh_instance.owner = root
		static_body.owner = root
		collision_body.owner = root
	elif not save_mesh:
		mesh_instance.owner = null
		static_body.owner = null
		collision_body.owner = null */
		

		
	}
	fn get_corner(c:i32, flat_north:bool) -> Vector3{
		let angle:i32 = 60 *c + {if flat_north {-60} else {-90}}; //angle degrees, with offsets
		let angle:f32 = TAU / 360f32 * angle as f32; //angle radians
		Vector3 { x: angle.cos(), y: 0.0, z: angle.sin() } //point
	}
	fn _autoset_grid_mesh(&mut self){
		if let Some(grid_mesh) = self.base().find_child("grid_mesh") {
			if let Ok(grid_mesh) = grid_mesh.try_cast::<MeshInstance3D>(){
				self.grid_mesh = Some(grid_mesh);
				return;
			}
		}
		for c in self.base().get_children().iter_shared(){			
			if let Ok(grid_mesh) = c.try_cast::<MeshInstance3D>() {
				self.grid_mesh = Some(grid_mesh);
				return;
			}
		}
		let mut grid_mesh = MeshInstance3D::new_alloc();
		grid_mesh.set_name("grid_mesh");
		self.base_mut().add_child(&grid_mesh);
		self.grid_mesh = Some(grid_mesh);
		return;
	}
	#[func]
	fn set_grid(&mut self, new_grid:Option<Gd<SpiralHexGrid>>){
		self.grid = new_grid;
		self.regenerate();
		//TODO
	}
	#[func]
	fn get_grid(&mut self) -> Option<Gd<SpiralHexGrid>>{
		self.grid.to_godot()
	}
	#[func]
	fn set_layers(&mut self, value:i32){
		self.layers = value as u8;
		self.regenerate();
		//TODO
	}
	#[func]
	fn get_layers(&mut self)-> i32 {
		self.layers as i32
	}

	#[func]
	fn update_grid(&mut self){
		//TODO
		godot_print!("UPDATE_GRID");
		if let Some(grid) = self.grid.as_mut() {
			grid.bind_mut()._set_layers(self.layers);
			grid.bind_mut().regenerate();
		}
		self.regenerate();
	}

}