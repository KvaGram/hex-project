#include "HexQRS.h"
#include <godot_cpp/core/object.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <cmath>
using namespace std;
using namespace godot;

void HexQRS::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_q"), &HexQRS::get_q);
    ClassDB::bind_method(D_METHOD("get_r"), &HexQRS::get_r);
    ClassDB::bind_method(D_METHOD("get_s"), &HexQRS::get_s);

    ClassDB::bind_method(D_METHOD("copy"), &HexQRS::copy);

    ClassDB::bind_method(D_METHOD("set_q", "q"), &HexQRS::set_q);
    ClassDB::bind_method(D_METHOD("set_r", "r"), &HexQRS::set_r);

    ClassDB::bind_method(D_METHOD("add", "other"), &HexQRS::add);
    ClassDB::bind_method(D_METHOD("addm", "other", "times"), &HexQRS::addm);
    ClassDB::bind_method(D_METHOD("sub", "other"), &HexQRS::sub);
    ClassDB::bind_method(D_METHOD("subm", "other", "times"), &HexQRS::subm);

    ADD_PROPERTY(PropertyInfo(Variant::INT, "q"), "get_q", "set_q");
    ADD_PROPERTY(PropertyInfo(Variant::INT, "r"), "get_r", "set_r");
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
int HexQRS::get_q(){return q;}
int HexQRS::get_r(){return r;}
int HexQRS::get_s(){return -q-r;}
void HexQRS::set_q(int _q){q = _q;}
void HexQRS::set_r(int _r){r = _r;}
HexQRS HexQRS::add(HexQRS other) {
    HexQRS ret = HexQRS();
    ret.q = q + other.q;
    ret.r = r + other.r;
    return ret;
}
HexQRS HexQRS::addm(HexQRS other, int times) {
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
HexQRS HexQRS::subm(HexQRS other, int times) {
    HexQRS ret = HexQRS();
    ret.q = q - other.q*times;
    ret.r = r - other.r*times;
    return ret;
}
int godot::HexQRS::get_layer()
{
    return max(abs(get_s()), max(abs(q), abs(r)));
}
HexQRS *HexQRS::copy()
{
    return new HexQRS(q, r);
}
HexQRS* HexQRS::GET_D(int value){
    switch (value % 6){
        case 0:
            return new HexQRS(0,-1);
            break;
        case 1:
            return new HexQRS(1,-1);
            break;
        case 2:
            return new HexQRS(+1,0);
            break;
        case 3:
            return new HexQRS(0,+1);
            break;
        case 4:
            return new HexQRS(-1,1);
            break;
        case 5:
            return new HexQRS(-1,0);
            break;
    }
    return new HexQRS(0,0);
}

HexQRS *godot::HexQRS::FROM_SPIRAL_INDEX(int index)
{
    return FROM_LSP(HexLSP::FROM_SPIRAL_INDEX(index));
}

HexQRS *godot::HexQRS::FROM_LSP(HexLSP *other)
{
    HexQRS* ret = new HexQRS();
    ret->addm(*GET_D(other->segment), other->layer).addm(*GET_D(other->segment+2), other->posision);
    return ret;
}
