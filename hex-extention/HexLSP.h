#ifndef HEX_LSP_H;
#define HEX_LSP_H;
#include <HexQRS.h>;
#include <godot_cpp/classes/object.hpp>;
namespace godot {
/* HexLSP - Hexagon layer, segment, posision
This is a local coordinate system defining the location of a hexagon tile in relation to a defined center.
Layer is number of tiles away, distance, from center. [0-]
The layer has a ring around the center, split into 6 segments [0 to 5]
Posision is the clockwise distance from the corner/origin of each segment [0-layer]

LSP may be used as a temporary coordinate system for varius uses, alongside QRS as a regular coordinate system
*/ 
class HexLSP : public Object {
    GDCLASS(HexLSP, Object);
    friend class HexQRS;
    private:
        int layer, segment, posision;
    public:
        ~HexLSP();
        HexLSP(int lay, int seg, int pos);
        static HexLSP* FROM_QRS(HexQRS* other);
        static HexLSP* FROM_SPIRAL_INDEX(int index);
        static int TO_SPIRAL_INDEX(HexLSP* other);
        static HexQRS* TO_QRS(HexLSP* other){
            return HexQRS::FROM_LSP(other);
        };
        int get_layer();
        int get_segment();
        int get_posision();
        /// @brief Sets layer of LSP coordinate. WARNING! Will adjust posision if posision is not zero.
        /// @param lay 
        void set_layer(int lay);
        /// @brief Sets segment of LSP cordinate. Also convinient for 1/6 rotations of hexagons.
        /// @param seg 
        void set_segment(int seg);
        /// @brief Sets posision from segment direction. Limited to [0-layer]. If set outside this limit, segment will be adjusted.
        /// @param pos 
        void set_posision(int pos);
        HexLSP* copy();
        /// @brief Shortcut for adding to posision
        /// @param steps 
        void rotate(int steps);

};
};
#endif