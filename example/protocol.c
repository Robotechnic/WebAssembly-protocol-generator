#include "protocol.h"
int big_endian_decode(uint8_t const *buffer, int size){
    int value = 0;
    for (int i = 0; i < size; i++) {
        value |= buffer[i] << (8 * (size - i - 1));
    }
    return value;
}

int big_endian_encode(int value, uint8_t *buffer, int size) {
    for (int i = 0; i < sizeof(int); i++) {
        buffer[i] = (value >> (8 * (sizeof(int) - i - 1))) & 0xFF;
    }
}
