#pragma once

class CoreNode
{
public:
	CoreNode(...);
	CoreNode(CoreNode &&) = default;
	CoreNode(const CoreNode &) = default;
	CoreNode &operator=(CoreNode &&) = default;
	CoreNode &operator=(const CoreNode &) = default;
	virtual ~CoreNode();

	CoreNode *get_next();
	void append(CoreNode *next);;
	CoreNode *remove();
	virtual void print() = 0;

private:
	CoreNode *next;
};
