#import "protocol.typ": *
#let plugin = plugin("example.wasm")

#let result = plugin.ask_number(encode-askNumber(("numberCount": 10)))

#let unpacked = decode-result(result).at(0).at("numbers")

= Converter example
#table(
	rows: 5,
	columns: 11,
	"Number",
	..unpacked.map(row => str(row.at("closestInt"))),
	"Roman representation",
	..unpacked.map(row => row.at("romanRepresentation")),
	"Half",
	..unpacked.map(row => str(calc.round(row.at("half"), digits: 10))),
	"Is negative",
	..unpacked.map(row => if row.at("isNegative") { "Yes" } else { "No" }),
	"Is odd",
	..unpacked.map(row => if row.at("isOdd") { "Yes" } else { "No" }),
)
