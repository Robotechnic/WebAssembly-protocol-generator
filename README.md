# Wasm protocol genrator

This is a simple tool to generate a protocol for a wasm module. It generates functions for c and typst to encode and decode the protocol. The protocol is defined in a simple file format that ressembles a list of c structs.

## Usage

| Command | Description |
| :-----: | ----------- |
| -o      | Output folder |
| -c | C code output folder, it overrides -o and requires -t |
| -t | Typescript code output folder, it overrides -o and requires -c |
| --check | Check the protocol file for errors without generating any code |

## Protocol file format

The protocol file format is a list of structs and protocol definitions. You can define multiple protocol in the same file and each file will generate a different set of functions.

### Structs

A struct is defined like follows:

```c
struct Name {
    ...fieds
}
```

Where `Name` is the name of the struct and `fields` is a list of fields separated by semicolon.

### Protocol

A protocol is defined like follows:

```c
protocol (C|Typst|Bidirectional) Name {
    ...fields
}
```

The langage is the target language of the protocol, if it is `C`, the protocol will be encoded by Typst and decoded by C. if it is `Typst` the protocol will be encoded by C and decoded by Typst. `Bidirectional` is used when you whant the protocol to be encoded and decoded by both languages.

Where `Name` is the name of the protocol and `fields` is a list of fields separated by semicolon.

### Fields

A field is defined like follows:

```c
type name;
```

Where `type` is the type of the field and `name` is the name of the field.

#### Types

The following types are supported:

| Type | Description |
| :--: | ----------- |
| int | 32 bits signed integer |
| bool | a boolean |
| string | a c like string |
| char | a single character |
| float | 32 bits floating point number |
| point | 32 bits floating point number in point, so it will be treated as a length in point in typst and as normal float in C |
| `Name` | The type of the struct `Name` defined previously in the file |

Any of the previous types can be put in an array by adding `[]` after the field name.

#### Optional fields

Adding a `?` after the type will make the field optional. This means that the field may be present or not in the encoded protocol. This is only supported for non-array fields. On typst side this is translated by the field being `none` or having a value. On C side, the field will be a pointer to the type. If the pointer is `NULL`, the field is not present, otherwise it is present.

## Example

```c
struct Point {
    int x;
    int y;
}
```

This defines a struct `Point` with two fields `x` and `y` of type `int`.

```c
protocol C Shape {
    Point points[];
    string name;
}
```

This defines a protocol `Shape` that will be encoded by Typst and decoded by C. It has two fields, `points`, an array of `Point` and `name` a string.

```c
protocol Typst Area {
    float area;
    string name;
}
```

This defines a protocol `Area` that will be encoded by C and decoded by Typst. It has two fields, `area`, a float and `name` a string.

## Genrated code

### C

The generated C code will expose the struct and the functions to encode and decode each protocol. The struct will be the same as defined in the protocol file with the same name.

Each function name will be the name of the protocol prefixed by `encode_` or `decode_`.

If you add an array to your protocol, the generated struct will have a field `name_len` that must contain the length of the array before encoding the protocol.

When you use them, you shouldn't manipulate the input buffer directly, the decode function will do it for you. The only thing you need to do is to pass the input buffer length to the decode function.

#### Error codes signification

| Code | Description |
| :--: | ----------- |
| 0 | No error |
| 1 | Malloc error |
| 2 | Invalid buffer length |
| 3 | Invalid protocol |

#### Example

Based on the previous example, you will get the following exposed interface:

```c
struct Point {
    int x;
    int y;
}

struct Shape {
    Point *points;
    int points_len;
    char *name;
}

struct Area {
    float area;
    char *name;
}

int encode_Shape(Shape *shape);
int decode_Area(size_t buffer_len, Area *area);
```

### Typst

The generated Typst will use dictionaries to represent the protocol structures. But the naming convention is rufly the same as the C code: the functions will be prefixed by `encode-` or `decode-`.

#### Example

Based on the previous example, you will get the following exposed interface:

```typst
#let encode-area(value) = ...
#let decode-shape(bytes) = ...
```
