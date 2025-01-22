extends Node3D

var orient:HexUtil.TileOrient = HexUtil.TileOrient.FLAT

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var testscenes:Array[PackedScene] = [
		load("res://assets/tiles/Testtile1.tscn"),
		load("res://assets/tiles/Testtile2.tscn"),
		load("res://assets/tiles/Testtile3.tscn"),
		load("res://assets/tiles/Testtile4.tscn"),
	]
	bulid_test_chunks(3, 4, testscenes)
	#for i in 7:
		#var off1 = 3.45
		#var off2 = 1
		#var chunk_spacing = sqrt(3) * 2.5
		#var chunk:Node3D = Node3D.new()
		#chunk.name = "chunk_%s"%[i]
		#self.add_child(chunk)
		##build_test_hexes(chunk, testscenes[i])
		#var chunk_qrs = HexUtil.tile_index_to_QRS(i)
		#var chunk_q = chunk_qrs[0]
		#var chunk_r = chunk_qrs[1]
		#if orient == HexUtil.TileOrient.POINT:
			#chunk.position = Vector3(
			#3.0/2.0 * (chunk_qrs[0]*4),
			#0,
			#(sqrt(3)/2) * (chunk_qrs[0]*4) + sqrt(3) * (chunk_qrs[1]*4)
			#) + Vector3(
				#sqrt(3)*chunk_qrs[0] + (sqrt(3)/2) * chunk_qrs[1],
				#0,
				#3.0/2.0 * chunk_qrs[1]
			#)
		#else:
			## Use pointy-topped positioning for chunks
			##chunk.position = Vector3(
				##sqrt(3) * chunk_spacing * chunk_q + (sqrt(3) / 2) * chunk_spacing * chunk_r,
				##0,
				##1.5 * chunk_spacing * chunk_r
			##)
			##chunk.position = Vector3(
				##1.5 * off1*chunk_qrs[0],
				##0,
				##(sqrt(3) / 2) * off1 + sqrt(3) * off1 * chunk_qrs[1]
			##)
			##chunk.position = Vector3(
				##sqrt(3)*(chunk_qrs[0]*off1) + (sqrt(3)/2) * (chunk_qrs[1]*off1),
				##0,
				##3.0/2.0 * (chunk_qrs[1]*off1)
				##) + Vector3(
				##3.0/2.0 * chunk_qrs[0]*off2,
				##0,
				##(sqrt(3)/2) * chunk_qrs[0]*off2 + sqrt(3) * chunk_qrs[1]*off2)
			#chunk.position = Vector3(
				#sqrt(3)*(chunk_qrs[0]*off1) + (sqrt(3)/2) * (chunk_qrs[1]*off1),
				#0,
				#3.0/2.0 * (chunk_qrs[1]*off1)
				#) + Vector3(
				#3.0/2.0 * chunk_qrs[0]*off2,
				#0,
				#(sqrt(3)/2) * chunk_qrs[0]*off2 + sqrt(3) * chunk_qrs[1]*off2)

	%Freecam3D.look_at_from_position(Vector3(0, 8, 3), Vector3.ZERO)
	call_deferred("test_coords")

func test_coords():
	print("Testing ti to QRS")
	test_ti_to_qrs(0, 0, 0, 0)
	test_ti_to_qrs(1, 0, -1, 1)
	test_ti_to_qrs(8, 1, -2, 1)
	test_ti_to_qrs(11, 2, 0, -2)
	test_ti_to_qrs(30, -2, 3, -1)
	test_ti_to_qrs(48, 1, 3, -4)
	test_ti_to_qrs(49, 0, 4, -4)
	test_ti_to_qrs(84, -5, 2, 3)
	print("testing QRS to ti")
	test_qrs_to_ti(0, 0, 0, 0)
	test_qrs_to_ti(1, 0, -1, 1)
	test_qrs_to_ti(8, 1, -2, 1)
	test_qrs_to_ti(11, 2, 0, -2)
	test_qrs_to_ti(30, -2, 3, -1)
	test_qrs_to_ti(48, 1, 3, -4)
	test_qrs_to_ti(49, 0, 4, -4)
	test_qrs_to_ti(84, -5, 2, 3)






func build_test_hexes(chunk:Node3D, tile_layers:int, hexscene:PackedScene):
	var n = 3 * (tile_layers+1) * tile_layers + 1
	for ti in n:
		var node:Node3D = hexscene.instantiate()
		chunk.add_child(node)
		var qrs:PackedInt32Array = HexUtil.tile_index_to_QRS(ti)
		node.ti = str(ti)
		node.q = str(qrs[0])
		node.r = str(qrs[1])
		node.s = str(qrs[2])
		if orient == HexUtil.TileOrient.POINT:
			node.position = Vector3(sqrt(3)*qrs[0] + (sqrt(3)/2) * qrs[1], 0, 3.0/2.0 * qrs[1])
		else:
			node.rotation_degrees = Vector3(0, 30, 0)
			node.position = Vector3(3.0/2.0 * qrs[0], 0, (sqrt(3)/2) * qrs[0] + sqrt(3) * qrs[1])
		node.name = "Node_%3s (%2s, %2s, %2s)" % [ti, qrs[0], qrs[1], qrs[2]]
	pass
func bulid_test_chunks(tile_layers:int, chunk_layers:int, hexscenes:Array[PackedScene]):
	var n = 3 * (chunk_layers+1) * chunk_layers + 1
	#var n = 9
	for ci in n:
		var chunk:Node3D = Node3D.new()
		self.add_child(chunk)
		var chunk_QRS:PackedInt32Array = HexUtil.tile_index_to_QRS(ci)
		var global_QRS:PackedInt32Array = HexUtil.chunk_to_global(chunk_QRS, tile_layers)
		chunk.name = "Chunk_%3s (%2s, %2s, %2s)" % [ci, chunk_QRS[0], chunk_QRS[1], chunk_QRS[2]]
		if orient == HexUtil.TileOrient.POINT:
			chunk.position = Vector3(sqrt(3)*global_QRS[0] + (sqrt(3)/2) * global_QRS[1], 0, 3.0/2.0 * global_QRS[1])
		else:
			chunk.position = Vector3(3.0/2.0 * global_QRS[0], 0, (sqrt(3)/2) * global_QRS[0] + sqrt(3) * global_QRS[1])

		build_test_hexes(chunk, tile_layers, hexscenes[ci%4])

		pass
	pass


func test_ti_to_qrs(ti:int, q:int, r:int, s:int)->bool:
	var result:PackedInt32Array = HexUtil.tile_index_to_QRS(ti)
	if (result[0] == q and result[1] == r and result[2] == s):
		print("test for %3s passed."%[ti])
		return true
	printerr("Test for %3s failed. result (Q %3s == q %3s: %3s) (R %3s == r %3s: %3s) (S %3s == s %3s: %3s)" % [ti, result[0], q, result[0] == q, result[1], r, result[1] == r, result[2], s, result[2] == s])
	return false

func test_qrs_to_ti(ti:int, q:int, r:int, s:int)->bool:
	var result:int = HexUtil.QRS_to_tile_index(PackedInt32Array([q, r, s]))
	if (result == ti):
		print("test for (Q %3s R %3s S %3s ) passed."%[q, r, s])
		return true
	printerr("Test for (Q %3s R %3s S %3s )  failed. result TI %3s == ti %3s: %s"%[q, r, s, result, ti, result == ti])
	return false


## Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta: float) -> void:
	#pass
