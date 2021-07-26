//
// Created by Ervin Bosenbacher on 26/07/2021.
//

#include "doubler.h"
#include <iostream>

extern const int FACTOR;
extern "C" {
    int doublercpp(int x) {
        std::cout << "doubler function runs... \n";
        return x * FACTOR;
    }
}