#include <criterion/criterion.h>
#include <criterion/internal/assert.h>
#include <tuple>

#include "../../Include/polinomials.hpp"

Test(add_operand, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	Polinomial b({{5,1.0},{0,3.5},{2,-0.5}});
	Polinomial c = a+b;
	Polinomial r({{1.0, 4.0, 3.0, 1.5, 0.0, 3.5}});
	cr_assert_eq(c, r);
}

Test(equal_operand_true, test) {
	Polinomial a({{3.0, 2.0, 1.0, 0.0}});
	Polinomial b({{3,3.0},{2,2.0},{1,1.0},{0,0.0}});
	cr_assert_eq(a, b);
}

Test(equal_operand_false, test) {
	Polinomial a({{3.0, 2.0, 1.0, 0.0}});
	Polinomial b({{2,2.0},{1,1.0},{0,0.0}});
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

#include <iostream>
#include <iomanip>
#include <math.h>
Test(evaluation_operand, test) {
	Polinomial a({{4.0, 3.0, 2.0, 0.0, 0.0}});
	double r = a(7.35);
	double expected = 12972.96315;
	cr_assert(std::fabs(expected-r) < (std::min(fabs(expected),fabs(r))) * std::numeric_limits<double>::epsilon());
}

