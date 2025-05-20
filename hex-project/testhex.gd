extends Node3D

var orient:HexUtil.TileOrient = HexUtil.TileOrient.FLAT
var testhex:SpiralHexGrid;
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	#testhex = SpiralHexGrid.new();
	#var layers:int = 10;
	#var size:int = (3 * (layers+1) * layers) + 1;
	#var testmap:Image;
	var t0:int = Time.get_ticks_msec();
	var t1:int;
	var t2:int;
	var t3:int;
	#var mesh:ArrayMesh;

	#testmap = Image.load_from_file("res://assets/maps/iceland_heightmap.png");
	#testmap.decompress();
	t1 = Time.get_ticks_msec();
	#testhex.from_hightmap(testmap);
	t2 = Time.get_ticks_msec();
	var mesh = SpiralHexMesh.new();
	#mesh.set_grid(testhex)
	mesh.set_layers(2);
	mesh.regenerate();
	t3 = Time.get_ticks_msec();
	print("loading & decompress time: " + str(float(t1 - t0)/1000) + " seconds");
	print("mapping time: " + str(float(t2 - t1)/1000) + " seconds");
	print("mesh time: "  + str(float(t3 - t2)/1000) + " seconds");
	print("total time: " + str(float(t3 - t0)/1000) + " seconds");
	
	#diagnose the mesh
	
	var aabb = mesh.get_aabb();
	print("→ Mesh AABB position = ", aabb.position)
	print("→ Mesh AABB size     = ", aabb.size)
	
	
	var arrs = mesh.surface_get_arrays(0)
	var verts = arrs[Mesh.ARRAY_VERTEX];
	var inds = arrs[Mesh.ARRAY_INDEX];
	var cols = arrs[Mesh.ARRAY_COLOR];
	print("→ FINAL VERTEX COUNT = %d" % verts.size())
	print("→ FINAL INDEX  COUNT = %d" % inds.size())
	print("→ FINAL COLOR  COUNT = %d" % cols.size())
	
	if verts.size() >= 14:
		var start_v = verts.size() - 14
		print("\n— last 14 VERTICES (2 centers + 12 corners) —")
		for i in range(start_v, verts.size()):
			print("   VERT[%2d] = %s" % [i, verts[i]])
	else:
		push_error("Not enough vertices to dump tail!")

	if inds.size() >= 36:
		var start_i = inds.size() - 36
		print("\n— last 36 INDICES (12 triangles) —")
		for i in range(start_i, inds.size()):
			print("   IDX[%3d] = %d" % [i, inds[i]])
	else:
		push_error("Not enough indices to dump tail!")
	
	var meshRender = MeshInstance3D.new();
	var material = StandardMaterial3D.new();
	material.vertex_color_use_as_albedo = true;
	meshRender.material_override = material;
	add_child(meshRender);
	meshRender.mesh = mesh;
	meshRender.set_base(mesh.get_rid())
	print(meshRender.get_base())
		
	#var verts:PackedVector3Array = testhex.test_draw_hex(true);
	#var height:PackedByteArray = testhex.get_heightdata();
	##print(verts);
	#var draw:Draw3D = Draw3D.new();
	#add_child(draw);
	#for i in size:
		#var todraw:Array = Array(verts.slice(i*7, (i+1)*7));
		#todraw.append(verts.get(i*7+1))
		#draw.draw_line_loop(todraw, draw.random_color())
#
		#pass

	var testscenes:Array[PackedScene] = [
		load("res://assets/tiles/Testtile1.tscn"),
		load("res://assets/tiles/Testtile2.tscn"),
		load("res://assets/tiles/Testtile3.tscn"),
		load("res://assets/tiles/Testtile4.tscn"),
	]
	#bulid_test_chunks(2, 1, testscenes)
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

	#%Freecam3D.look_at_from_position(Vector3(0, 8, 3), Vector3.ZERO)
	#call_deferred("test_coords")

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
		var chunk_QRS:PackedInt32Array = HexUtil.tile_index_to_QRS(ci)
		var global_QRS:PackedInt32Array = HexUtil.chunk_to_global(chunk_QRS, tile_layers)
		var chunk:Hex_Chunk = Hex_Chunk.new(tile_layers, ci, chunk_QRS[0], chunk_QRS[1], chunk_QRS[2])
		self.add_child(chunk)
		chunk.name = "Chunk_%3s (%2s, %2s, %2s)" % [ci, chunk_QRS[0], chunk_QRS[1], chunk_QRS[2]]
		if orient == HexUtil.TileOrient.POINT:
			chunk.position = Vector3(sqrt(3)*global_QRS[0] + (sqrt(3)/2) * global_QRS[1], 0, 3.0/2.0 * global_QRS[1])
		else:
			chunk.position = Vector3(3.0/2.0 * global_QRS[0], 0, (sqrt(3)/2) * global_QRS[0] + sqrt(3) * global_QRS[1])

		build_test_hexes(chunk, tile_layers, hexscenes[ci%4])
		chunk.checkNeighbors.connect(checkNeighbors)
		pass
	pass
func checkNeighbors(tile_layers:int, chunk_q:int, chunk_r:int, q:int, r:int):
	var chunk_s = -chunk_q - chunk_r
	var s = -q-r
	var neighbors:Array = []

	for d in 6:
		var n =HexUtil.get_dir(d).duplicate()
		var nq = n[0]+q
		var nr = n[1]+r
		var ns = n[2]+s
		var ncq = chunk_q
		var ncr = chunk_r
		var ncs = chunk_s
		##test out of bounds
		#1 find tile's global coords.
		#2 find tile's local coords in adjacent chunk
		#3 apply direction to get neighbour

		#If neighbor is within chunk borders, skip to next.
		if (abs(nq)>tile_layers or abs(nr)>tile_layers or abs(ns)>tile_layers):
			var n_global := HexUtil.chunk_to_global(PackedInt32Array([chunk_q, chunk_r, chunk_s]), tile_layers)
			n_global[0]+=nq; n_global[1]+=nr; n_global[2]+=ns
			var n_dir:int = -1
			match(d):
				0: n_dir= d if q <= 0 else d+1
				1: n_dir= d if s >= 0 else d+1
				2: n_dir= d if r <= 0 else d+1
				3: n_dir= d if q >= 0 else d+1
				4: n_dir= d if s <= 0 else d+1
				5: n_dir= d if r >= 0 else d+1
			ncq += HexUtil.get_dir(n_dir)[0]
			ncr += HexUtil.get_dir(n_dir)[1]
			ncs += HexUtil.get_dir(n_dir)[2]
			if(ncq + ncr + ncs) != 0:
				printerr("Bad chunk coodinates")
			var n_qrs = HexUtil.tile_to_other_chunk(PackedInt32Array([nq, nr, ns]),PackedInt32Array([chunk_q, chunk_r, chunk_s]),PackedInt32Array([ncq, ncr, ncs]), tile_layers)
			nq = n_qrs[0]
			nr = n_qrs[1]
			ns = n_qrs[2]
		neighbors.append([nq, nr, ns, ncq, ncr, ncs])
		print(neighbors[d])
	pass
	print()


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
