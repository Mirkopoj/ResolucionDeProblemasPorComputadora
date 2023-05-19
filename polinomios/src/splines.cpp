#include "../Include/splines.hpp"
#include <cmath>

double SplineFactory::m(int k) const {
	return 
		(1-c)*
		((m_knots[k+1].p-m_knots[k-1].p)/
		 (m_knots[k+1].x-m_knots[k-1].x));
}

double SplineFactory::t_(double x, int k) const {
	return (x-m_knots[k].x)/
		(m_knots[k+1].x-m_knots[k].x);
}

Polinomial SplineFactory::gen_spline(int k) const {
	return h00*m_knots[k].p
		+ h10*(m_knots[k+1].x-m_knots[k].x)*m(k)
		+ h01*m_knots[k+1].p
		+ h11*(m_knots[k+1].x-m_knots[k].x)*m(k+1);
}
