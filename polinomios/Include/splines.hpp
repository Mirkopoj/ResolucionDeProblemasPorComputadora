#pragma once
#include "polinomials.hpp"
#include <vector>

typedef struct {
	double x;
	double p;
} knot;

class Spline {
public:
	Spline(std::vector<knot>) noexcept(false);
	Spline(std::vector<knot>, double) noexcept(false);
	Spline(Spline &&) = default;
	Spline(const Spline &) = default;
	Spline &operator=(Spline &&) = default;
	Spline &operator=(const Spline &) = default;
	~Spline() = default;

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
	
	class InsuficientKnots : public std::exception {
		public:
		InsuficientKnots() = default;
		~InsuficientKnots() = default;
		InsuficientKnots(InsuficientKnots &&) = default;
		InsuficientKnots(const InsuficientKnots &) = default;
		const char * what() const noexcept(true) override;
		private:
		const char * msg = "Atempt to crete spline with insuficient knots";
	};
	
	double operator()(double) const noexcept(false);

private:
	std::vector<Polinomial> m_splines;
	std::vector<knot> m_knots;

	void splines_init(std::vector<knot>, double) noexcept(false);

	double t_(double, int) const;

	double m(int, double) const;

	Polinomial gen_spline_section(int, double) const;

	double evaluate(double) const noexcept(false);

};
