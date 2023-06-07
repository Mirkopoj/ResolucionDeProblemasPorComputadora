#include <complex>
#include <criterion/criterion.h>
#include <criterion/internal/assert.h>
#include <cstdint>
#include <limits>
#include <tuple>
#include <vector>

#include "../../Include/polinomials.hpp"

Test(add_operand, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{5,1.0},{0,3.5},{2,-0.5}});
	Polinomial c = a+b;
	Polinomial r({{1.0, 4.0, 3.0, 1.5, 0.0, 3.5}});
	cr_assert_eq(c, r);
}

Test(add_operand_zero, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b;
	Polinomial c = a+b;
	cr_assert_eq(c, a);
}

Test(equal_operand_true, test) {
	Polinomial a({{3.0, 2.0, 1.0, 0.0}});
	Polinomial b({{3,3.0},{2,2.0},{1,1.0},{0,0.0}});
	cr_assert_eq(a, b);
}

Test(equal_operand_false, test) {
	Polinomial a({{3.0, 2.0, 1.0, 1.0}});
	Polinomial b({{3,3.0},{2,2.0},{1,1.0},{0,0.0}});
	cr_assert_neq(a, b);
}

Test(multiplication_operand_scalar, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial c = a*4.2;
	Polinomial r({{16.8, 12.6, 8.4, 0.0, 0.0}});
	cr_assert_eq(c, r);
}

Test(multiplication_operand_poli, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{2,2.0},{1,1.0},{0,0.0}});
	Polinomial c = a*b;
	Polinomial r({{3,2.0},{4,7.0},{5,10.0},{6,8.0}});
	cr_assert_eq(c, r);
}

Test(divition_operand, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{2,2.0},{1,1.0},{0,0.0}});
	std::tuple<Polinomial, Polinomial> c = a/b;
	Polinomial q({{2,2.0},{1,0.5},{0,0.75}});
	double f[] = {-0.75, 0.0};
	Polinomial r(f);
	Polinomial cq = std::get<0>(c);
	Polinomial cr = std::get<1>(c);
	cr_assert_eq(cq, q);
	cr_assert_eq(cr, r);
}

Test(divition_operand_exception, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{0.0, 0.0, 0.0}});
	bool exception = false;
	try{
		std::tuple<Polinomial, Polinomial> c = a/b;
	}
	catch(Polinomial::ZeroDivision){
		exception = true;
	}
	cr_assert(exception);
}

#include <math.h>
Test(evaluation_operand_scalar, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	double r = a(7.35);
	double expected = 12972.96315;
	cr_assert(std::fabs(expected-r) < (std::min(fabs(expected),fabs(r))) * std::numeric_limits<double>::epsilon());
}

Test(evaluation_operand_poli, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{3.0, 0.0, 0.0}});
	Polinomial r = a(b);
	Polinomial expected({{8,324.0},{6,81.0},{4,18.0}});
	cr_assert_eq(expected, r);
}

Test(derivation_operand, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 1.0}});
	Polinomial r = *a;
	Polinomial expected({{3,16.0},{2,9.0},{1,4.0}});
	cr_assert_eq(expected, r);
}

Test(c_roots, test) {
	Polinomial a({{1.0, 1.0, -6.0, 1.0, -3.0}});
	std::vector<std::complex<double>> r = a.roots((uint32_t)1000);
	for (std::complex<double> i : r) {
		cr_assert(abs(a(i)) <= std::numeric_limits<double>::epsilon() * 35);
	}
}

Test(r_roots, test) {
	Polinomial a({{1.0, -15.0, 70.0, -120.0, 64.0}});
	std::vector<double> r = a.r_roots((uint32_t)1000);
	cr_assert_eq(r.size(), 4);
	for (double i : r) {
		cr_assert(abs(a(i)) <= std::numeric_limits<double>::epsilon() * 35);
	}
}

