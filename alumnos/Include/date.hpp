#pragma once

class Date
{
public:
	Date(int d, int m, int y);
	Date(Date &&) = default;
	Date(const Date &) = default;
	Date &operator=(Date &&) = default;
	Date &operator=(const Date &) = default;
	~Date();

	void print();
	int get_year();
	int get_month();
	int get_day();

private:
	int day;
	int month;
	int year;
};
