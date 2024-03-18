use crate::{protocol::Protocol, struct_::StructType, types::Types, Struct};
use std::{fs, io::Write};

const HEADER: &str = "#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include \"emscripten.h\"

#define PROTOCOL_FUNCTION __attribute__((import_module(\"typst_env\"))) extern

PROTOCOL_FUNCTION void wasm_minimal_protocol_send_result_to_host(const uint8_t *ptr, size_t len);
PROTOCOL_FUNCTION void wasm_minimal_protocol_write_args_to_buffer(uint8_t *ptr);

union FloatBuffer {
	float f;
	int i;
};

void big_endian_encode(uint8_t *buffer, int value);

int big_endian_decode(uint8_t const *buffer, int size);

#define TYPST_INT_SIZE 4

#define INIT_BUFFER_UNPACK(buffer_len)                                                             \\
    size_t __buffer_offset = 0;                                                                    \\
    uint8_t *__input_buffer = malloc((buffer_len));                                                \\
    if (!__input_buffer) {                                                                         \\
        return 1;                                                                                  \\
    }                                                                                              \\
    wasm_minimal_protocol_write_args_to_buffer(__input_buffer);

#define NEXT_STR(dst)                                                                              \\
    {                                                                                              \\
        int __str_len = strlen((char *)__input_buffer + __buffer_offset);                          \\
        (dst) = malloc(__str_len + 1);                                                             \\
        strcpy((dst), (char *)__input_buffer + __buffer_offset);                                   \\
        __buffer_offset += __str_len + 1;                                                          \\
    }

#define NEXT_INT(dst)                                                                              \\
    (dst) = big_endian_decode(__input_buffer + __buffer_offset, TYPST_INT_SIZE);                   \\
    __buffer_offset += TYPST_INT_SIZE;

#define NEXT_CHAR(dst)                                                                             \\
	(dst) = __input_buffer[__buffer_offset++];

#define NEXT_FLOAT(dst)                                                                            \\
	{                                                                                              \\
		int __encoded_value;                                                                       \\
		NEXT_INT(__encoded_value);                                                                 \\
		union FloatBuffer __float_buffer;                                                          \\
		__float_buffer.i = __encoded_value;                                                        \\
		(dst) = __float_buffer.f;                                                                  \\
	}
	
#define FREE_BUFFER()                                                                              \\
    free(__input_buffer);                                                                          \\
    __input_buffer = NULL;

#define INIT_BUFFER_PACK(buffer, buffer_len)                                                       \\
	size_t __buffer_offset = 0;                                                                    \\
	uint8_t *__input_buffer = malloc((buffer_len));                                                \\
	buffer = __input_buffer;                                                                       \\
	if (!__input_buffer) {                                                                         \\
		return 1;                                                                                  \\
	}

#define FLOAT_PACK(f)                                                                              \\
	{                                                                                              \\
		union FloatBuffer __float_buffer;                                                          \\
		__float_buffer.f = (f);                                                                    \\
		big_endian_encode(__float_buffer.i, __input_buffer + __buffer_offset, TYPST_INT_SIZE);     \\
		__buffer_offset += TYPST_INT_SIZE;                                                         \\
	}

#define INT_PACK(i)                                                                                \\
	big_endian_encode((i), __input_buffer + __buffer_offset, TYPST_INT_SIZE);                      \\
	__buffer_offset += TYPST_INT_SIZE;

#define CHAR_PACK(c)                                                                               \\
	__input_buffer[__buffer_offset++] = (c);

#define STR_PACK(s)                                                                                \\
	strcpy((char *)__input_buffer + __buffer_offset, (s));                                         \\
	__input_buffer[__buffer_offset + strlen((s))] = '\\0';                                         \\
	__buffer_offset += strlen((char *)__input_buffer + __buffer_offset) + 1;
";

const C: &str = "#include \"protocol.h\"
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
";

fn generate_header(h_file: &mut fs::File) -> Result<(), std::io::Error> {
    h_file.write(HEADER.as_bytes())?;
    Ok(())
}

fn generate_footer(h_file: &mut fs::File) -> Result<(), std::io::Error> {
    h_file.write(b"#endif\n")?;
    Ok(())
}

fn generate_struct(h_file: &mut fs::File, name: &str, s: &Struct) -> Result<(), std::io::Error> {
    h_file.write(b"typedef struct {\n")?;
    for field in s.fields() {
        h_file.write(format!("    {} {};\n", field.1.to_c(), field.0).as_bytes())?;
        if let Types::Array(_) = field.1 {
            h_file.write(format!("    size_t {}_len;\n", field.0).as_bytes())?;
        }
    }
    h_file.write(b"} ")?;
    h_file.write(format!("{};\n", name).as_bytes())?;
    Ok(())
}

