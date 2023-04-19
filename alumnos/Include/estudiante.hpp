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
	Student();
	Student(Student &&) = default;
	Student(const Student &) = default;
	Student &operator=(Student &&) = default;
	Student &operator=(const Student &) = default;
	~Student();

	bool afiliate_to(Mayors mayor);

private:
	std::string name;
	std::string surname;
	Date data_of_birth;
	MayorsList mayor;
	AssingmentsList passed_assingments;
};
