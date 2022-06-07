struct Set(T)
{
	private import std.range: empty, enumerate;
	
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
	
	bool empty()
	{
		return set.empty;
	}
	
	auto range()
	{
		return set.byKey;
	}
	
	void opOpAssign(string op)(T v)
	if(op == "+" || op == "-")
	{
		static if(op == "+")
			add(v);
		else
			remove(v);
	}
	
	bool opBinaryRight(string op)(T v)
	if(op == "in")
	{
		return (v in set) != null;
	}
	
	int opApply(scope int delegate(T) fn)
	{
		return opApply((_, v) => fn(v));
	}
	
	int opApply(scope int delegate(size_t, T) fn)
	{
		foreach(i, v; range.enumerate)
			if(auto ret = fn(i, v) != 0)
				return ret;
		return 0;
	}
}

unittest
{
	import std: array, equal, sort;
	
	bool setEqual(ref Set!int set, int[] expected)
	{
		return set
			.range
			.array
			.sort
			.equal(expected)
		;
	}
	
	auto set = Set!int();
	assert(set.empty);
	
	set.add(1);
	set.add(2);
	set.add(2);
	set.add(3);
	set += 4;
	assert(1 in set && 2 in set && 3 in set && 4 in set);
	assert(setEqual(set, [1, 2, 3, 4]));
	assert(!set.empty);
	
	set.remove(2);
	set -= 4;
	assert(2 !in set && 4 !in set);
	assert(setEqual(set, [1, 3]));
	
	foreach(v; set)
		assert(v == 1 || v == 3);
	foreach(i, v; set)
		assert((i == 0 || i == 1) && (v == 1 || v == 3));
}