//TODO disabled checks for development. Remove when cleaning.
#![allow(unused_variables)]
#![allow(unused_mut)]
// NOTE to self. Use this as an example/guide for what to implement https://github.com/nicholas-maltbie/Godot-OpenKCC/blob/main/demo-project/scripts/stairs.gd

use std::{array, f32::consts::TAU};

use godot::{ classes::{MeshInstance3D, SurfaceTool}, prelude::*};
use kva_hex_core::{direction, spiral::{self}};

use crate::SpiralHexGrid;

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
struct HexGridNode3D {
    base:Base<Node3D>,
	mi:Option<Gd<MeshInstance3D>>,

    #[var(
        set = set_grid
    )]
    #[export]
    grid:Option<Gd<SpiralHexGrid>>,

    #[var(
        set = set_layers,
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
		mi: None,
		grid:None,
		layers:2,
		do_update_grid:Callable::invalid(),


		}
	}
	fn ready(&mut self,) {
		for c in self.base().get_children().iter_shared(){
			if let Ok(mi) = c.try_cast::<MeshInstance3D>(){
				self.mi = Some(mi);
				break;
			}
		}
		if self.mi.is_none() {
			let mi = MeshInstance3D::new_alloc();
			self.base_mut().add_child(&mi);
			self.mi = Some (mi);
		}
		self.do_update_grid = self.base().callable("update_grid");
	}
}
#[godot_api]
impl HexGridNode3D {
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
		let mut builder = SurfaceTool::new_gd();
		//variables to be set by object settings
		let tile_count = 7;
		//let layer_count = 1;
		let flat_north = true;
		//heightdata copied from grid object, or a placeholder of size equal to 'tile_count'.
		let heightdata = [5,4,4,2,2,2,2];
		//scale of tiles. Defaults to (1,0.1,1).
		let _scale = Vector3{x:1f32, y:0.1f32, z:1f32};

		//arrays with data
		let mut vertecies = PackedVector3Array::new();
		vertecies.resize(vertex_count);
		let mut indicies = PackedInt32Array::new();
		indicies.resize(index_count);
		
		//process each tile
		for i in 0..tile_count {
			//tile start index of vertex and indicies
			let vi_start = i * VERTS_PER_TILE;
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
			let center = Vector3{x:center.0, y:0f32, z: center.1};
			//For each corner (and each polygon/triangle)
			for c in 0..6 {
				//vertex indicies
				let v1 = vi_start + 1 + c;
                let v2 = vi_start + 1 + (c+1)%6;
				
				//calculate height for this corner. Average of corner's adjacent tiles.
                let h1 = n_heights[c];
                let h2 = n_heights[(c+1)%6];
                let h = (height as f32 + h1 + h2) / 3.0;

				//calculate vertex of corner
				//wip-todo...
			}


			for c in 0..6{
                let v1 = vi_start + 1 + c;
                let v2 = vi_start + 1 + (c+1)%6;
				let v = Self::get_corner(c, flat_north);

			}
		}
	}
	fn get_corner(c:usize, flat_north:bool) -> Vector3{
		let angle:i32 = 60 *c as i32 + {if flat_north {0} else {-30}};
		let angle:f32 = TAU / 360f32 * angle as f32;
		Vector3 { x: angle.cos(), y: 0.0, z: angle.sin() }
	}
	#[func]
	fn set_grid(&mut self, new_grid:Option<Gd<SpiralHexGrid>>){
		self.grid = new_grid;
		//TODO
	}
	#[func]
	fn set_layers(&mut self, value:i32){
		self.layers = value as u8;
		//TODO
	}

	#[func]
	fn update_grid(&mut self){
		//TODO
		godot_print!("UPDATE_GRID");
		if let Some(grid) = &self.grid {
			//grid.
		}
	}

}