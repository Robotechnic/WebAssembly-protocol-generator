use std::{fs, io::Write};

use crate::{protocol::Protocol, struct_::Struct, types::Types};

const FILE_HEADER: &str = "/// Encodes a 32-bytes integer into big-endian bytes.
#let encode-int(value) = {
  bytes((
    calc.rem(calc.quo(value, 0x1000000), 0x100),
    calc.rem(calc.quo(value, 0x10000), 0x100),
    calc.rem(calc.quo(value, 0x100), 0x100),
    calc.rem(calc.quo(value, 0x1), 0x100),
  ))
}

/// Decodes a big-endian integer from the given bytes.
#let decode-int(bytes) = {
  let result = 0
  for byte in array(bytes.slice(0,4)) {
    result = result * 256 + byte
  }
  if (result > 2147483647) { // the number is negative
    result = 2147483647 - result + 2147483647
  }
  (result, 4)
}

/// Encodes a string into bytes.
#let encode-string(value) = {
	bytes(value) + bytes((0x00,))
}

/// Decodes a string from the given bytes.
#let decode-string(bytes) = {
	let length = 0
	for byte in array(bytes) {
		length = length + 1
		if byte == 0x00 {
			break
		}
	}
	if length == 0 {
		(\"\", 1)
	} else { 
		(str(bytes.slice(0, length - 1)), length)
	}
	//(array(bytes.slice(0, length - 1)), length)
}

/// Encodes a boolean into bytes
#let encode-bool(value) = {
  if value {
	bytes((0x01,))
  } else {
	bytes((0x00,))
  }
}

/// Decodes a boolean from the given bytes
#let decode-bool(bytes) = {
  if bytes.at(0) == 0x00 {
	(false, 1)
  } else {
	(true, 1)
  }
}

/// Encodes a character into bytes
#let encode-char(value) = {
  bytes(value)
}

/// Decodes a character from the given bytes
#let decode-char(bytes) = {
  (bytes.at(0), 1)
}

#let fractional-to-binary(fractional_part, max_dec, zero) = {
	let result = 0
	let i = 22 - max_dec
	let first_one = 0
	if zero {
		while fractional_part < 1 {
			fractional_part *= 2
			first_one += 1
		}
		fractional_part -= 1
		i = 23
	}
	while i > 0 and fractional_part > 0 {
		fractional_part *= 2
		if fractional_part >= 1 {
			result += calc.pow(2, i - 1)
			fractional_part -= 1
		}
		i -= 1
	}
	(result, first_one)
}

#let float-to-int(value) = {
	if value == 0 {
		return 0
	}
	let sign = if value < 0.0 { 1 } else { 0 }
	let value = calc.abs(value)
	let mantissa = calc.trunc(value)
	let fractional_part = calc.fract(value)
	let exponent = if mantissa == 0 {
		0
	} else {
		calc.floor(calc.log(base: 2, mantissa)) - 1
	}
	let (fractional_part, first_one) = fractional-to-binary(fractional_part, exponent, mantissa == 0)
	mantissa *= calc.pow(2, 22 - exponent)
	mantissa += fractional_part
	if exponent == 0 {
		exponent = -first_one
	}
	exponent += 127
	return  sign * calc.pow(2, 31) + exponent * calc.pow(2, 23) + mantissa
}

#let mantissa-to-float(mantissa) = {
	let result = 1.0
	for i in range(0,23) {
		if calc.rem(mantissa, 2) == 1 {
			result += 1.0/calc.pow(2, 23 - i)
		}
		mantissa = calc.quo(mantissa, 2)
	}
	result
}

#let int-to-float(value) = {
	if value == 0 {
		return 0.0
	}
	let sign = if value >= calc.pow(2, 31) {
		value -= calc.pow(2, 31)
		 -1 
	} else { 
		1
	}
	let exponent = calc.rem(calc.quo(value, calc.pow(2, 23)), calc.pow(2, 8))
	let mantissa = calc.rem(value, calc.pow(2, 23))
	sign * calc.pow(2, exponent - 127) * mantissa-to-float(mantissa)
}

