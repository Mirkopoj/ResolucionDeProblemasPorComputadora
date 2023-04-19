#pragma once
#include <string>
#include "assingment.hpp"
#include "core_list.hpp"
#include "date.hpp"
#include "assingments_list.hpp"
#include "mayors.hpp"
#include "mayors_list.hpp"

class Student
{
public:
	Student(std::string n, std::string s, Date d);
	Student(Student &&) = default;
	Student(const Student &) = default;
	Student &operator=(Student &&) = default;
	Student &operator=(const Student &) = default;
	~Student();

	void afiliate_to(Mayor mayor);
	void mark_as_passed(Assingment assingment);
	void print();
	int age();

private:
	std::string name;
	std::string surname;
	Date date_of_birth;
	MayorsList *mayors;
	AssingmentsList *passed_assingments;
};
