#include "../Include/polinomials.hpp"
#include <cmath>
#include <criterion/hooks.h>
#include <cstdint>
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
	terms.reserve(i+1);
	for (double coef : c) {
		if (coef != 0.0) {
			terms.push_back({i,coef});
		}
		i--;
	}
	terms.shrink_to_fit();
}

Polinomial::~Polinomial(){}

Polinomial Polinomial::add(const Polinomial b) const {
	Polinomial res = b;
	for (term i : terms) {
		vec_term t = res.get_term(i.order);
		if (t.pos > res.terms.size()-1) {res.terms.push_back({i.order,0});}
		res.terms[t.pos].coeficient = t.coeficient + i.coeficient;
	}
	res.sort();
	return res;
};

Polinomial Polinomial::add_term(const term b) const {
	Polinomial res = *this;
	vec_term t = res.get_term(b.order);
	if (t.pos > res.terms.size()) {return res;}
	if (t.pos == res.terms.size()) {res.terms.push_back({b.order,0});}
	res.terms[t.pos].coeficient = t.coeficient + b.coeficient;
	res.sort();
	return res;
};

Polinomial Polinomial::operator+(const Polinomial& b){
	return add(b);
}

Polinomial Polinomial::operator+(const term& b){
	return add_term(b);
}

Polinomial Polinomial::operator+=(const Polinomial& b){
	*this = add(b);
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
	return add(-b);
}

Polinomial Polinomial::operator-(const term& b){
	return add_term(-b);
}

Polinomial Polinomial::operator-=(const Polinomial& b){
	*this = add(-b);
	return *this;
}

Polinomial Polinomial::operator-=(const term& b){
	*this = add_term(-b);
	return *this;
}

bool Polinomial::operator==(const Polinomial& b){
	if (terms.size() != b.terms.size()){ return false; }
	for (term i : b.terms) {
		double a = i.coeficient;
		double b = get_term(i.order).coeficient;
		if (std::fabs(a-b) > (std::min(fabs(a),fabs(b))) * std::numeric_limits<double>::epsilon()) { return false; }	
	}
	return true;
}

Polinomial::vec_term Polinomial::get_term(int order){
	uint32_t index = 0;
	for (term i : terms) {
		if (i.order == order) {
			return {index, i.coeficient};
		}
		index++;
	}
	return {index, 0};
}

void Polinomial::print(){
	for (term i : terms) {
		printf("%.2fX^%d + ", i.coeficient, i.order);
	}
	printf("0\n");
}

void Polinomial::vec_init(std::vector<term> t){
	terms.clear();
	terms.reserve(t.size());
	for (term i : t) {
		if (i.coeficient != 0.0) {
			terms.push_back(i);
		}
	}
	terms.shrink_to_fit();
}

Polinomial &Polinomial:: operator=(Polinomial &&a){
	vec_init(a.terms);
	return *this;
}

Polinomial &Polinomial:: operator=(const Polinomial &a){
	vec_init(a.terms);
	return *this;
}

Polinomial Polinomial::multiplication(const Polinomial b) const {
	Polinomial ret;
	ret.terms.reserve(terms.size()+b.terms.size());
	for (term i : terms) {
		for (term j : b.terms) {
			ret+=(i*j);
		}
	}
	ret.terms.shrink_to_fit();
	return ret;
}

Polinomial::term Polinomial::term::operator*(term b){
	return {
		.order = order+b.order,
		.coeficient = coeficient*b.coeficient,
	};
}

Polinomial::term Polinomial::term::operator*(double b){
	return {
		.order = order,
		.coeficient = coeficient*b,
	};
}

Polinomial::term Polinomial::term::operator-() const {
	return {
		.order = order,
		.coeficient = -coeficient,
	};
}

Polinomial::term Polinomial::term::operator/(term b){
	return {
		.order = order-b.order,
		.coeficient = coeficient/b.coeficient,
	};
}

Polinomial Polinomial::operator*(const Polinomial& b){
	return multiplication(b);
}

Polinomial Polinomial::operator*(const term& b){
	Polinomial p({b});
	return multiplication(p);
}

Polinomial Polinomial::term::operator*(const Polinomial& b){
	Polinomial p({*this});
	return b.multiplication(p);
}

Polinomial Polinomial::operator*(const double b) const {
	double a[] = {b};
	Polinomial p(a);
	return this->multiplication(p);
}

void Polinomial::sort() {
	std::sort(terms.begin(), terms.end(), []
			(const Polinomial::term &a, const Polinomial::term &b)
			{ return (a.order>b.order); });
}

std::tuple<Polinomial, Polinomial> Polinomial::division(Polinomial d) {
	sort();
	d.sort();
	Polinomial q;
	Polinomial r = *this;
	while (r.terms.size() != 0 && r.terms[0].order >= d.terms[0].order){
		term t = r.terms[0]/d.terms[0];
		q += t;
		r -= t*d;
	}
	return {q, r};
}

std::tuple<Polinomial, Polinomial> Polinomial::operator/(Polinomial& b){
	return division(b);
}

#include <iostream>
#include <iomanip>
double Polinomial::evaluate(const double x) const {
	double ret = 0.0;
	for (term i: terms) {
		ret += std::pow(x, i.order) * i.coeficient;
	}
	return ret;
}

double Polinomial::operator()(const double x) const {
	return evaluate(x);
}
