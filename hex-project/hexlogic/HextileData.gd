extends RefCounted
class_name HexTileData

var _i:int = 0
var _qrs:PackedFloat32Array
var q:int:
	get:
		return _qrs[0]
var r:int:
	get:
		return _qrs[1]
var s:int:
	get:
		return _qrs[2]
var ti:int:
	get:
		return _i


func validateQRS():
	return q + r + s == 0

func _init(qrs:PackedFloat32Array, ti:int):
	_i = ti
	_qrs = qrs
	if not validateQRS():
		printerr("INVALID QRS")
