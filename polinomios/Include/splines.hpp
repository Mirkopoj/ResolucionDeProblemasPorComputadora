#pragma once
#include "polinomials.hpp"
#include <vector>

typedef struct {
	double x;
	double p;
} knot;

class Spline {
public:
	Spline(std::vector<knot>);
	Spline(std::vector<knot>, double);
	Spline(Spline &&) = default;
	Spline(const Spline &) = default;
	Spline &operator=(Spline &&) = default;
	Spline &operator=(const Spline &) = default;
	~Spline() = default;

	class OutOfRange : public std::exception {
		public:
		OutOfRange();
		~OutOfRange();
		OutOfRange(OutOfRange &&);
		OutOfRange(const OutOfRange &);
		const char * what() const noexcept(true) override;
		private:
		const char * msg = "Atempt to evaluate spline out of range";
	};
	
	double operator()(double) const noexcept(false);

private:
	std::vector<Polinomial> m_splines;
	std::vector<knot> m_knots;

	void splines_init(std::vector<knot>, double);

	double t_(double, int) const;

	double m(int, double) const;

	Polinomial gen_spline_section(int, double) const;

	double evaluate(double) const noexcept(false);

};
