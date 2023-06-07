#include "../Include/splines_from_function.hpp"
#include <algorithm>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <ginac/power.h>
#include <ginac/symbol.h>
#include <math.h>
#include <vector>

SplineFromFunction::SplineFromFunction(
		GiNaC::ex e,
		GiNaC::symbol x,
		double start,
		double end,
		uint32_t count
){
	GiNaC::ex d = e.diff(x);
	double step = (end-start)/count;
	m_knots.reserve(count);
	GiNaC::exmap subs;
	m_splines.reserve(count-1);
	for (uint32_t i=0; i<=count; i++) {
		double x_ = start + i*step;
		subs[x] = x_;
		GiNaC::ex sub_e = e.subs(subs);
		GiNaC::ex sub_d = d.subs(subs);
		m_knots.push_back({
			.m_x = x_,
			.m_y = GiNaC::ex_to<GiNaC::numeric>(sub_e).to_double(),
			.m_k = GiNaC::ex_to<GiNaC::numeric>(sub_d).to_double(),
		});
		if (i!=0){
			m_splines.push_back(gen_spline_section(i-1));
		}
	}
}

double SplineFromFunction::a_(int k) const {
	return 
		m_knots[k].m_k*
		(m_knots[k+1].m_x-m_knots[k].m_x)-
		(m_knots[k+1].m_y-m_knots[k].m_y);
}

double SplineFromFunction::b_(int k) const {
	return 
		(m_knots[k+1].m_y-m_knots[k].m_y)-
		m_knots[k+1].m_k*
		(m_knots[k+1].m_x-m_knots[k].m_x);
}

//Read gen_spline_section.pdf, under doc/
Polinomial SplineFromFunction::gen_spline_section(int k) const {
	double x1 = m_knots[k].m_x;
	double x2 = m_knots[k+1].m_x;
	double y1 = m_knots[k].m_y;
	double y2 = m_knots[k+1].m_y;
	double den = x2-x1;
	double a = a_(k);
	double b = b_(k);
	double x_h1 = x1/(x2-x1);
	double x_h2 = 1+x_h1;
	double o0 = x_h2*y1 - x_h1*y2-x_h1*pow(x_h2, 2)*a+x_h2*pow(x_h1, 2)*b; 
	double o1 = (y2-y1+(x_h2+x_h1)*(a*x_h2-b*x_h1)-x_h1*x_h2*(b-a))/(den);
	double o2 = (b*x_h1-a*x_h2+(x_h2+x_h1)*(b-a))/(std::pow(den,2));
	double o3 = (a-b)/(std::pow(den,3));
	return Polinomial({{3,o3},{2,o2},{1,o1},{0,o0}});
}

double SplineFromFunction::t_(double x, int k) const {
	return (x-m_knots[k].m_x)/
		(m_knots[k+1].m_x-m_knots[k].m_x);
}

double SplineFromFunction::evaluate(double x) const {
	for (int i=0; i<m_knots.size()-1; i++) {
		if(x>=m_knots[i].m_x && x<=m_knots[i+1].m_x)
			return m_splines[i](x);
	}
	OutOfRange e;
	throw e;
}

double SplineFromFunction::operator()(double x) const noexcept(false){
	return evaluate(x);
}

const char * SplineFromFunction::OutOfRange::what() const noexcept(true){
	return msg;
}
