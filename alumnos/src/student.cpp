#include "../Include/student.hpp"
#include <ctime>
#include <iostream>
#include <iterator>

Student::Student(std::string n, std::string s, Date d):
	name(n), surname(s), date_of_birth(d) { 
		mayors = new MayorsList;
		passed_assingments = new AssingmentsList;
	}

Student::~Student(){
	delete mayors;
	delete passed_assingments;
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
	if (  now->tm_mon - date_of_birth.get_month() < 0 ||
			now->tm_mday - date_of_birth.get_day()) {
		years--;
	} 

	return years;
}
