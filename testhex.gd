extends Node3D

var orient:HexUtil.TileOrient = HexUtil.TileOrient.FLAT

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var testscenes= [
		"res://assets/tiles/Testtile1.tscn",
		"res://assets/tiles/Testtile2.tscn",
		"res://assets/tiles/Testtile3.tscn",
		"res://assets/tiles/Testtile4.tscn",
		"res://assets/tiles/Testtile2.tscn",
		"res://assets/tiles/Testtile3.tscn",
		"res://assets/tiles/Testtile4.tscn"
	]
	for i in 7:

		var off1 = 3.45
		var off2 = 1
		var chunk_spacing = sqrt(3) * 2.5
		var chunk:Node3D = Node3D.new()
		chunk.name = "chunk_%s"%[i]
		self.add_child(chunk)
		build_test_hexes(chunk, testscenes[i])
		var chunk_qrs = HexUtil.tile_index_to_QRS(i)
		var chunk_q = chunk_qrs[0]
		var chunk_r = chunk_qrs[1]
		if orient == HexUtil.TileOrient.POINT:
			chunk.position = Vector3(
			3.0/2.0 * (chunk_qrs[0]*4),
			0,
			(sqrt(3)/2) * (chunk_qrs[0]*4) + sqrt(3) * (chunk_qrs[1]*4)
			) + Vector3(
				sqrt(3)*chunk_qrs[0] + (sqrt(3)/2) * chunk_qrs[1],
				0,
				3.0/2.0 * chunk_qrs[1]
			)
		else:
			# Use pointy-topped positioning for chunks
			#chunk.position = Vector3(
				#sqrt(3) * chunk_spacing * chunk_q + (sqrt(3) / 2) * chunk_spacing * chunk_r,
				#0,
				#1.5 * chunk_spacing * chunk_r
			#)
			#chunk.position = Vector3(
				#1.5 * off1*chunk_qrs[0],
				#0,
				#(sqrt(3) / 2) * off1 + sqrt(3) * off1 * chunk_qrs[1]
			#)
			#chunk.position = Vector3(
				#sqrt(3)*(chunk_qrs[0]*off1) + (sqrt(3)/2) * (chunk_qrs[1]*off1),
				#0,
				#3.0/2.0 * (chunk_qrs[1]*off1)
				#) + Vector3(
				#3.0/2.0 * chunk_qrs[0]*off2,
				#0,
				#(sqrt(3)/2) * chunk_qrs[0]*off2 + sqrt(3) * chunk_qrs[1]*off2)
			chunk.position = Vector3(
				sqrt(3)*(chunk_qrs[0]*off1) + (sqrt(3)/2) * (chunk_qrs[1]*off1),
				0,
				3.0/2.0 * (chunk_qrs[1]*off1)
				) + Vector3(
				3.0/2.0 * chunk_qrs[0]*off2,
				0,
				(sqrt(3)/2) * chunk_qrs[0]*off2 + sqrt(3) * chunk_qrs[1]*off2)







func build_test_hexes(chunk:Node3D, hexscene:String):
	var tilescene:PackedScene = load(hexscene)
	for i in 19:#127:
		var node:Node3D = tilescene.instantiate()
		chunk.add_child(node)
		var qrs:PackedInt32Array = HexUtil.tile_index_to_QRS(i)
		node.ti = str(i)
		node.q = str(qrs[0])
		node.r = str(qrs[1])
		node.s = str(qrs[2])
		if orient == HexUtil.TileOrient.POINT:
			node.position = Vector3(sqrt(3)*qrs[0] + (sqrt(3)/2) * qrs[1], 0, 3.0/2.0 * qrs[1])
		else:
			node.rotation_degrees = Vector3(0, 30, 0)
			node.position = Vector3(3.0/2.0 * qrs[0], 0, (sqrt(3)/2) * qrs[0] + sqrt(3) * qrs[1])

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
