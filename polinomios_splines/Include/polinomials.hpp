#pragma once
#include <complex>
#include <cstdint>
#include <cwchar>
#include <ostream>
#include <span>
#include <string>
#include <vector>
#include <exception>
#include <algorithm>

class Polinomial {
private:

	typedef struct term{
		int m_order;
		double m_coeficient;
		struct term operator*(term);
		struct term operator*(double);
		Polinomial operator*(const Polinomial&);
		struct term operator/(term);
		struct term operator-() const;
	} term;

public:

	Polinomial(std::vector<term> t);
	Polinomial(std::span<const double> c);
	Polinomial() = default;
	Polinomial(Polinomial &&) = default;
	Polinomial(const Polinomial &) = default;
	Polinomial &operator=(Polinomial &&);
	Polinomial &operator=(const Polinomial &);
	~Polinomial();

	class ZeroDivision : public std::exception {
		public:
		ZeroDivision();
		~ZeroDivision();
		ZeroDivision(ZeroDivision &&);
		ZeroDivision(const ZeroDivision &);
		const char * what() const noexcept(true) override;
		private:
		const char * msg = "Atempt to divide by zero";
	};
	

	Polinomial operator+(const Polinomial&) const;
	Polinomial operator+(const term&) const;
	Polinomial operator+=(const Polinomial&);
	Polinomial operator+=(const term&);

	Polinomial operator-(const Polinomial&) const;
	Polinomial operator-(const term&) const;
	Polinomial operator-=(const Polinomial&);
	Polinomial operator-=(const term&);
	Polinomial operator-() const;

	Polinomial operator*(const Polinomial&) const;
	Polinomial operator*(const term&) const;
	Polinomial operator*(const double) const;
	Polinomial operator*=(const Polinomial&) const;
	Polinomial operator*=(const term&) const;
	Polinomial operator*=(const double) const;

	double operator()(const double) const;
	std::complex<double> operator()(const std::complex<double>) const;
	Polinomial operator()(const Polinomial) const;

	std::tuple<Polinomial, Polinomial> operator/(Polinomial&) const noexcept(false);
	std::tuple<Polinomial, Polinomial> operator/=(Polinomial&) noexcept(false);

	bool operator==(const Polinomial&) const;

	friend std::ostream &operator<<(std::ostream &out, const Polinomial &p);
	friend std::istream &operator>>(std::istream &in, Polinomial &p);

	Polinomial operator*() const;

	std::vector<std::complex<double>> roots() const;
	std::vector<double> r_roots() const;
	std::vector<std::complex<double>> roots(double) const;
	std::vector<double> r_roots(double) const;
	std::vector<std::complex<double>> roots(uint32_t) const;
	std::vector<double> r_roots(uint32_t) const;

	uint32_t degree() const;
	
	std::ostream &pprint(std::ostream&, char) const;

private:

	typedef struct {
		uint32_t m_pos;
		double m_coeficient;
	} vec_term;

	Polinomial m_add(const Polinomial) const;
	Polinomial m_multiplication(const Polinomial) const;
	std::tuple<Polinomial, Polinomial> m_division(const Polinomial divisor) const;
	Polinomial m_evaluate(const Polinomial) const;
	double m_evaluate(const double) const;
	std::complex<double> m_evaluate(const std::complex<double>) const;
	std::complex<long double> m_evaluate(const std::complex<long double>) const;

	Polinomial add_term(const term) const;
	vec_term get_term(int order) const;
	int get_term_count() const;
	void sort();
	Polinomial derivate() const;
	std::vector<std::complex<double>> aberth_roots(double, uint32_t) const;

	void vec_init(std::vector<term>);

	std::vector<term> m_terms;
};
