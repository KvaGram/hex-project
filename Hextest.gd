extends Node


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	DebugConsole.add_command(
		"makehex",
		makehex,
		self,
		[
			DebugCommand.Parameter.new("radius", DebugCommand.ParameterType.Int),
			DebugCommand.Parameter.new("id", DebugCommand.ParameterType.String)
		 ],
		"Make a test hexagon chunk"
	)
	DebugConsole.add_command("test_ti_to_qrs", test_ti_to_qrs, self, [
		DebugCommand.Parameter.new("tile index", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("Q", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("R", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("S", DebugCommand.ParameterType.Int),
	])
	DebugConsole.add_command("test_qrs_to_ti", test_qrs_to_ti, self, [
		DebugCommand.Parameter.new("tile index", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("Q", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("R", DebugCommand.ParameterType.Int),
		DebugCommand.Parameter.new("S", DebugCommand.ParameterType.Int),
	])
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func makehex(radius:int, id:String):
	print(radius, id)
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
