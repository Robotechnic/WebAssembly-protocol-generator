use crate::{protocol::Protocol, struct_::StructType, types::Types, Struct};
use std::{fs, io::Write};

const HEADER: &str = "#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include \"emscripten.h\"

#ifndef PROTOCOL_FUNCTION
#define PROTOCOL_FUNCTION __attribute__((import_module(\"typst_env\"))) extern
#endif

PROTOCOL_FUNCTION void wasm_minimal_protocol_send_result_to_host(const uint8_t *ptr, size_t len);
PROTOCOL_FUNCTION void wasm_minimal_protocol_write_args_to_buffer(uint8_t *ptr);


#define TYPST_INT_SIZE 4

#define INIT_BUFFER_UNPACK(buffer_len)                                                             \\
    size_t __buffer_offset = 0;                                                                    \\
    uint8_t *__input_buffer = malloc((buffer_len));                                                \\
    if (!__input_buffer) {                                                                         \\
        return 1;                                                                                  \\
    }                                                                                              \\
    wasm_minimal_protocol_write_args_to_buffer(__input_buffer);

#define CHECK_BUFFER()                                                                             \\
	if (__buffer_offset >= buffer_len) {                                                           \\
		return 2;                                                                                  \\
	}

#define NEXT_STR(dst)                                                                              \\
	CHECK_BUFFER()                                                                                 \\
    {                                                                                              \\
		if (__input_buffer[__buffer_offset] == '\\0') {                                            \\
			(dst) = malloc(1);                                                                     \\
			if (!(dst)) {                                                                          \\
				return 1;                                                                          \\
			}                                                                                      \\
			(dst)[0] = '\\0';                                                                      \\
			__buffer_offset++;                                                                     \\
		} else {                                                                                   \\
			int __str_len = strlen((char *)__input_buffer + __buffer_offset);                      \\
			(dst) = malloc(__str_len + 1);                                                         \\
			if (!(dst)) {                                                                          \\
				return 1;                                                                          \\
			}                                                                                      \\
			strcpy((dst), (char *)__input_buffer + __buffer_offset);                               \\
			__buffer_offset += __str_len + 1;                                                      \\
		}                                                                                          \\
    }

#define NEXT_INT(dst)                                                                              \\
	CHECK_BUFFER()                                                                                 \\
    (dst) = big_endian_decode(__input_buffer + __buffer_offset, TYPST_INT_SIZE);                   \\
    __buffer_offset += TYPST_INT_SIZE;

#define NEXT_CHAR(dst)                                                                             \\
	CHECK_BUFFER()                                                                                 \\
    (dst) = __input_buffer[__buffer_offset++];

#define NEXT_FLOAT(dst)                                                                            \\
	CHECK_BUFFER()                                                                                 \\
    (dst) = decode_float(__input_buffer + __buffer_offset);                                        \\
	__buffer_offset += TYPST_INT_SIZE;
    
#define FREE_BUFFER()                                                                              \\
    free(__input_buffer);                                                                          \\
    __input_buffer = NULL;

#define INIT_BUFFER_PACK(buffer_len)                                                               \\
    size_t __buffer_offset = 0;                                                                    \\
    uint8_t *__input_buffer = malloc((buffer_len));                                                \\
    if (!__input_buffer) {                                                                         \\
        return 1;                                                                                  \\
    }

#define FLOAT_PACK(fp)                                                                             \\
    {                                                                                              \\
		if (fp == 0.0f) {  																	       \\
			big_endian_encode(0, __input_buffer + __buffer_offset, TYPST_INT_SIZE);                \\
		} else {                                                                                   \\
			union FloatBuffer { 																   \\
				float f;   																	       \\
				int i;   																	       \\
			} __float_buffer;                                                                      \\
			__float_buffer.f = (fp);                                                               \\
			big_endian_encode(__float_buffer.i, __input_buffer + __buffer_offset, TYPST_INT_SIZE); \\
		}                                                                                          \\
		__buffer_offset += TYPST_INT_SIZE;                                                         \\
	}

