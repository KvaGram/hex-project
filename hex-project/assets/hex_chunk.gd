extends StaticBody3D
class_name Hex_Chunk
var chunk_q:int
var chunk_r:int
var chunk_s:int
var chunk_index:int

var _tile_layers:int

signal checkNeighbors(layers:int, c_q:int, c_r:int, t_q:int, t_r:int)

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
func _init(layers:int, ci:int, c_q:int, c_r:int, c_s:int) -> void:
	_tile_layers = layers
	var coll:CollisionShape3D = CollisionShape3D.new()
	var cyl:CylinderShape3D = CylinderShape3D.new()
	cyl.radius = sqrt(3) * layers + 1
	cyl.height = 0.2
	coll.shape = cyl
	self.add_child(coll)
	self.input_event.connect(_on_input_event)
	chunk_index = ci
	chunk_q = c_q
	chunk_r = c_r
	chunk_s = c_s



func _on_input_event(camera: Node, event: InputEvent, event_position: Vector3, normal: Vector3, shape_idx: int) -> void:
	if not event is InputEventMouseButton:
		return
	if not (event as InputEventMouseButton).is_pressed():
		return
	var pos:Vector3 = self.to_local(event_position)
	var ori:HexUtil.TileOrient = HexUtil.TileOrient.FLAT
	var q:int
	var r:int
	var s:int
	if ori == HexUtil.TileOrient.POINT:
		q = round( sqrt(3)/3 * pos.x - 1.0/3.0 * pos.z )
		r = round( 2.0/3.0 * pos.z )
		#function pixel_to_pointy_hex(point):
		#var q = (sqrt(3)/3 * point.x  -  1./3 * point.y) / size
		#var r = (                        2./3 * point.y) / size
		#return axial_round(Hex(q, r))
		pass
	else:
		q = round( 2./3 * pos.x )
		r = round(-1./3 * pos.x  +  sqrt(3)/3 * pos.z)
		#function pixel_to_flat_hex(point):
		#var q = ( 2./3 * point.x                        ) / size
		#var r = (-1./3 * point.x  +  sqrt(3)/3 * point.y) / size
		#return axial_round(Hex(q, r))
		pass
	s = -q - r
	checkNeighbors.emit(_tile_layers, chunk_q, chunk_r, q, r)
	#print(pos.x," ",pos.y)
	#print(q," ", r," ", s)