/// Encodes a float into bytes
#let encode-float(value) = {
	encode-int(float-to-int(value))
}

#let encode-point(value) = {
	encode-float(value.pt())
}

/// Decodes a float from the given bytes
#let decode-float(bytes) = {
	let (decoded, size) = decode-int(bytes)
	(int-to-float(decoded), size)
}

#let decode-point(bytes) = {
	let (value, size) = decode-float(bytes)
	(value * 1pt, size)
}

/// Encodes a list of elements into bytes
#let encode-list(arr, encoder) = {
	let length = encode-int(arr.len())
	let encoded = bytes(arr.map(encoder).map(array).flatten())
	length + encoded
}

/// Decodes a list of elements from the given bytes
#let decode-list(bytes, decoder) = {
	let (length, length_size) = decode-int(bytes)
	let result = ()
	let offset = length_size
	for i in range(0, length) {
		let (element, size) = decoder(bytes.slice(offset, bytes.len()))
		result.push(element)
		offset += size
	}
	(result, offset)
}
";

fn generate_header(file: &mut fs::File) -> Result<(), std::io::Error> {
    file.write(FILE_HEADER.as_bytes())?;
    Ok(())
}

fn generate_dictionary_serialisation(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    file.write(format!("#let encode-{}(value) = {{\n", name).as_bytes())?;
    file.write(b"  ")?;
    let mut first = true;
    for (name, t, _) in s.iter() {
        if !first {
            file.write(b" + ")?;
        }
        match t {
            Types::Array(t) => {
                file.write(
                    format!("encode-list(value.at(\"{}\"), encode-{})", name, t.to_typst()).as_bytes(),
                )?;
            }
            Types::Struct(t) => {
                file.write(format!("encode-{}(value.at(\"{}\"))", t, name).as_bytes())?;
            }
            _ => {
                file.write(format!("encode-{}(value.at(\"{}\"))", t.to_typst(), name).as_bytes())?;
            }
        }
        first = false;
    }
    file.write(b"\n}\n")?;
    Ok(())
}

fn generate_dictionary_deserialisaion(
    file: &mut fs::File,
    name: &str,
    s: &Struct,
) -> Result<(), std::io::Error> {
    file.write(format!("#let decode-{}(bytes) = {{\n", name).as_bytes())?;
    file.write(b"  let offset = 0\n")?;
    for (name, t, _) in s.iter() {
        file.write(format!("  let (f_{}, size) = ", name).as_bytes())?;
        match t {
            Types::Array(t) => {
                file.write(
                    format!(
                        "decode-list(bytes.slice(offset, bytes.len()), decode-{})",
                        t.to_typst()
                    )
                    .as_bytes(),
                )?;
            }
            Types::Struct(t) => {
                file.write(format!("decode-{}(bytes.slice(offset, bytes.len()))", t).as_bytes())?;
            }
            _ => {
                file.write(
                    format!("decode-{}(bytes.slice(offset, bytes.len()))", t.to_typst()).as_bytes(),
                )?;
            }
        }
        file.write(b"\n  offset += size\n")?;
    }
	file.write(b"  ((\n")?;
	for (name, _, _) in s.iter() {
		file.write(format!("    {}: f_{},\n", name, name).as_bytes())?;
	}
    file.write(b"  ), offset)\n}\n")?;
    Ok(())
}

fn generate(file: &mut fs::File, name: &str, s: &Struct) -> Result<(), std::io::Error> {
    if s.decoder {
        generate_dictionary_serialisation(file, name, s)?;
    } else {
        generate_dictionary_deserialisaion(file, name, s)?;
    }
    Ok(())
}

pub fn generate_protocol(path: &str, p: &Protocol) -> Result<(), std::io::Error> {
    let path = format!("{}/protocol.typ", path);
    let mut file = fs::File::create(path)?;
    generate_header(&mut file)?;
    for (name, s) in p.ordered_structs() {
        generate(&mut file, name, s)?;
    }
    for (name, s) in p.protocols() {
        generate(&mut file, name, s)?;
    }
    Ok(())
}
