#pragma once

class Date
{
public:
	Date();
	Date(Date &&) = default;
	Date(const Date &) = default;
	Date &operator=(Date &&) = default;
	Date &operator=(const Date &) = default;
	~Date();

private:
	int day;
	int month;
	int year;
};
