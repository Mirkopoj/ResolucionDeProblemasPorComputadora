#include <vector>
#include "../../Include/splines.hpp"
#include <iostream>
#include <matplot/matplot.h>

namespace plt = matplot;

int main (int argc, char *argv[]) {
	std::vector<knot> knots = {
		{.m_x =  0.0, .m_p =  0.0},
		{.m_x =  1.0, .m_p =  0.84147098481},
		{.m_x =  2.0, .m_p =  0.90929742683},
		{.m_x =  3.0, .m_p =  0.14112000806},
		{.m_x =  4.0, .m_p = -0.75680249531},
		{.m_x =  5.0, .m_p = -0.95892427266},
		{.m_x =  6.0, .m_p = -0.2794154982},
		{.m_x =  7.0, .m_p =  0.65698659872},
		{.m_x =  8.0, .m_p =  0.98935824662},
		{.m_x =  9.0, .m_p =  0.41211848524},
		{.m_x = 10.0, .m_p = -0.54402111089},
		{.m_x = 11.0, .m_p = -0.99999029655},
		{.m_x = 12.0, .m_p = -0.536572918},
		{.m_x = 13.0, .m_p =  0.42016703683},
		{.m_x = 14.0, .m_p =  0.99060735569},
	};
	Spline spline_0_0(knots, 0.0);
	Spline spline_0_2(knots, 0.2);
	Spline spline_0_4(knots, 0.4);
	Spline spline_0_6(knots, 0.6);
	Spline spline_0_8(knots, 0.8);
	Spline spline_1_0(knots, 1.0);
	std::vector<double> puntos_0_0;
	std::vector<double> puntos_0_2;
	std::vector<double> puntos_0_4;
	std::vector<double> puntos_0_6;
	std::vector<double> puntos_0_8;
	std::vector<double> puntos_1_0;
	std::vector<double> eje;
	puntos_0_0.reserve(128);
	puntos_0_2.reserve(128);
	puntos_0_4.reserve(128);
	puntos_0_6.reserve(128);
	puntos_0_8.reserve(128);
	puntos_1_0.reserve(128);
	eje.reserve(128);
	for (double i = 1.1; i<12.9; i+=0.1) {
		puntos_0_0.push_back(spline_0_0(i));
		puntos_0_2.push_back(spline_0_2(i));
		puntos_0_4.push_back(spline_0_4(i));
		puntos_0_6.push_back(spline_0_6(i));
		puntos_0_8.push_back(spline_0_8(i));
		puntos_1_0.push_back(spline_1_0(i));
		eje.push_back(i);
	}
	plt::hold(plt::on);
	plt::plot(eje, puntos_0_0);
	plt::plot(eje, puntos_0_2);
	plt::plot(eje, puntos_0_4);
	plt::plot(eje, puntos_0_6);
	plt::plot(eje, puntos_0_8);
	plt::plot(eje, puntos_1_0);
	plt::legend({"0.0", "0.2", "0.4", "0.6", "0.8", "1.0"});
	plt::show();
	return 0;
}
