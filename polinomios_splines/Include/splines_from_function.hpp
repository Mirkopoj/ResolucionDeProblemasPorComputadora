#pragma once
#include "polinomials.hpp"
#include <ginac/symbol.h>
#include <vector>
#include <ginac/ginac.h>

typedef struct {
	double m_x;
	double m_y;
	double m_k;
} knot;

class SplineFromFunction {
public:
	SplineFromFunction(
			GiNaC::ex,
			GiNaC::symbol,
			double,
			double,
			uint32_t);
	SplineFromFunction(SplineFromFunction &&) = default;
	SplineFromFunction(const SplineFromFunction &) = default;
	SplineFromFunction &operator=(SplineFromFunction &&) = default;
	SplineFromFunction &operator=(const SplineFromFunction &) = default;
	~SplineFromFunction() = default;

	class OutOfRange : public std::exception {
		public:
		OutOfRange() = default;
		~OutOfRange() = default;
		OutOfRange(OutOfRange &&) = default;
		OutOfRange(const OutOfRange &) = default;
		const char * what() const noexcept(true) override;
		private:
		const char * msg = "Atempt to evaluate spline out of range";
	};
	
	double operator()(double) const noexcept(false);

private:
	std::vector<Polinomial> m_splines;
	std::vector<knot> m_knots;

	double t_(double, int) const;
	double a_(int) const;
	double b_(int) const;

	Polinomial gen_spline_section(int) const;

	double evaluate(double) const noexcept(false);

};
