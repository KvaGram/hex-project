#include "HexLSP.h"
#include <cmath>
using namespace std;
using namespace godot;
HexLSP::~HexLSP()
{
}

HexLSP::HexLSP(int lay, int seg, int pos)
{
    layer = lay;
    segment = seg;
    posision = pos;
}

/// @brief Generates layer segment posision coordinates for a hexagon tile given an index.
/// @param index a non-zero index for a hexagonal tile
/// @return A LSP coordinate object for given index
HexLSP* HexLSP::FROM_SPIRAL_INDEX(int index)
{
    if(index < 0) //foolproofing. A negative index is illigal. 
        index = abs(index);
    //math calculating the layer. Don't ask me how the math works. Had an AI to work it out for me.
    //It was derived from some calculus I barely understood. Yet, it is tested, and it works.
    int l = ceil((sqrt(12 * index + 9)-3)/6);
    //count the number of tiles below the layer. Required to find segment and posision.
    int c = 3 * l * (l-1) + 1;
    //finds the segment around the hex layer. range [0-5]
    int s = floor((index - c)/l);
    //finding the posision is as simple as using the reminder of the above math (using modulo)
    int p = (index-c) % l;
    return new HexLSP(l, s, p);
}

HexLSP *godot::HexLSP::copy()
{
    return this->copy();
}
