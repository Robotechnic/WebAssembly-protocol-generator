#include "protocol.h"
int big_endian_decode(uint8_t const *buffer, int size){
    int value = 0;
    for (int i = 0; i < size; i++) {
        value |= buffer[i] << (8 * (size - i - 1));
    }
    return value;
}

void big_endian_encode(int value, uint8_t *buffer, int size) {
    for (int i = 0; i < sizeof(int); i++) {
        buffer[i] = (value >> (8 * (sizeof(int) - i - 1))) & 0xFF;
    }
}

size_t list_size(void *list, size_t size, size_t (*sf)(const void*), size_t element_size) {
    size_t result = 0;
    for (int i = 0; i < size; i++) {
        result += sf(list + i * element_size);
    }
    return result;
}

size_t int_size(const void* elem) {
    return TYPST_INT_SIZE;
}
size_t float_size(const void *elem) {
    return TYPST_INT_SIZE;
}
size_t bool_size(const void *elem) {
    return TYPST_INT_SIZE;
}
size_t char_size(const void *elem) {
    return 1;
}
size_t string_size(const void *elem) {
    return strlen((char *)elem) + 1;
}

size_t Number_size(const void *vs){
    const Number *s = vs;
return TYPST_INT_SIZE + TYPST_INT_SIZE + strlen(s->romanRepresentation) + 1 + 1 + 1;
}
int pack_Number(const Number *s, uint8_t *__input_buffer, size_t *buffer_len, size_t *buffer_offset) {
    size_t __buffer_offset = 0;    size_t s_size = Number_size(s);
    if (s_size > *buffer_len) {
        return 1;
    }
    FLOAT_PACK(s->half)
    INT_PACK(s->closestInt)
    STR_PACK(s->romanRepresentation)
    CHAR_PACK(s->isNegative)
    CHAR_PACK(s->isOdd)

    *buffer_offset += __buffer_offset;
    return 0;
}
int unpack_askNumber(size_t buffer_len, askNumber *out) {
    INIT_BUFFER_UNPACK(buffer_len)
    NEXT_INT(out->numberCount)
    FREE_BUFFER()
    return 0;
}
size_t result_size(const void *vs){
    const result *s = vs;
return TYPST_INT_SIZE + list_size((void*)s->numbers, s->numbers_len, Number_size, sizeof(*s->numbers));
}
int pack_result(const result *s) {
    size_t buffer_len = result_size(s);
    INIT_BUFFER_PACK(buffer_len)
    INT_PACK(s->numbers_len)
    for (size_t i = 0; i < s->numbers_len; i++) {
    if (pack_Number(&s->numbers[i], __input_buffer + __buffer_offset, &buffer_len, &__buffer_offset)) {
        return 1;
    }
    }

    wasm_minimal_protocol_send_result_to_host(__input_buffer, buffer_len);
    return 0;
}
