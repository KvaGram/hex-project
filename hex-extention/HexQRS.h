#ifndef HEX_QRS_H
#define HEX_QRS_H

#include "object.hpp"
#include "HexLSP.h"
namespace godot {
class HexQRS : public Object {
    GDCLASS(HexQRS, Object)
    friend class HexLSP;
private:
	int q;
    int r;

protected:
	static void _bind_methods();

public:
	HexQRS();
    HexQRS(int q, int r);
    HexQRS(int q, int r, int s);
	~HexQRS();
    int get_q();
    int get_r();
    int get_s();
    void set_q(int _q);
    void set_r(int _r);
    HexQRS add(HexQRS other);
    HexQRS addm(HexQRS other, int times);
    HexQRS sub(HexQRS other);
    HexQRS subm(HexQRS other, int times);
    int get_layer();

    HexQRS* copy();
    static HexQRS* GET_D(int value);

    //static HexQRS* from_LSP(HexLSP other)
    static HexQRS* FROM_SPIRAL_INDEX(int index);
    static HexQRS* FROM_LSP(HexLSP* other);
    static HexLSP* TO_LSP(HexQRS* other){
        return HexLSP::FROM_QRS(other);
    }
};
}
#endif