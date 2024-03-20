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