#define INT_PACK(i)                                                                                \\
    big_endian_encode((i), __input_buffer + __buffer_offset, TYPST_INT_SIZE);                      \\
    __buffer_offset += TYPST_INT_SIZE;

#define CHAR_PACK(c)                                                                               \\
    __input_buffer[__buffer_offset++] = (c);

#define STR_PACK(s)                                                                                \\
    if (s == NULL || s[0] == '\\0') {                                                              \\
        __input_buffer[__buffer_offset++] = '\\0';                                                 \\
    } else {                                                                                       \\
        strcpy((char *)__input_buffer + __buffer_offset, (s));                                     \\
        size_t __str_len = strlen((s));                                                            \\
        __input_buffer[__buffer_offset + __str_len] = '\\0';                                       \\
        __buffer_offset += __str_len + 1;                                                          \\
    }
";

const C: &str = "#include \"protocol.h\"
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

";

/// Write the header of the generated .h file
fn generate_header(h_file: &mut fs::File) -> Result<(), std::io::Error> {
    h_file.write(HEADER.as_bytes())?;
    Ok(())
}

/// Write the footer of the generated .h file
fn generate_footer(h_file: &mut fs::File) -> Result<(), std::io::Error> {
    h_file.write(b"#endif\n")?;
    Ok(())
}

/// Write a struct definition in the generated .h file
fn generate_struct(h_file: &mut fs::File, name: &str, s: &Struct) -> Result<(), std::io::Error> {
    h_file.write(format!("typedef struct {}_t {{\n", name).as_bytes())?;
    for field in s.iter() {
        h_file.write(format!("    {} {};\n", field.1.to_c(true), field.0).as_bytes())?;
		if let Types::Array(_) = field.1 {
            h_file.write(format!("    size_t {}_len;\n", field.0).as_bytes())?;
        }
    }
    h_file.write(b"} ")?;
    h_file.write(format!("{};\n", name).as_bytes())?;
    Ok(())
}

/// Write a struct free function signature
fn generate_struct_free_signature(
	file: &mut fs::File,
	name: &str,
) -> Result<(), std::io::Error> {
	file.write(format!("void free_{}({} *s)", name, name).as_bytes())?;
	Ok(())
}

/// Check if a type needs to be freed
fn need_free(t: &Types) -> bool {
	match t {
		Types::String => true,
		Types::Struct(_) => true,
		Types::Array(t) => need_free(t.as_ref()),
		_ => false,
	}
}

/// Write a struct free function body in the generated .c file
fn generate_struct_field_free_body(
	c_file: &mut fs::File,
	field_name: &str,
	t: &Types,
) -> Result<(), std::io::Error> {
	match t {
		Types::String => {
			c_file.write(format!("    if (s->{}) {{\n", field_name).as_bytes())?;
			c_file.write(format!("        free(s->{});\n", field_name).as_bytes())?;
			c_file.write(b"    }\n")?;
		}
		Types::Struct(_) => {
			c_file.write(format!("    free_{}(&s->{});\n", t.to_c(false), field_name).as_bytes())?;
		}
		Types::Array(t) => {
			if need_free(t.as_ref()) {
				c_file.write(format!("    for (size_t i = 0; i < s->{}_len; i++) {{\n", field_name).as_bytes())?;
				generate_struct_field_free_body(c_file, &format!("{}[i]", field_name), t.as_ref())?;
				c_file.write(b"    }\n")?;
			}
			c_file.write(format!("    free(s->{});\n", field_name).as_bytes())?;
		}
		_ => {}
	}
	Ok(())
}

/// Write a struct free function in the generated .c file
fn generate_struct_free(
	c_file: &mut fs::File,
	name: &str,
	s: &Struct
) -> Result<(), std::io::Error> {
	generate_struct_free_signature(c_file, name)?;
	c_file.write(b" {\n")?;
	for field in s.iter() {
		generate_struct_field_free_body(c_file, &field.0, &field.1)?;
	}
	c_file.write(b"}\n")?;
	Ok(())
}

