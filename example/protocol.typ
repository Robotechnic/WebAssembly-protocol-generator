/// Encodes a 32-bytes integer into big-endian bytes.
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
  (result, 4)
}

/// Encodes a string into bytes.
#let encode-string(value) = {
	bytes(value) + bytes(0x00)
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
	(str(bytes.slice(0, length - 1)), length)
	//(array(bytes.slice(0, length - 1)), length)
}

/// Encodes a boolean into bytes
#let encode-bool(value) = {
  if value {
	bytes(0x01)
  } else {
	bytes(0x00)
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

#let fractional-to-binary(fractional_part, max_dec) = {
	let result = 0
	let i = 0
	while fractional_part != 0 and i < (23 - max_dec) {
		fractional_part = fractional_part * 2
		if fractional_part >= 1 {
			result *= 2
			result += 1
			fractional_part = fractional_part - 1
		}
		i += 1
	}
	(result, i)
}

#let float-to-int(value) = {
	let sign = if value < 0.0 { 1 } else { 0 }
	let value = calc.abs(value)
	let integer_part = calc.trunc(value)
	let fractional_part = calc.frac(value)
	let exponent = calc.floor(calc.log(base: 2, integer_part))
	let (fractional_part, shift) = fractional-to-binary(fractional_part, exponent)
	integer_part *= calc.pow(2, 23 - exponent)
	integer_part += fractional_part * calc.pow(2, 23 - exponent - shift)
	integer_part -= calc.pow(2, 23)
	exponent += 127
	sign * calc.pow(2, 31) + exponent * calc.pow(2, 23) + integer_part
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

/// Decodes a float from the given bytes
#let decode-float(bytes) = {
	(int-to-float(decode-int(bytes).at(0)), 4)
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
#let decode-Number(bytes) = {
  let offset = 0
  let (f_half, size) = decode-float(bytes.slice(offset, bytes.len()))
  offset += size
  let (f_closestInt, size) = decode-int(bytes.slice(offset, bytes.len()))
  offset += size
  let (f_romanRepresentation, size) = decode-string(bytes.slice(offset, bytes.len()))
  offset += size
  let (f_isNegative, size) = decode-bool(bytes.slice(offset, bytes.len()))
  offset += size
  let (f_isOdd, size) = decode-bool(bytes.slice(offset, bytes.len()))
  offset += size
  ((
    half: f_half,
    closestInt: f_closestInt,
    romanRepresentation: f_romanRepresentation,
    isNegative: f_isNegative,
    isOdd: f_isOdd,
  ), offset)
}
#let encode-askNumber(value) = {
  encode-int(value.at("numberCount"))
}
#let decode-result(bytes) = {
  let offset = 0
  let (f_numbers, size) = decode-list(bytes.slice(offset, bytes.len()), decode-Number)
  offset += size
  ((
    numbers: f_numbers,
  ), offset)
}
