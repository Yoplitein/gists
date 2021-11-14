struct Set(T)
{
	private alias Nil = void[0]; // superior to empty struct, has .sizeof == 0
	private Nil[T] set;
	
	void add(T v)
	{
		set[v] = Nil.init;
	}
	
	void remove(T v)
	{
		set.remove(v);
	}
	
	bool opBinaryRight(string op)(T v)
	if(op == "in")
	{
		return (v in set) != null;
	}
	
	auto range()
	{
		return set.keys;
	}
	
	int opApply(scope int delegate(T) fn)
	{
		return opApplyImpl!false((_, v) => fn(v));
	}
	
	int opApply(scope int delegate(size_t, T) fn)
	{
		return opApplyImpl!false(fn);
	}
	
	int opApplyReverse(scope int delegate(T) fn)
	{
		return opApplyImpl!true((_, v) => fn(v));
	}
	
	int opApplyReverse(scope int delegate(size_t, T) fn)
	{
		return opApplyImpl!true(fn);
	}
	
	private int opApplyImpl(bool reverse)(scope int delegate(size_t, T) fn)
	{
		enum inner = "if(auto ret = fn(i, v) != 0) return ret;";
		mixin("foreach", reverse ? "_reverse" : "", "(i, v; range) ", inner);
		return 0;
	}
}

unittest
{
	import std.algorithm: equal, sort;
	
	auto set = Set!int();
	set.add(1);
	set.add(2);
	set.add(3);
	
	assert(1 in set && 2 in set && 3 in set);
	assert(set.range.sort.equal([1, 2, 3]));
	
	set.remove(2);
	assert(2 !in set);
	assert(set.range.sort.equal([1, 3]));
	
	foreach(v; set)
		assert(v == 1 || v == 3);
	foreach_reverse(v; set)
		assert(v == 1 || v == 3);
	foreach(i, v; set)
		assert((i == 0 || i == 1) && (v == 1 || v == 3));
	foreach_reverse(i, v; set)
		assert((i == 0 || i == 1) && (v == 1 || v == 3));
}