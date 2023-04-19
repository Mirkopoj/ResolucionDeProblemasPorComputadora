#include "../Include/date.hpp"
#include <iostream>

Date::Date(int d, int m, int y):
	day(d), month(m), year(y) {}

Date::~Date(){}

void Date::print(){
	std::cout << day << "/" << month << "/" << year << "\n";
}

int Date::get_year(){
	return year;
}

int Date::get_month(){
	return month;
}

int Date::get_day(){
	return day;
}
