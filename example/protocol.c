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

void free_Number(Number *s) {
    if (s->romanRepresentation) {
        free(s->romanRepresentation);
    }
}
size_t Number_size(const void *s){
	return TYPST_INT_SIZE + TYPST_INT_SIZE + strlen(((Number*)s)->romanRepresentation) + 1 + 1 + 1;
}
int encode_Number(const Number *s, uint8_t *__input_buffer, size_t *buffer_len, size_t *buffer_offset) {
    size_t __buffer_offset = 0;    size_t s_size = Number_size(s);
    if (s_size > *buffer_len) {
        return 2;
    }
    int err;
	(void)err;
    FLOAT_PACK(s->half)
    INT_PACK(s->closestInt)
    STR_PACK(s->romanRepresentation)
    CHAR_PACK(s->isNegative)
    CHAR_PACK(s->isOdd)

    *buffer_offset += __buffer_offset;
    return 0;
}
void free_decimalResult(decimalResult *s) {
}
size_t decimalResult_size(const void *s){
	return TYPST_INT_SIZE;
}
int encode_decimalResult(const decimalResult *s) {
    size_t buffer_len = decimalResult_size(s);
    INIT_BUFFER_PACK(buffer_len)
    int err;
	(void)err;
    INT_PACK(s->decimal)

    wasm_minimal_protocol_send_result_to_host(__input_buffer, buffer_len);
    return 0;
}
void free_result(result *s) {
    for (size_t i = 0; i < s->numbers_len; i++) {
    free_Number(&s->numbers[i]);
    }
    free(s->numbers);
}
size_t result_size(const void *s){
	return TYPST_INT_SIZE + list_size(((result*)s)->numbers, ((result*)s)->numbers_len, Number_size, sizeof(*((result*)s)->numbers));
}
int encode_result(const result *s) {
    size_t buffer_len = result_size(s);
    INIT_BUFFER_PACK(buffer_len)
    int err;
	(void)err;
    INT_PACK(s->numbers_len)
    for (size_t i = 0; i < s->numbers_len; i++) {
        if ((err = encode_Number(&s->numbers[i], __input_buffer + __buffer_offset, &buffer_len, &__buffer_offset))) {
            return err;
        }
    }

    wasm_minimal_protocol_send_result_to_host(__input_buffer, buffer_len);
    return 0;
}
void free_askNumber(askNumber *s) {
}
int decode_askNumber(size_t buffer_len, askNumber *out) {
    INIT_BUFFER_UNPACK(buffer_len)
    int err;
    (void)err;
    NEXT_INT(out->numberCount)
    FREE_BUFFER()
    return 0;
}
void free_toDecimal(toDecimal *s) {
    if (s->roman) {
        free(s->roman);
    }
}
int decode_toDecimal(size_t buffer_len, toDecimal *out) {
    INIT_BUFFER_UNPACK(buffer_len)
    int err;
    (void)err;
    NEXT_STR(out->roman)
    FREE_BUFFER()
    return 0;
}
