extends RefCounted
class_name HexChunk
var data:Array[HexTileData]

func get_array_size() -> int:
	var r = HexSettings.get_chunk_size()
	var size = 3*r*(r+1) +1
	return size

func build():
	var ti = 0 #tile index in chunk
	var q = 0 #Q-coordinate in chunk
	var r = 0 #R-coordinate in chunk
	var s = 0 #S-coordinate in chunk
	for i in HexSettings.get_chunk_size():

		pass
