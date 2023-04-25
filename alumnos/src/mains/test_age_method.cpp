#include <criterion/criterion.h>
#include <cstdio>
#include <ctime>

#include "../../Include/student.hpp"

Test(Age_before_month, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1 - 1;
	int day = now->tm_mday;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 10);
}

Test(Age_before_day, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1;
	int day = now->tm_mday - 1;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 10);
}

Test(Age_before_day_and_month, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1 - 1;
	int day = now->tm_mday - 1;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 10);
}

Test(Age_before_day_after_month, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1 + 1;
	int day = now->tm_mday - 1;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 9);
}

Test(Age_before_month_after_day, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1 - 1;
	int day = now->tm_mday + 1;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 10);
}

Test(Age_after_month, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1 + 1;
	int day = now->tm_mday;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 9);
}

Test(Age_after_day, test) {
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	int year = now->tm_year + 1900 - 10;
	int month = now->tm_mon+1;
	int day = now->tm_mday + 1;
	Student test("a", "b", {day, month, year});
	int age = test.age();
	cr_assert_eq(age, 9);
}
