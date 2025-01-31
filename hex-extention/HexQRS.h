#ifndef HEX_QRS_H
#define HEX_QRS_H
#include <godot_cpp/classes/object.hpp>
namespace godot {
class HexQRS : public Object {
    GDCLASS(HexQRS, Object)
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
    int get_s();
    HexQRS add(HexQRS other);
    HexQRS add(HexQRS other, int times);
    HexQRS sub(HexQRS other);
    HexQRS sub(HexQRS other, int times);
    HexQRS* copy();
};
}
#endif