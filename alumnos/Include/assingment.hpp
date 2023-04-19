#pragma once
#include "core_node.hpp"
#include <string>

class Assingment
{
public:
	Assingment(std::string name, int grade);
	Assingment(Assingment &&) = default;
	Assingment(const Assingment &) = default;
	Assingment &operator=(Assingment &&) = default;
	Assingment &operator=(const Assingment &) = default;
	~Assingment();

	std::string get_name();
	int get_mark();

private:
	std::string name;
	int grade;
};

class AssingmentNode:public CoreNode
{
public:
	AssingmentNode(Assingment assingment);
	AssingmentNode(AssingmentNode &&) = default;
	AssingmentNode(const AssingmentNode &) = default;
	AssingmentNode &operator=(AssingmentNode &&) = default;
	AssingmentNode &operator=(const AssingmentNode &) = default;
	~AssingmentNode();

	AssingmentNode *get_next();
	AssingmentNode *remove();
	Assingment get_assingment();

private:
	Assingment assingment;
};
