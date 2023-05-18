#include "../Include/polinomials.hpp"
#include <array>
#include <cmath>
#include <complex>
#include <criterion/hooks.h>
#include <cstdint>
#include <cstdlib>
#include <functional>
#include <limits>
#include <math.h>
#include <span>
#include <tuple>
#include <vector>
#include <algorithm>

Polinomial::Polinomial(std::vector<term> t){ 
	vec_init(t);
}

Polinomial::Polinomial(std::span<const double> c){
	int i = c.size()-1;
	m_terms.reserve(i+1);
	for (double coef : c) {
		if (coef != 0.0) {
			m_terms.push_back({i,coef});
		}
		i--;
	}
	m_terms.shrink_to_fit();
}

Polinomial::~Polinomial(){}

Polinomial Polinomial::m_add(const Polinomial b) const {
	Polinomial res = b;
	for (term i : m_terms) {
		vec_term t = res.get_term(i.m_order);
		if (t.m_pos > res.m_terms.size()-1) {res.m_terms.push_back({i.m_order,0});}
		res.m_terms[t.m_pos].m_coeficient = t.m_coeficient + i.m_coeficient;
	}
	res.sort();
	return res;
};

Polinomial Polinomial::add_term(const term b) const {
	Polinomial res = *this;
	vec_term t = res.get_term(b.m_order);
	if (t.m_pos > res.m_terms.size()) {return res;}
	if (t.m_pos == res.m_terms.size()) {res.m_terms.push_back({b.m_order,0});}
	res.m_terms[t.m_pos].m_coeficient = t.m_coeficient + b.m_coeficient;
	res.sort();
	return res;
};

Polinomial Polinomial::operator+(const Polinomial& b){
	return m_add(b);
}

Polinomial Polinomial::operator+(const term& b){
	return add_term(b);
}

Polinomial Polinomial::operator+=(const Polinomial& b){
	*this = m_add(b);
	return *this;
}

Polinomial Polinomial::operator+=(const term& b){
	*this = add_term(b);
	return *this;
}

Polinomial Polinomial::operator-() const {
	return *this*(-1.0);
}

Polinomial Polinomial::operator-(const Polinomial& b){
	return m_add(-b);
}

Polinomial Polinomial::operator-(const term& b){
	return add_term(-b);
}

Polinomial Polinomial::operator-=(const Polinomial& b){
	*this = m_add(-b);
	return *this;
}

Polinomial Polinomial::operator-=(const term& b){
	*this = add_term(-b);
	return *this;
}

bool esentialy_equal(double a, double b){
	return std::fabs(a-b)
		<= (std::min(fabs(a),fabs(b)))
		* std::numeric_limits<double>::epsilon();
}

bool practicaly_equal(double a, double b){
	return std::fabs(a-b)
		<= (std::max(fabs(a),fabs(b))) *
		std::numeric_limits<double>::epsilon();
}

bool Polinomial::operator==(const Polinomial& b){
	if (m_terms.size() != b.m_terms.size()){ return false; }
	for (term i : b.m_terms) {
		double a = i.m_coeficient;
		double b = get_term(i.m_order).m_coeficient;
		if (!esentialy_equal(a, b)) { return false; }	
	}
	return true;
}

Polinomial::vec_term Polinomial::get_term(int order){
	uint32_t index = 0;
	for (term i : m_terms) {
		if (i.m_order == order) {
			return {index, i.m_coeficient};
		}
		index++;
	}
	return {index, 0};
}

void Polinomial::print(){
	for (term i : m_terms) {
		printf("%.2fX^%d + ", i.m_coeficient, i.m_order);
	}
	printf("0\n");
}

void Polinomial::vec_init(std::vector<term> t){
	m_terms.clear();
	m_terms.reserve(t.size());
	for (term i : t) {
		if (i.m_coeficient != 0.0) {
			m_terms.push_back(i);
		}
	}
	m_terms.shrink_to_fit();
}

Polinomial &Polinomial:: operator=(Polinomial &&a){
	vec_init(a.m_terms);
	return *this;
}

Polinomial &Polinomial:: operator=(const Polinomial &a){
	vec_init(a.m_terms);
	return *this;
}

