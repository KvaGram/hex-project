#include "HexQRS.h"
#include <godot_cpp/core/object.hpp>

using namespace godot;

void HexQRS::_bind_methods() {
}
HexQRS::HexQRS() {
	// Initialize any variables here.
	q = 0;
    r = 0;
}
HexQRS::HexQRS(int _q, int _r) {
    q = _q;
    r = _r;
}
HexQRS::HexQRS(int _q, int _r, int _s) {
    q = _q;
    r = _r;
    // optionally add in a test to confirm s is correct
    // assert q + r + s == 0
    // else invalid
}
HexQRS::~HexQRS() {
	// Add your cleanup here.
    // delete q;
    // delete r;
}
int HexQRS::get_s() {
    return -q-r;
}
HexQRS HexQRS::add(HexQRS other) {
    HexQRS ret = HexQRS();
    ret.q = q + other.q;
    ret.r = r + other.r;
    return ret;
}
HexQRS HexQRS::add(HexQRS other, int times) {
    HexQRS ret = HexQRS();
    ret.q = q + other.q*times;
    ret.r = r + other.r*times;
    return ret;
}
HexQRS HexQRS::sub(HexQRS other) {
    HexQRS ret = HexQRS();
    ret.q = q - other.q;
    ret.r = r - other.r;
    return ret;
}
HexQRS HexQRS::sub(HexQRS other, int times) {
    HexQRS ret = HexQRS();
    ret.q = q - other.q*times;
    ret.r = r - other.r*times;
    return ret;
}
HexQRS* HexQRS::copy() {
    return new HexQRS(q, r);
}


