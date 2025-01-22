extends Object
class_name HexUtil

enum TileOrient {FLAT, POINT}

const _HEXAGON_DIRECTIONS:Array[Array] = [
	[0, -1, +1], #F - UP, P - UP-LEFT
	[+1, -1, 0], # UP-RIGHT
	[+1, 0, -1], #F - DOWN-RIGHT, P - RIGHT
	[0, +1, -1], #F - DOWN, P - DOWN-RIGHT
	[-1, +1, 0], # DOWN-LEFT
	[-1, 0, +1], # F - UP-LEFT, P - LEFT
]
static func get_dir(d:int)->Array:
	return _HEXAGON_DIRECTIONS[d%6]
## Enumnames for neigbours in a hexagon. Values match indicies in dirs.
## Names are pair-wise interchangable, matching flat then pointy orientation.
## in the names, the primary direction goes first.
enum hexdir {
	UP=0,			UP_LEFT= 0 ,	#0
	RIGHT_UP=1,		UP_RIGHT = 1,	#1
	RIGHT_DOWN= 2, 	RIGHT= 2,		#2
	DOWN = 3, 		DOWN_RIGHT = 3,	#3
	LEFT_DOWN = 4,	DOWN_LEFT = 4,	#4
	LEFT_UP = 5,	LEFT = 5,		#5
}
## Returns a readable technical name for the hexagon's direction
static func get_direction_name(d:int, ori:TileOrient = TileOrient.FLAT)->String:
	d = d * 2 + ori
	if d < 12:
		return hexdir.values()[d]
	return "INVALID"

##translating a spiral tile index to a QRS coordinate system where each layer begins at Q = 0, R = -layer and S = layer
static func tile_index_to_QRS(ti:int) -> PackedInt32Array:
	if ti <= 0:
		return PackedInt32Array([0, 0, 0])
	# Layer math refined with assistance from AI.
	# this replaces iterative code, adding the number of segments from each previus layer
	var layer: int = ceil((sqrt(12 * ti + 9) - 3) / 6)
	var count: int = 3 * layer * (layer - 1) + 1  # Count of tiles in all previous layers

	#clockwise posision arond the layer circle, defined with segment then clockwise position from segment orign
	var segment: int = floor((ti-count) / layer) #segment of the layer (0 to 5)
	var pos: int = (ti-count) % layer #segment posision

	var q = get_dir(segment)[0] * layer + get_dir(segment+2)[0]*pos
	var r = get_dir(segment)[1] * layer + get_dir(segment+2)[1]*pos
	var s = get_dir(segment)[2] * layer + get_dir(segment+2)[2]*pos

	return PackedInt32Array([q, r, s])

func QRS_to_segposlayer(QRS:PackedInt32Array)->PackedInt32Array:
	var q:=QRS[0]
	var r:=QRS[1]
	var s:=QRS[2]
	var seg:int
	var pos:int
	var layer:int =max(abs(q), abs(r), abs(s))
	#There is a borderland overlap at the segment starts.
	#The ternary expressions takes on these edge cases.
	if r == -layer:
		seg = 0 if (q != layer) else 1
		pos = q if (q != layer) else 0
	elif q == layer:
		seg = 1 if (s != -layer) else 2
		pos = -s if (s != -layer) else 0
	elif s == -layer:
		seg = 2 if (r !=  layer) else 3
		pos = r if (r !=  layer) else 0
	elif r == layer:
		seg = 3 if (q != -layer) else 4
		pos = -q if (q != -layer) else 0
	elif q == -layer:
		seg = 4 if (s !=  layer) else 5
		pos = s if (s !=  layer) else 0
	elif s == layer:
		seg = 5 if (r != -layer) else 0
		pos = -r if (r != -layer) else 0
	return PackedInt32Array([seg, pos, layer])

func chunk_to_global(cube_QRS:PackedInt32Array, layers:int)->PackedInt32Array:
	var q:=0
	var r:=0
	var s:=0
	# Find sub-segment and segment posision
	return PackedInt32Array([q,r,s])

static func QRS_to_tile_index(qrs:PackedInt32Array) -> int:
	var q:int = qrs[0]
	var r:int = qrs[1]
	var s:int = qrs[2]
	var layer:int = max(abs(q), abs(r), abs(s))
	if layer == 0:
		return 0
	var count: int = 3 * layer * (layer - 1) + 1  # Count of tiles in all previous layers

	#find the segment
	var seg = 0
	var pos = 0

	#There is a borderland overlap at the segment starts. Only segment 0 can even get a posision 0.
	#This is fine. Adding an exception rule to each may not save much if anything on calculations.
	if r == -layer:
		seg = 0
		pos = q
	elif q == layer:
		seg = 1
		pos = -s
	elif s == -layer:
		seg = 2
		pos = r
	elif r == layer:
		seg = 3
		pos = -q
	elif q == -layer:
		seg = 4
		pos = s
	elif s == layer:
		seg = 5
		pos = -r
	return seg * layer + count + pos


	return 0
