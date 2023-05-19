#pragma once
#include "polinomials.hpp"
#include <vector>

class SplineFactory {
	private:
		typedef struct {
			double x;
			double p;
		} knot;

		std::vector<knot> m_knots;
		/*
		x = (xk,xk+1)
		t =(x-xk)/(xk+1-x)*/
		const Polinomial h00 = Polinomial({{ 2.0,-3.0, 0.0, 1.0}});
		const Polinomial h10 = Polinomial({{ 1.0,-2.0, 1.0, 0.0}});
		const Polinomial h01 = Polinomial({{-2.0, 3.0, 0.0, 0.0}});
		const Polinomial h11 = Polinomial({{ 1.0,-1.0, 0.0, 0.0}});

		double c;

		double m(int) const;

		double t_(double, int) const;

		Polinomial gen_spline(int) const;
};