Polinomial Polinomial::m_multiplication(const Polinomial b) const {
	Polinomial ret;
	ret.m_terms.reserve(m_terms.size()+b.m_terms.size());
	for (term i : m_terms) {
		for (term j : b.m_terms) {
			ret+=(i*j);
		}
	}
	ret.m_terms.shrink_to_fit();
	return ret;
}

Polinomial::term Polinomial::term::operator*(term b){
	return {
		.m_order = m_order+b.m_order,
		.m_coeficient = m_coeficient*b.m_coeficient,
	};
}

Polinomial::term Polinomial::term::operator*(double b){
	return {
		.m_order = m_order,
		.m_coeficient = m_coeficient*b,
	};
}

Polinomial::term Polinomial::term::operator-() const {
	return {
		.m_order = m_order,
		.m_coeficient = -m_coeficient,
	};
}

Polinomial::term Polinomial::term::operator/(term b){
	return {
		.m_order = m_order-b.m_order,
		.m_coeficient = m_coeficient/b.m_coeficient,
	};
}

Polinomial Polinomial::operator*(const Polinomial& b){
	return m_multiplication(b);
}

Polinomial Polinomial::operator*(const term& b){
	Polinomial p({b});
	return m_multiplication(p);
}

Polinomial Polinomial::term::operator*(const Polinomial& b){
	Polinomial p({*this});
	return b.m_multiplication(p);
}

Polinomial Polinomial::operator*(const double b) const {
	double a[] = {b};
	Polinomial p(a);
	return this->m_multiplication(p);
}

void Polinomial::sort() {
	std::sort(m_terms.begin(), m_terms.end(), []
			(const Polinomial::term &a, const Polinomial::term &b)
			{ return (a.m_order>b.m_order); });
}

std::tuple<Polinomial, Polinomial> Polinomial::m_division(Polinomial d) {
	sort();
	d.sort();
	Polinomial q;
	Polinomial r = *this;
	while (r.m_terms.size() != 0 && r.m_terms[0].m_order >= d.m_terms[0].m_order){
		term t = r.m_terms[0]/d.m_terms[0];
		q += t;
		r -= t*d;
	}
	return {q, r};
}

std::tuple<Polinomial, Polinomial> Polinomial::operator/(Polinomial& b) noexcept(false){
	if (b.m_terms.size() == 0) {
		ZeroDivision e;
		throw e;
	}
	return m_division(b);
}

double Polinomial::m_evaluate(const double x) const {
	double ret = 0.0;
	for (term i: m_terms) {
		ret += std::pow(x, i.m_order) * i.m_coeficient;
	}
	return ret;
}

double Polinomial::operator()(const double x) const {
	return m_evaluate(x);
}

std::complex<double> Polinomial::m_evaluate(const std::complex<double> x) const {
	std::complex<double> ret = 0.0;
	for (term i: m_terms) {
		ret += std::pow(x, i.m_order) * i.m_coeficient;
	}
	return ret;
}

std::complex<double> Polinomial::operator()(const std::complex<double> x) const {
	return m_evaluate(x);
}

Polinomial Polinomial::m_evaluate(const Polinomial p) const {
	Polinomial ret;
	ret.m_terms.reserve(m_terms.size()*p.m_terms.size());
	for (term i: m_terms) {
		for (term j : p.m_terms) {
			term n = {
				.m_order = i.m_order * j.m_order,
				.m_coeficient = std::pow(j.m_coeficient, i.m_order) * i.m_coeficient,
			};
			ret+=n;
		}
	}
	ret.m_terms.shrink_to_fit();
	return ret;
}

Polinomial Polinomial::operator()(const Polinomial p) const {
	return m_evaluate(p);
}

Polinomial::ZeroDivision::ZeroDivision(): std::exception() {}

Polinomial::ZeroDivision::~ZeroDivision(){}

Polinomial::ZeroDivision::ZeroDivision(ZeroDivision && copy) {}

Polinomial::ZeroDivision::ZeroDivision(const ZeroDivision & copy) {}

const char * Polinomial::ZeroDivision::what() const noexcept(true){
	return msg;
}

