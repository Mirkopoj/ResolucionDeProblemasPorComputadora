#include "../../Include/splines.hpp"
#include <ginac/ginac.h>
#include <ginac/parser.h>
#include <iostream>

int main (int argc, char *argv[]) {
	GiNaC::symbol a("a"), b("b"), c("c"), d("d"), f("f"), g("g"), w("w"), x("x"), y("y"), z("z");
	GiNaC::symtab table;
	table["a"] = a;
	table["b"] = b;
	table["c"] = c;
	table["d"] = d;
	table["f"] = f;
	table["w"] = w;
	table["x"] = x;
	table["y"] = y;
	table["z"] = z;
	GiNaC::parser reader(table);
	if (argc < 2) {
		std::cout<< "You must provide an expresion, exp. %s 3+2*x^2\n";
		exit(-1);
	}
	std::string input_expresion = "";
	for (int i = 1; i<argc; i++) {
		input_expresion += argv[i];
	}
	GiNaC::ex expresion = reader(input_expresion);
	std::cout << GiNaC::latex << expresion << std::endl;
	return 0;
}
