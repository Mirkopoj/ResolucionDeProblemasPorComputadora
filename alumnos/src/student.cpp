#include "../Include/student.hpp"
#include <ctime>
#include <iostream>
#include <iterator>

Student::Student(std::string n, std::string s, Date d):
	name(n), surname(s), date_of_birth(d) { 
		mayors = new MayorsList;
		passed_assingments = new AssingmentsList;
		rc = new unsigned int(1);
}

Student::Student(const Student &original):
	name(original.name),
	surname(original.surname),
	date_of_birth(original.date_of_birth),
	mayors(original.mayors),
	passed_assingments(original.passed_assingments),
	rc(original.rc)
{
	(*rc)++;
}

Student::~Student(){
	(*rc)--;
	if (*rc == 0){
		delete rc;
		delete mayors;
		delete passed_assingments;
	}
}

void Student::afiliate_to(Mayor mayor){
	MayorNode *mayor_node = new MayorNode(mayor);
	mayors->add(mayor_node);
}

void Student::print(){
	std::cout << "Name: " << name << "\n";
	std::cout << "Surname: " << surname << "\n";
	std::cout << "Date of birth: "; date_of_birth.print();
	std::cout << "Mayors: "; mayors->print();
	std::cout << "Assingments: "; passed_assingments->print();
}

void Student::mark_as_passed(Assingment assingment){
	AssingmentNode *assingment_node = new AssingmentNode(assingment);
	passed_assingments->add(assingment_node);
}

int Student::age(){
	std::time_t t = std::time(0);
	std::tm *now = std::localtime(&t);
	
	int years = now->tm_year - date_of_birth.get_year() + 1900;
	int month_dif = now->tm_mon+1 - date_of_birth.get_month();
	if ( month_dif < 0){
		years--;
		return years;
	} 

	if (  month_dif == 0 &&
			now->tm_mday - date_of_birth.get_day() < 0) {
		years--;
	}

	return years;
}
