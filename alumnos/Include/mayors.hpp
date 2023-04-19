#pragma once
#include "core_node.hpp"
#include <string>

enum class Mayor {
	ElectronisEngineering,
	TelecomunicationsEngineering,
	ComputingEngineering,
	EnviromentalEngineering,
};

extern std::string MayorNames[];

class MayorNode:public CoreNode
{
public:
	MayorNode(Mayor m);
	MayorNode(MayorNode &&) = default;
	MayorNode(const MayorNode &) = default;
	MayorNode &operator=(MayorNode &&) = default;
	MayorNode &operator=(const MayorNode &) = default;
	~MayorNode();

	MayorNode *get_next();
	MayorNode *remove();
	Mayor get_mayor();
	void print();

private:
	Mayor mayor;
};
