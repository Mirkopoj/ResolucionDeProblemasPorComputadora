#pragma once
#include <cstdint>
#include <cwchar>
#include <span>
#include <string>
#include <vector>
#include <exception>
#include <algorithm>

class Polinomial {
private:
	typedef struct term{
		int order;
		double coeficient;
		struct term operator*(term b);
		struct term operator*(double b);
		Polinomial operator*(const Polinomial& b);
		struct term operator/(term b);
		struct term operator-() const;
	} term;

	typedef struct {
		uint32_t pos;
		double coeficient;
	} vec_term;

	Polinomial add(const Polinomial b) const;
	Polinomial multiplication(const Polinomial b) const;
	std::tuple<Polinomial, Polinomial> division(const Polinomial divisor);
	Polinomial evaluate(const Polinomial a) const;
	std::vector<double> roots(const Polinomial a) const;
	double evaluate(const double) const;

	Polinomial add_term(const term b) const;
	vec_term get_term(int order);
	int get_term_count();
	void sort();

	void vec_init(std::vector<term>);

	std::vector<term> terms;
	
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
	
	void print();

	Polinomial operator+(const Polinomial&);
	Polinomial operator+(const term&);
	Polinomial operator+=(const Polinomial&);
	Polinomial operator+=(const term&);
	Polinomial operator-(const Polinomial&);
	Polinomial operator-(const term&);
	Polinomial operator-=(const Polinomial&);
	Polinomial operator-=(const term&);
	Polinomial operator-() const;
	Polinomial operator*(const Polinomial&);
	Polinomial operator*(const term&);
	Polinomial operator*(const double) const;
	double operator()(const double) const;
	std::tuple<Polinomial, Polinomial> operator/(Polinomial&) noexcept(false);
	bool operator==(const Polinomial&);
};
