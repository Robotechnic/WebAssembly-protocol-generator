#include "emscripten.h"
#include "protocol.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

typedef struct {
	char *roman;
	int decimal;
} Convert;

Convert conversions[] = {
  {"M", 1000},
  {"CM", 900},
  {"D", 500},
  {"CD", 400},
  {"C", 100},
  {"XC", 90},
  {"L", 50},
  {"XL", 40},
  {"X", 10},
  {"IX", 9},
  {"V", 5},
  {"IV", 4},
  {"I", 1}
};

char *roman_numeral(int number) {
  char *result = (char *)malloc(100);
  result[0] = '\0';
  for (int i = 0; i < sizeof(conversions) / sizeof(Convert); i++) {
	while (number >= conversions[i].decimal) {
	  strcat(result, conversions[i].roman);
	  number -= conversions[i].decimal;
	}
  }
  return result;
}

int roman_to_integer(char roman) {
	switch (roman) {
		case 'I':
		return 1;
		case 'V':
		return 5;
		case 'X':
		return 10;
		case 'L':
		return 50;
		case 'C':
		return 100;
		case 'D':
		return 500;
		case 'M':
		return 1000;
		default:
		return 0;
	}
}

int roman_to_int(char *roman) {
	int prev = 1001;
	int total = 0;
	for (int i = 0; i < strlen(roman); i++) {
		int current = roman_to_integer(roman[i]);
		if (current > prev) {
			total += current - 2 * prev;
		} else {
			total += current;
		}
	}
	return total;
}

void free_response(Number *response, size_t count) {
	for (int i = 0; i < count; i++) {
		free(response[i].romanRepresentation);
	}
}

void write_error_message(char *message) {
	wasm_minimal_protocol_send_result_to_host((uint8_t*)message, strlen(message));
}

EMSCRIPTEN_KEEPALIVE
int ask_number(size_t buffer_size) {
	askNumber n;
	if (decode_askNumber(buffer_size, &n)) {
        write_error_message("Failed to unpack askNumber");
		return 1;
	}
	Number response[n.numberCount];
	
	for (int i = 0; i < n.numberCount; i++) {
		int val = i;
		response[i].closestInt = val;
		response[i].romanRepresentation = roman_numeral(val);
		response[i].half = (float)val / 2;
		response[i].isOdd = val % 2;
		response[i].isNegative = val < 0;
	}
	result r = {response, n.numberCount};
	if (encode_result(&r)) {
        write_error_message("Failed to pack result");
		free_response(response, n.numberCount);
		return 1;
	}
	free_response(response, n.numberCount);
	return 0;
}

EMSCRIPTEN_KEEPALIVE
int roman_to_decimal(size_t buffer_size) {
	toDecimal n;
	if (decode_toDecimal(buffer_size, &n)) {
		write_error_message("Failed to unpack toDecimal");
		return 1;
	}
	decimalResult response;
	response.decimal = roman_to_int(n.roman);
	if (encode_decimalResult(&response)) {
		write_error_message("Failed to pack decimalResult");
		return 1;
	}
	return 0;
}