#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include "emscripten.h"

#define PROTOCOL_FUNCTION __attribute__((import_module("typst_env"))) extern

PROTOCOL_FUNCTION void wasm_minimal_protocol_send_result_to_host(const uint8_t *ptr, size_t len);
PROTOCOL_FUNCTION void wasm_minimal_protocol_write_args_to_buffer(uint8_t *ptr);

union FloatBuffer {
    float f;
    int i;
};

void big_endian_encode(int value, uint8_t *buffer, int size);

int big_endian_decode(uint8_t const *buffer, int size);

#define TYPST_INT_SIZE 4

#define INIT_BUFFER_UNPACK(buffer_len)                                                             \
    size_t __buffer_offset = 0;                                                                    \
    uint8_t *__input_buffer = malloc((buffer_len));                                                \
    if (!__input_buffer) {                                                                         \
        return 1;                                                                                  \
    }                                                                                              \
    wasm_minimal_protocol_write_args_to_buffer(__input_buffer);

#define CHECK_BUFFER()                                                                             \
	if (__buffer_offset >= buffer_len) {                                                           \
		return 2;                                                                                  \
	}

#define NEXT_STR(dst)                                                                              \
	CHECK_BUFFER()                                                                                 \
    {                                                                                              \
		if (__input_buffer[__buffer_offset] == '\0') {                                            \
			(dst) = malloc(1);                                                                     \
			if (!(dst)) {                                                                          \
				return 1;                                                                          \
			}                                                                                      \
			(dst)[0] = '\0';                                                                      \
			__buffer_offset++;                                                                     \
		} else {                                                                                   \
			int __str_len = strlen((char *)__input_buffer + __buffer_offset);                      \
			(dst) = malloc(__str_len + 1);                                                         \
			if (!(dst)) {                                                                          \
				return 1;                                                                          \
			}                                                                                      \
			strcpy((dst), (char *)__input_buffer + __buffer_offset);                               \
			__buffer_offset += __str_len + 1;                                                      \
		}                                                                                          \
    }

#define NEXT_INT(dst)                                                                              \
	CHECK_BUFFER()                                                                                 \
    (dst) = big_endian_decode(__input_buffer + __buffer_offset, TYPST_INT_SIZE);                   \
    __buffer_offset += TYPST_INT_SIZE;

#define NEXT_CHAR(dst)                                                                             \
	CHECK_BUFFER()                                                                                 \
    (dst) = __input_buffer[__buffer_offset++];

#define NEXT_FLOAT(dst)                                                                            \
	CHECK_BUFFER()                                                                                 \
    {                                                                                              \
        int __encoded_value;                                                                       \
        NEXT_INT(__encoded_value);                                                                 \
        union FloatBuffer __float_buffer;                                                          \
        __float_buffer.i = __encoded_value;                                                        \
        (dst) = __float_buffer.f;                                                                  \
    }
    
#define FREE_BUFFER()                                                                              \
    free(__input_buffer);                                                                          \
    __input_buffer = NULL;

#define INIT_BUFFER_PACK(buffer_len)                                                               \
    size_t __buffer_offset = 0;                                                                    \
    uint8_t *__input_buffer = malloc((buffer_len));                                                \
    if (!__input_buffer) {                                                                         \
        return 1;                                                                                  \
    }

#define FLOAT_PACK(fp)                                                                             \
    {                                                                                              \
        union FloatBuffer __float_buffer;                                                          \
        __float_buffer.f = (fp);                                                                    \
        big_endian_encode(__float_buffer.i, __input_buffer + __buffer_offset, TYPST_INT_SIZE);     \
        __buffer_offset += TYPST_INT_SIZE;                                                         \
    }

#define INT_PACK(i)                                                                                \
    big_endian_encode((i), __input_buffer + __buffer_offset, TYPST_INT_SIZE);                      \
    __buffer_offset += TYPST_INT_SIZE;

#define CHAR_PACK(c)                                                                               \
    __input_buffer[__buffer_offset++] = (c);

#define STR_PACK(s)                                                                                \
    if (s == NULL || s[0] == '\0') {                                                               \
        __input_buffer[__buffer_offset++] = '\0';                                                 \
    } else {                                                                                       \
        strcpy((char *)__input_buffer + __buffer_offset, (s));                                     \
        size_t __str_len = strlen((s));                                                            \
        __input_buffer[__buffer_offset + __str_len] = '\0';                                       \
        __buffer_offset += __str_len + 1;                                                          \
    }
typedef struct {
    float half;
    int closestInt;
    char* romanRepresentation;
    bool isNegative;
    bool isOdd;
} Number;
void free_Number(Number *s);

typedef struct {
    int decimal;
} decimalResult;
void free_decimalResult(decimalResult *s);
int encode_decimalResult(const decimalResult *s);

typedef struct {
    int numberCount;
} askNumber;
void free_askNumber(askNumber *s);
int decode_askNumber(size_t buffer_len, askNumber *out);

typedef struct {
    char* roman;
} toDecimal;
void free_toDecimal(toDecimal *s);
int decode_toDecimal(size_t buffer_len, toDecimal *out);

typedef struct {
    Number * numbers;
    size_t numbers_len;
} result;
void free_result(result *s);
int encode_result(const result *s);

#endif