Polinomial Polinomial::derivate() const {
	Polinomial ret;
	ret.m_terms.reserve(m_terms.size());
	for (term i : m_terms) {
		term n = {
			.m_order = i.m_order-1,
			.m_coeficient = i.m_coeficient * i.m_order,
		};
		ret+=n;
	}
	ret.m_terms.shrink_to_fit();
	return ret;
}

Polinomial Polinomial::operator*() const {
	return derivate();
}

#include <iostream>
std::vector<std::complex<double>> Polinomial::aberth_roots(double min_change, uint32_t max_iter) const {
	Polinomial der = **this;
	int deg = m_terms[0].m_order;
	std::vector<std::complex<double>> ret;
	ret.reserve(deg);
	constexpr auto π_2 {std::numbers::pi / 2.0};
   constexpr auto mag {1.0};
	//Initial aproximations on a quarte of the unity circle
	for (int i=0; i<deg; i++) {
		const auto θ {i * π_2/deg};
		std::complex<double> a = std::polar(mag, θ);
		ret.push_back(a);
	}
	ret.shrink_to_fit();
	double change = 1.0;
	uint32_t iter_cont = 0;
	while (!practicaly_equal(change, 0.0) && change>min_change && iter_cont<max_iter) {
		iter_cont++;
		change = 0.0;
		int zindex = 0;
		for (std::complex<double> &zi : ret) {
			std::complex<double> aux = (*this)(zi)/der(zi);
			std::complex<double> sum = 0.0;
			for (std::complex<double> zj : ret) {
				sum += (zi!=zj)? (1.0/(zi-zj)):0.0;
			}
			std::complex<double> wi = (aux)/(1.0-(aux*sum));
			//Ocational random noise to scape from oscilating values
			if (!(iter_cont%10000)) {
				double r_n = ((double)std::rand()/RAND_MAX)
					*3
					*std::numeric_limits<double>::epsilon()
					*abs(wi);
				std::complex<double> r_w(r_n,r_n);
				wi += r_w;
			}
			change = std::max(change, abs(wi));
			zi-=wi;
		}
	}
	std::sort(ret.begin(), ret.end(), []
			(const std::complex<double> &a, const std::complex<double> &b)
			{ return (a.real()<b.real()); });
	return ret;
}

std::vector<std::complex<double>> Polinomial::roots() const {
	return aberth_roots(0.0, std::numeric_limits<uint32_t>::max());
}

std::vector<double> Polinomial::r_roots() const {
	std::vector<double> ret;
	std::vector<std::complex<double>> c_roots;
	ret.reserve(c_roots.size());
	c_roots = roots();
	for (std::complex<double> i : c_roots) {
		if (practicaly_equal(i.imag(), 0.0)) {
			ret.push_back(i.real());
		}	
	}
	ret.shrink_to_fit();
	return ret;
}

std::vector<std::complex<double>> Polinomial::roots(double min_change) const {
	return aberth_roots(min_change, std::numeric_limits<uint32_t>::max());
}

std::vector<double> Polinomial::r_roots(double min_change) const {
	std::vector<double> ret;
	std::vector<std::complex<double>> c_roots;
	ret.reserve(c_roots.size());
	c_roots = roots(min_change);
	for (std::complex<double> i : c_roots) {
		if (practicaly_equal(i.imag(), 0.0)) {
			ret.push_back(i.real());
		}	
	}
	ret.shrink_to_fit();
	return ret;
}

std::vector<std::complex<double>> Polinomial::roots(uint32_t max_iter) const {
	return aberth_roots(0.0, max_iter);
}

std::vector<double> Polinomial::r_roots(uint32_t max_iter) const {
	std::vector<double> ret;
	std::vector<std::complex<double>> c_roots;
	ret.reserve(c_roots.size());
	c_roots = roots(max_iter);
	for (std::complex<double> i : c_roots) {
		if (practicaly_equal(i.imag(), 0.0)) {
			ret.push_back(i.real());
		}	
	}
	ret.shrink_to_fit();
	return ret;
}

uint32_t Polinomial::degree(){
	return m_terms.size()>0? m_terms[0].m_order:0;
}
