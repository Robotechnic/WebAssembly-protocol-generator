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

float decode_float(uint8_t *buffer) {
	int value = big_endian_decode(buffer, TYPST_INT_SIZE);
	if (value == 0) {
		return 0.0f;
	}
	union FloatBuffer {
		float f;
		int i;
	} float_buffer;
	float_buffer.i = value;
	return float_buffer.f;
}

void encode_float(float value, uint8_t *buffer) {
	if (value == 0.0f) {
		big_endian_encode(0, buffer, TYPST_INT_SIZE);
	} else {
		union FloatBuffer {
			float f;
			int i;
		} float_buffer;
		float_buffer.f = value;
		big_endian_encode(float_buffer.i, buffer, TYPST_INT_SIZE);
	}
}

size_t list_size(void *list, size_t size, size_function sf, size_t element_size) {
    size_t result = 0;
    for (int i = 0; i < size; i++) {
        result += sf(list + i * element_size);
    }
    return result;
}

size_t optional_size(void *opt, size_function sf) {
    return 1 + (opt ? sf(opt) : 0);
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
    if (!elem || !((char *)elem)[0]) {
        return 1;
    }
    return strlen((char *)elem) + 1;
}
size_t string_list_size(char **list, size_t size) {
	size_t result = 0;
	for (size_t i = 0; i < size; i++) {
		result += string_size(list[i]);
	}
	return result;
}

void free_B(B *s) {
    for (size_t i = 0; i < s->b_len; i++) {
    free_B(&s->b[i]);
    }
    free(s->b);
}
int decode_B(uint8_t *__input_buffer, size_t buffer_len, B *out, size_t *buffer_offset) {
    size_t __buffer_offset = 0;
    int err;
    (void)err;
    NEXT_INT(out->b_len)
    if (out->b_len == 0) {
        out->b = NULL;
    } else {
        out->b = malloc(out->b_len * sizeof(B));
        if (!out->b){
            return 1;
        }
        for (size_t i = 0; i < out->b_len; i++) {
    if ((err = decode_B(__input_buffer + __buffer_offset, buffer_len - __buffer_offset, &out->b[i], &__buffer_offset))){return err;}
        }
    }
    NEXT_CHAR(out->bidule)
    *buffer_offset += __buffer_offset;
    return 0;
}
size_t B_size(const void *s){
	return TYPST_INT_SIZE + list_size(((B*)s)->b, ((B*)s)->b_len, B_size, sizeof(*((B*)s)->b)) + 1;
}
int encode_B(const B *s, uint8_t *__input_buffer, size_t *buffer_len, size_t *buffer_offset) {
    size_t __buffer_offset = 0;    size_t s_size = B_size(s);
    if (s_size > *buffer_len) {
        return 2;
    }
    int err;
	(void)err;
    INT_PACK(s->b_len)
    for (size_t i = 0; i < s->b_len; i++) {
        if ((err = encode_B(&s->b[i], __input_buffer + __buffer_offset, buffer_len, &__buffer_offset))) {
            return err;
        }
    }
    CHAR_PACK(s->bidule)

    *buffer_offset += __buffer_offset;
    return 0;
}
void free_Test(Test *s) {
    for (size_t i = 0; i < s->b_len; i++) {
    free_B(&s->b[i]);
    }
    free(s->b);
}
int decode_Test(size_t buffer_len, Test *out) {
    INIT_BUFFER_UNPACK(buffer_len)
    int err;
    (void)err;
    NEXT_INT(out->b_len)
    if (out->b_len == 0) {
        out->b = NULL;
    } else {
        out->b = malloc(out->b_len * sizeof(B));
        if (!out->b){
            return 1;
        }
        for (size_t i = 0; i < out->b_len; i++) {
    if ((err = decode_B(__input_buffer + __buffer_offset, buffer_len - __buffer_offset, &out->b[i], &__buffer_offset))){return err;}
        }
    }
    FREE_BUFFER()
    return 0;
}
void free_Essses(Essses *s) {
    for (size_t i = 0; i < s->b_len; i++) {
    free_B(&s->b[i]);
    }
    free(s->b);
}
size_t Essses_size(const void *s){
	return TYPST_INT_SIZE + list_size(((Essses*)s)->b, ((Essses*)s)->b_len, B_size, sizeof(*((Essses*)s)->b)) + optional_size(((Essses*)s)->test, string_size);
}
int encode_Essses(const Essses *s) {
    size_t buffer_len = Essses_size(s);
    INIT_BUFFER_PACK(buffer_len)
    int err;
	(void)err;
    INT_PACK(s->b_len)
    for (size_t i = 0; i < s->b_len; i++) {
        if ((err = encode_B(&s->b[i], __input_buffer + __buffer_offset, &buffer_len, &__buffer_offset))) {
            return err;
        }
    }
    CHAR_PACK(s->test != NULL)
    if (s->test) {
    STR_PACK(s->test[0])
    }

    wasm_minimal_protocol_send_result_to_host(__input_buffer, buffer_len);
    return 0;
}