/// Write a struct decode function signature
fn generate_struct_decode_signature(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    if let StructType::Struct = s.get_type() {
        file.write(
            format!(
                "int decode_{}(uint8_t *__input_buffer, size_t buffer_len, {} *out, size_t *buffer_offset)",
                name, name
            )
            .as_bytes(),
        )?;
    } else {
        file.write(format!("int decode_{}(size_t buffer_len, {} *out)", name, name).as_bytes())?;
    }
    Ok(())
}

/// Write a line to decode a field in a struct
fn generate_struct_decode_line(
	file: &mut fs::File,
	field_name: &str,
	t: &Types,
) -> Result<(), std::io::Error> {
	match t {
		Types::Int => {
			file.write(format!("    NEXT_INT(out->{})\n", field_name).as_bytes())?;
		}
		Types::Float | Types::Point => {
			file.write(format!("    NEXT_FLOAT(out->{})\n", field_name).as_bytes())?;
		}
		Types::String => {
			file.write(format!("    NEXT_STR(out->{})\n", field_name).as_bytes())?;
		}
		Types::Bool | Types::Char => {
			file.write(format!("    NEXT_CHAR(out->{})\n", field_name).as_bytes())?;
		}
		Types::Struct(name) => {
			file.write(format!("    if ((err = decode_{}(__input_buffer + __buffer_offset, buffer_len - __buffer_offset, &out->{}, &__buffer_offset))){{return err;}}\n", name, field_name).as_bytes())?;
		}
		Types::Array(t) => {
			file.write(format!("    NEXT_INT(out->{}_len)\n", field_name).as_bytes())?;
			file.write(format!("    if (out->{}_len == 0) {{\n        out->{} = NULL;\n    }} else {{\n", field_name, field_name).as_bytes())?;
			file.write(format!("        out->{} = malloc(out->{}_len * sizeof({}));\n", field_name, field_name, t.to_c(false)).as_bytes())?;
			file.write(format!("        if (!out->{}){{\n            return 1;\n        }}\n", field_name).as_bytes())?;
			file.write(format!("        for (size_t i = 0; i < out->{}_len; i++) {{\n", field_name).as_bytes())?;
			generate_struct_decode_line(file, &format!("{}[i]", field_name), t)?;
			file.write(b"        }\n")?;
			file.write(b"    }\n")?;
		}
	}
	Ok(())
}

/// Write a struct decode function body in the generated .c file
fn generate_struct_decode_function(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
    free_buffer: bool,
) -> Result<(), std::io::Error> {
    generate_struct_decode_signature(file, name, s)?;
    file.write(b" {\n")?;
    if let StructType::Struct = s.get_type() {
        file.write(b"    size_t __buffer_offset = 0;\n")?;
    } else {
        file.write(b"    INIT_BUFFER_UNPACK(buffer_len)\n")?;
    }
	file.write(b"    int err;\n    (void)err;\n")?;
    for field in s.iter() {
		generate_struct_decode_line(file, &field.0, &field.1)?;
    }
    if free_buffer {
        file.write(b"    FREE_BUFFER()\n")?;
    }
	if let StructType::Struct = s.get_type() {
		file.write(b"    *buffer_offset += __buffer_offset;\n")?;
	}
    file.write(b"    return 0;\n")?;
    file.write(b"}\n")?;
    Ok(())
}

/// Write a struct decode function in the generated .c file and its signature in the generated .h file
fn generate_struct_decode(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    let protocol = if let StructType::Protocol(_) = s.get_type() {
        generate_struct_decode_signature(h_file, name, s)?;
        h_file.write(b";\n")?;
        true
    } else {
        false
    };
    generate_struct_decode_function(c_file, name, s, protocol)?;
    Ok(())
}

