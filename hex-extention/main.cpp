/** @file main.cpp
 * Runs a test on the HexQRS module
 */

// #include "HexLSP.h"
#include "HexQRS.h"

#include <stdio.h>
using namespace std;
// using namespace godot;

/**
 * @brief Runs unit tests for QRS base functions
 * @return number of failed tests
 */
int test1();

int main(int argc, char* argv[])
{
	test1();
    if (argc)
    {
        
    }
    return 0;
}

// Implementation
int test1() {
	printf("Hello World");
	return 42; 
	}