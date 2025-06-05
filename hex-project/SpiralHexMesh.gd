@tool
extends Mesh
class_name SpiralHexMesh
@export var grid:SpiralHexGrid
var rid:RID


func _set(property: StringName, value: Variant) -> bool:
	if property == "grid":
		pass
	return false
	
func _init() -> void:
	rid = RenderingServer.mesh_create()
	
	