/// Write a size function signature, used to calculate the size of a struct
fn generate_size_function_signature(
    c_file: &mut fs::File,
    name: &str,
) -> Result<(), std::io::Error> {
    c_file.write(format!("size_t {}_size(const void *s)", name).as_bytes())?;
    Ok(())
}

/// Write a line to calculate the size of a field in a struct
fn generate_type_size(
    file: &mut fs::File,
	name: &str,
    t: &Types,
    field_name: &str,
) -> Result<(), std::io::Error> {
    match t {
        Types::Int | Types::Float | Types::Point=> {
            file.write(b"TYPST_INT_SIZE")?;
        }
        Types::Bool | Types::Char => {
            file.write(b"1")?;
        }
        Types::String => {
            file.write(format!("string_size((({}*)s)->{})",name, field_name).as_bytes())?;
        }
        Types::Struct(_) => {
            file.write(format!("{}_size((void*)&(({}*)s)->{})", t.to_c(false), name, field_name).as_bytes())?;
        }
        Types::Array(t) => {
			if let Types::String = t.as_ref() {
				file.write(
					format!(
						"TYPST_INT_SIZE + string_list_size((({}*)s)->{}, (({}*)s)->{}_len)",
						name, field_name, name, field_name
					)
					.as_bytes(),
				)?;
			} else {
				file.write(
					format!(
						"TYPST_INT_SIZE + list_size((({}*)s)->{}, (({}*)s)->{}_len, ",
						name, field_name, name, field_name
					)
					.as_bytes(),
				)?;
				match t.as_ref() {
					Types::Int | Types::Float | Types::Point => {
						file.write(b"int_size")?;
					}
					Types::Bool | Types::Char => {
						file.write(b"char_size")?;
					}
					Types::String => {
						unreachable!("Array of strings are special cases");
					}
					Types::Struct(name) => {
						file.write(format!("{}_size", name).as_bytes())?;
					}
					Types::Array(_) => {
						unimplemented!("Array of arrays not supported");
					}
				}
				file.write(format!(", sizeof(*(({}*)s)->{}))", name, field_name).as_bytes())?;
			}
        }
    }
    Ok(())
}

/// Write a size function in the generated .c file
fn generate_size_function(
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_size_function_signature(c_file, name)?;
    c_file.write(b"{\n")?;
    c_file.write(b"\treturn ")?;
    let mut first = true;
    for field in s.iter() {
        if !first {
            c_file.write(b" + ")?;
        }
        first = false;
        generate_type_size(c_file, name,  &field.1, &field.0)?;
    }
    c_file.write(b";\n}\n")?;
    Ok(())
}

/// Write a struct encode function signature
fn generate_struct_encode_signature(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    if let StructType::Struct = s.get_type() {
        file.write(
            format!(
                "int encode_{}(const {} *s, uint8_t *__input_buffer, size_t *buffer_len, size_t *buffer_offset)",
                name, name
            )
            .as_bytes(),
        )?;
    } else {
        file.write(format!("int encode_{}(const {} *s)", name, name).as_bytes())?;
    }
    Ok(())
}

