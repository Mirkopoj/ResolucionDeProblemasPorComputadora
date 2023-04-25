#include "../../Include/student.hpp"
#include <iostream>

int main() { 
	Student mirko("Mirko", "Pojmaevich", {9,9,2000});

	mirko.afiliate_to(Mayor::ElectronisEngineering);

	mirko.mark_as_passed({"ILEA", 6});
	mirko.mark_as_passed({"Analisis", 7});
	mirko.mark_as_passed({"RDPPC", 11});

	mirko.print();

	std::cout<<mirko.age()<<std::endl;
	
	return 0; 
}
