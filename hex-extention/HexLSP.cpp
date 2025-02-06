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

HexLSP *godot::HexLSP::FROM_QRS(HexQRS* other)
{
    int lay = other->get_layer();
    int seg, pos;
    if (lay == 0)
        return new HexLSP(0, 0, 0);
    if         (other->r == -lay){
        seg = (other->q != lay) ? 0 : 1;
        pos = (other->q != lay) ?  other->q       : 0;
    } else if  (other->q == lay){
        seg = (other->get_s() != -lay) ? 1 : 2;
        pos = (other->get_s() != -lay) ? -other->get_s() : 0;
    } else if  (other->get_s() == -lay){
        seg = (other->r != lay) ? 2 : 3;
        pos = (other->r != lay) ?  other->r       : 0;
    } else if  (other->r == lay) {
        seg = (other->q != lay) ? 3 : 4;
        pos = (other->q != lay) ? -other->q       : 0;
    } else if  (other->q == -lay) {
        seg = (other->get_s() != lay) ? 4 : 5;
        pos = (other->get_s() != lay) ?  other->get_s() : 0;
    } else if  (other->get_s() == lay) {
        seg = (other->r != -lay) ? 5 : 0;
        pos = (other->r != -lay) ? -other->r       : 0;
    } else {
        seg = 0;
        pos = 0;
    }
    return new HexLSP(lay, seg, pos);
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

//TODO: check math with testing
int godot::HexLSP::TO_SPIRAL_INDEX(HexLSP* other)
{
    if (other->layer <= 0){
        return 0;
    }
    return 3 * other->layer * (other->layer-1) + other->posision + other->segment*other->layer + 1;
}

int godot::HexLSP::get_layer()
{
    return layer;
}

int godot::HexLSP::get_segment()
{
    return segment;
}

int godot::HexLSP::get_posision()
{
    return posision;
}

void godot::HexLSP::set_layer(int lay)
{
    if (posision != 0 && lay > 0 && layer > 0) {
        posision = floor(posision * (float)layer / (float)lay);
    }
    layer = max(0, lay);
    if (layer == 0){
        posision = 0;
        segment = 0;
    }
}

void godot::HexLSP::set_segment(int seg)
{
    if (layer <= 0){
        segment = 0;
        return;
    }
    segment = seg % 6;
}

void godot::HexLSP::set_posision(int pos)
{
    //maintain center tile exception
    if (layer <= 0){
        posision = 0;
        return;
    }
    posision += pos;

    set_segment(segment + floor(posision / layer));
    posision = posision % layer;
}

HexLSP *godot::HexLSP::copy()
{
    return this->copy();
}

void godot::HexLSP::rotate(int steps)
{
    set_posision(posision + steps);
}
