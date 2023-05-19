#include "../Include/splines.hpp"
#include <algorithm>
#include <cmath>
#include <cstddef>
#include <vector>

double Spline::m(int k, double c) const {
	return 
		(1-c)*
		((m_knots[k+1].p-m_knots[k-1].p)/
		 (m_knots[k+1].x-m_knots[k-1].x));
}

Polinomial Spline::gen_spline_section(int k, double c) const {
	const Polinomial h00 = Polinomial({{ 2.0,-3.0, 0.0, 1.0}});
	const Polinomial h10 = Polinomial({{ 1.0,-2.0, 1.0, 0.0}});
	const Polinomial h01 = Polinomial({{-2.0, 3.0, 0.0, 0.0}});
	const Polinomial h11 = Polinomial({{ 1.0,-1.0, 0.0, 0.0}});
	return h00*m_knots[k].p
		+ h10*(m_knots[k+1].x-m_knots[k].x)*m(k, c)
		+ h01*m_knots[k+1].p
		+ h11*(m_knots[k+1].x-m_knots[k].x)*m(k+1, c);
}

double Spline::t_(double x, int k) const {
	return (x-m_knots[k].x)/
		(m_knots[k+1].x-m_knots[k].x);
}

void Spline::splines_init(std::vector<knot> ks, double c) {
	m_knots = ks;
	for (size_t k=1; k<ks.size()-1; k++ ) {
		m_splines.push_back(gen_spline_section(k, c));
	}
}

Spline::Spline(std::vector<knot> ks, double c) {
	splines_init(ks, c);
}

Spline::Spline(std::vector<knot> ks) {
	splines_init(ks, 1.0);
}

double Spline::evaluate(double x) const {
	for (int i=1; i<m_knots.size()-2; i++) {
		if(x>=m_knots[i].x && x<=m_knots[i+1].x)
			return m_splines[i-1](x);
	}
	OutOfRange e;
	throw e;
}

double Spline::operator()(double x) const noexcept(false){
	return evaluate(x);
}

const char * Spline::OutOfRange::what() const noexcept(true){
	return msg;
}