fn generate_struct_deserialisation_signature(
    file: &mut fs::File,
    name: &str,
) -> Result<(), std::io::Error> {
    file.write(
        format!(
            "{} unpack_{}(uint8_t *buffer, size_t buffer_len)",
            name, name
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn generate_struct_deserialisation_function(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
    free_buffer: bool,
) -> Result<(), std::io::Error> {
    generate_struct_deserialisation_signature(file, name)?;
    file.write(b" {\n")?;
    if let StructType::Struct = s.get_type() {
        file.write(b"	size_t __buffer_offset = 0;\n")?;
    } else {
        file.write(b"    INIT_BUFFER_UNPACK(buffer_len)\n")?;
    }
    file.write(format!("    {} result;\n", name).as_bytes())?;
    for field in s.fields() {
        println!("{:?}", field);
        match field.1 {
            Types::Bool | Types::Int => {
                file.write(format!("    NEXT_INT(result.{})\n", field.0).as_bytes())?;
            }
            Types::Float => {
                file.write(format!("    NEXT_FLOAT(result.{})\n", field.0).as_bytes())?;
            }
            Types::String => {
                file.write(format!("    NEXT_STR(result.{})\n", field.0).as_bytes())?;
            }
            Types::Char => {
                file.write(format!("    NEXT_CHAR(result.{})\n", field.0).as_bytes())?;
            }
            Types::Struct(name) => {
                file.write(format!("    result.{} = unpack_{}(__input_buffer + __buffer_offset, buffer_len - __buffer_offset);\n", field.0, name).as_bytes())?;
            }
            Types::Array(t) => {
                file.write(format!("    NEXT_INT(result.{}_len)\n", field.0).as_bytes())?;
                file.write(
                    format!(
                        "    result.{} = malloc(result.{}_len * sizeof({}));\n",
                        field.0,
                        field.0,
                        t.to_c()
                    )
                    .as_bytes(),
                )?;
                file.write(
                    format!(
                        "    for (size_t i = 0; i < result.{}_len; i++) {{\n",
                        field.0
                    )
                    .as_bytes(),
                )?;
                match t.as_ref() {
                    Types::Bool | Types::Int => {
                        file.write(
                            format!("        NEXT_INT(result.{}[i])\n", field.0).as_bytes(),
                        )?;
                    }
                    Types::Char => {
                        file.write(
                            format!("        NEXT_CHAR(result.{}[i])\n", field.0).as_bytes(),
                        )?;
                    }
                    Types::Float => {
                        file.write(
                            format!("        NEXT_FLOAT(result.{}[i])\n", field.0).as_bytes(),
                        )?;
                    }
                    Types::String => {
                        file.write(
                            format!("        NEXT_STR(result.{}[i])\n", field.0).as_bytes(),
                        )?;
                    }
                    Types::Struct(name) => {
                        file.write(format!("        result.{}[i] = unpack_{}(__input_buffer + __buffer_offset, buffer_len - __buffer_offset);\n", field.0, name).as_bytes())?;
                    }
                    Types::Array(_) => {
                        unimplemented!("Array of arrays not supported");
                    }
                }
                file.write(b"    }\n")?;
            }
        }
    }
    if free_buffer {
        file.write(b"    FREE_BUFFER()\n")?;
    }
    file.write(b"    return result;\n")?;
    file.write(b"}\n")?;
    Ok(())
}

fn generate_struct_deserialisation(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    let protocol = if let StructType::Protocol(_) = s.get_type() {
        generate_struct_deserialisation_signature(h_file, name)?;
        h_file.write(b";\n")?;
        true
    } else {
        false
    };
    generate_struct_deserialisation_function(c_file, name, s, protocol)?;
    Ok(())
}

fn generate_size_function_signature(
    c_file: &mut fs::File,
    name: &str,
) -> Result<(), std::io::Error> {
    c_file.write(format!("size_t {}_size(const {} *s)", name, name).as_bytes())?;
    Ok(())
}

fn generate_size_function(
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_size_function_signature(c_file, name)?;
    c_file.write(b"{\n	return ")?;
    let mut first = true;
    for field in s.fields() {
        if !first {
            c_file.write(b" + ")?;
        }
        first = false;
        match field.1 {
            Types::Bool | Types::Int | Types::Float => {
                c_file.write(format!("TYPST_INT_SIZE").as_bytes())?;
            }
            Types::String => {
                c_file.write(format!("strlen(s->{}) + 1", field.0).as_bytes())?;
            }
            Types::Struct(_) => {
                c_file.write(format!("{}_size(&s->{})", field.1.to_c(), field.0).as_bytes())?;
            }
            Types::Array(_) => {
                c_file.write(
                    format!(
                        "TYPST_INT_SIZE + s->{}_len * sizeof({})",
                        field.0,
                        field.1.to_c()
                    )
                    .as_bytes(),
                )?;
            }
            _ => {
                unimplemented!("Array and Struct deserialisation");
            }
        }
    }
    c_file.write(b";\n}\n")?;
    Ok(())
}

fn generate_struct_serialisation_signature(
    file: &mut fs::File,
    name: &str,
) -> Result<(), std::io::Error> {
    file.write(
        format!(
            "int pack_{}(const {} *s, uint8_t *buffer, size_t *buffer_len)",
            name, name
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn generate_struct_serialisation_function(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_struct_serialisation_signature(file, name)?;
    file.write(b" {\n")?;
    if let StructType::Struct = s.get_type() {
        file.write(b"	size_t __buffer_offset = 0;")?;
        file.write(format!("	size_t s_size = {}_size(s);\n", name).as_bytes())?;
        file.write(b"	if (s_size > *buffer_len) {\n")?;
        file.write(b"		return 1;\n")?;
        file.write(b"	}\n")?;
    } else {
        file.write(format!("*buffer_len = {}_size(s);\n", name).as_bytes())?;
        file.write(b"    INIT_BUFFER_PACK(buffer, buffer_len)\n")?;
    }

    for field in s.fields() {
        match field.1 {
            Types::Bool | Types::Int => {
                file.write(format!("    INT_PACK(s->{})\n", field.0).as_bytes())?;
            }
            Types::Float => {
                file.write(format!("    FLOAT_PACK(s->{})\n", field.0).as_bytes())?;
            }
            Types::String => {
                file.write(format!("    STR_PACK(s->{})\n", field.0).as_bytes())?;
            }
            Types::Char => {
                file.write(format!("    CHAR_PACK(s->{})\n", field.0).as_bytes())?;
            }
            Types::Struct(name) => {
                file.write(
                    format!(
                        "    pack_{}(&s->{}, __input_buffer + __buffer_offset, buffer_len);\n",
                        name, field.0
                    )
                    .as_bytes(),
                )?;
            }
            Types::Array(t) => {
                file.write(format!("    INT_PACK(s->{}_len)\n", field.0).as_bytes())?;
                file.write(
                    format!("    for (size_t i = 0; i < s->{}_len; i++) {{\n", field.0).as_bytes(),
                )?;
                match t.as_ref() {
                    Types::Bool | Types::Int => {
                        file.write(format!("        INT_PACK(s->{}[i])\n", field.0).as_bytes())?;
                    }
                    Types::Float => {
                        file.write(format!("        FLOAT_PACK(s->{}[i])\n", field.0).as_bytes())?;
                    }
                    Types::String => {
                        file.write(format!("        STR_PACK(s->{}[i])\n", field.0).as_bytes())?;
                    }
                    Types::Char => {
                        file.write(format!("        CHAR_PACK(s->{}[i])\n", field.0).as_bytes())?;
                    }
                    Types::Struct(name) => {
                        file.write(format!("        pack_{}(&s->{}[i], __input_buffer + __buffer_offset, buffer_len);\n", name, field.0).as_bytes())?;
                    }
                    Types::Array(_) => {
                        unreachable!("Array of arrays not supported");
                    }
                }
                file.write(b"    }\n")?;
            }
        }
    }
    file.write(b"\n\treturn 0;\n}\n")?;
    Ok(())
}

fn generate_struct_serialisation(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    if let StructType::Protocol(_) = s.get_type() {
        generate_struct_serialisation_signature(h_file, name)?;
        h_file.write(b";\n")?;
    }

    generate_size_function(c_file, name, s)?;
    generate_struct_serialisation_function(c_file, name, s)?;
    Ok(())
}

fn generate(
    h_file: &mut fs::File,
    c_file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    generate_struct(h_file, name, s)?;
    if s.decoder {
        generate_struct_deserialisation(h_file, c_file, name, s)?;
    }
    if s.encoder {
        generate_struct_serialisation(h_file, c_file, name, s)?;
    }
    h_file.write(b"\n")?;
    Ok(())
}

pub fn generate_protocol(path: &str, p: &Protocol) -> Result<(), std::io::Error> {
    let c_path = format!("{}/protocol.c", path);
    let h_path = format!("{}/protocol.h", path);
    let mut c_file = fs::File::create(c_path)?;
    let mut h_file = fs::File::create(h_path)?;
    generate_header(&mut h_file)?;
    c_file.write(C.as_bytes())?;
    for (name, s) in p.structs() {
        generate(&mut h_file, &mut c_file, name, s)?;
    }
    for (name, s) in p.protocols() {
        generate(&mut h_file, &mut c_file, name, s)?;
    }
    generate_footer(&mut h_file)?;
    Ok(())
}
