#include <ginac/ginac.h>
#include <ginac/parser.h>
#include <iostream>
#include <vector>
#define WITHOUT_NUMPY
#include <matplotlibcpp.h>
#include "../../Include/splines_from_function.hpp"

namespace plt = matplotlibcpp;

void print_help(char * name){
	std::cout<< "You must provide an expresion, exp. %s 3+2*x^2\n";
	std::cout<< "Variable should be x";
	std::cout<< "Usage: " << name << "[options] <expresion>\n";
	std::cout<< "-b <begining>\n";
	std::cout<< "-e <end>\n";
	std::cout<< "-s <steps>" << std::endl;
	exit(-1);
}

int main (int argc, char *argv[]) {
	double begining = -10;
	double end = 10;
	int steps = 20;

	GiNaC::symbol x("x");
	GiNaC::symtab table;
	table["x"] = x;
	GiNaC::parser reader(table);
	if (argc < 2) {
		print_help(argv[0]);
	}
	int ignore[6] = {0};
	int argc_p = argc-1;
	char* argv_p[argc_p];
	for (int i = 1; i<argc; i++) {
		if(!strcmp(argv[i], "-b")){
			begining = atof(argv[i+1]);
			ignore[0] = i;
			ignore[1] = i+1;
			argc_p -= 2;
		}
		if(!strcmp(argv[i], "-e")){
			end = atof(argv[i+1]);
			ignore[2] = i;
			ignore[3] = i+1;
			argc_p -= 2;
		}
		if(!strcmp(argv[i], "-s")){
			steps = atoi(argv[i+1]);
			ignore[4] = i;
			ignore[5] = i+1;
			argc_p -= 2;
		}
		if(!strcmp(argv[i], "-h")){
			print_help(argv[0]);
		}
	}
	int u=0;
	for (int i=1; i<argc; i++) {
		bool ign = false;
		for (int j : ignore) {
			if (i==j) {
				ign = true;
			}
		}
		if (!ign) {
			argv_p[u] = argv[i];
			u++;
		}
	}
	std::string input_expresion = "";
	for (int i = 0; i<argc_p; i++) {
		input_expresion += argv_p[i];
	}
	GiNaC::ex expresion = reader(input_expresion);

	SplineFromFunction spline(expresion, x, begining, end, steps);
	std::vector<double> puntos;
	std::vector<double> puntos_x;
	puntos.reserve((end-begining)/0.1);
	puntos_x.reserve((end-begining)/0.1);
	for (double i = begining+0.1; i<end-0.1; i+=0.1) {
		puntos.push_back(spline(i));
		puntos_x.push_back(i);
	}
	plt::plot(puntos_x, puntos);
	plt::show();

	return 0;
}