/// Write a line to encode a field in a struct
fn generate_struct_encode_function_encode_line(
    file: &mut fs::File,
    field_name: &str,
    t: &Types,
    is_struct: bool,
) -> Result<(), std::io::Error> {
    match t {
        Types::Int => {
            file.write(format!("    INT_PACK(s->{})\n", field_name).as_bytes())?;
        }
        Types::Float | Types::Point => {
            file.write(format!("    FLOAT_PACK(s->{})\n", field_name).as_bytes())?;
        }
        Types::String => {
            file.write(format!("    STR_PACK(s->{})\n", field_name).as_bytes())?;
        }
        Types::Bool | Types::Char => {
            file.write(format!("    CHAR_PACK(s->{})\n", field_name).as_bytes())?;
        }
        Types::Struct(name) => {
            file.write(format!("        if ((err = encode_{}(&s->{}, __input_buffer + __buffer_offset, {}buffer_len, &__buffer_offset))) {{\n", name, field_name, (if is_struct { "" } else { "&" })).as_bytes())?;
            file.write(b"            return err;\n")?;
            file.write(b"        }\n")?;
        }
        Types::Array(t) => {
            file.write(format!("    INT_PACK(s->{}_len)\n", field_name).as_bytes())?;
            file.write(
                format!("    for (size_t i = 0; i < s->{}_len; i++) {{\n", field_name).as_bytes(),
            )?;
            match t.as_ref() {
                Types::Array(_) => {
                    unreachable!("Array of arrays not supported");
                }
                _ => {
                    generate_struct_encode_function_encode_line(
                        file,
                        &format!("{}[i]", field_name),
                        t,
                        is_struct,
                    )?;
                }
            }
			file.write(b"    }\n")?;
        }
    }
    Ok(())
}

/// Write a struct encode function in the generated .c file
fn generate_struct_encode_function(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_struct_encode_signature(file, name, s)?;
    file.write(b" {\n")?;
    if let StructType::Struct = s.get_type() {
        file.write(b"    size_t __buffer_offset = 0;")?;
        file.write(format!("    size_t s_size = {}_size(s);\n", name).as_bytes())?;
        file.write(b"    if (s_size > *buffer_len) {\n")?;
        file.write(b"        return 2;\n")?;
        file.write(b"    }\n")?;
    } else {
        file.write(format!("    size_t buffer_len = {}_size(s);\n", name).as_bytes())?;
        file.write(b"    INIT_BUFFER_PACK(buffer_len)\n")?;
    }
	file.write(b"    int err;\n	(void)err;\n")?;

    for field in s.iter() {
        generate_struct_encode_function_encode_line(
            file,
            &field.0,
            &field.1,
            if let StructType::Struct = s.get_type() {
                true
            } else {
                false
            },
        )?;
    }
    if let StructType::Struct = s.get_type() {
        file.write(b"\n    *buffer_offset += __buffer_offset;")?;
    } else {
        file.write(
            b"\n    wasm_minimal_protocol_send_result_to_host(__input_buffer, buffer_len);",
        )?;
    }
    file.write(b"\n    return 0;\n}\n")?;
    Ok(())
}

/// Write a struct encode function in the generated .c file and its signature in the generated .h file
/// It also writes the size function in the generated .c file
fn generate_struct_encode(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    if let StructType::Protocol(_) = s.get_type() {
        generate_struct_encode_signature(h_file, name, s)?;
        h_file.write(b";\n")?;
    }

    generate_size_function(c_file, name, s)?;
    generate_struct_encode_function(c_file, name, s)?;
    Ok(())
}

/// Generate all the necessary code for a struct in the generated .h and .c files
fn generate(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_struct(h_file, name, s)?;
	generate_struct_free_signature(h_file, name)?;
	h_file.write(b";\n")?;
	generate_struct_free(c_file, name, s)?;
    if s.decoder {
        generate_struct_decode(h_file, c_file, name, s)?;
    }
    if s.encoder {
        generate_struct_encode(h_file, c_file, name, s)?;
    }
    h_file.write(b"\n")?;
    Ok(())
}

/// Generate the protocol in the given path
pub fn generate_protocol(path: &str, p: &Protocol) -> Result<(), std::io::Error> {
    let c_path = format!("{}/protocol.c", path);
    let h_path = format!("{}/protocol.h", path);
    let mut c_file = fs::File::create(c_path)?;
    let mut h_file = fs::File::create(h_path)?;
    generate_header(&mut h_file)?;
    c_file.write(C.as_bytes())?;
    for (name, s) in p.ordered_structs() {
        generate(&mut h_file, &mut c_file, name, s)?;
    }
    for (name, s) in p.protocols() {
        generate(&mut h_file, &mut c_file, name, s)?;
    }
    generate_footer(&mut h_file)?;
    Ok(())
}
