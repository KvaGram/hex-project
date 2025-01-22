extends Object
class_name HexSettings
const _CHUNK_SIZE:=  6
const _REGION_SIZE:= 6
const _TILE_ORIENT:= HexUtil.TileOrient.FLAT



static func get_chunk_size() -> int:
	return _CHUNK_SIZE
static func get_region_size() -> int:
	return _REGION_SIZE
static func get_tile_orient() -> HexUtil.TileOrient:
	return _TILE_ORIENT
