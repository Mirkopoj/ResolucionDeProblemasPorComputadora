#include <vector>
#define WITHOUT_NUMPY
#include <matplotlibcpp.h>
#include "../../Include/splines_from_function.hpp"
#include <iostream>

namespace plt = matplotlibcpp;

int main (int argc, char *argv[]) {
	GiNaC::symbol x;
	GiNaC::ex e = sin(x);
	SplineFromFunction spline(e, x, -10.0, 10.0, 20);
	std::vector<double> puntos;
	puntos.reserve(198);
	for (double i = -9.9; i<9.9; i+=0.1) {
		puntos.push_back(spline(i));
	}
	plt::plot(puntos);
	plt::show();
	return 0;
}
