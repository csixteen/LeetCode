// https://leetcode.com/problems/reverse-bits/
//
// For future reference:
// g++ -Wall -g --std=c++17 reverse-bits.cpp -o main -lstdc++

#include <cstdint>
#include <iostream>

class Solution {
public:
    uint32_t reverseBits(uint32_t n) {
        uint32_t acc = 0;

        for (uint32_t i = 0; i < 32; i++) {
            acc |= ((1 << i) & n) > 0 ? 1 << (31 - i) : 0;
        }

        return acc;
    }
};

int main() {
    std::cout << "Hello, world!\n";
}
