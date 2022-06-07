struct Set(T)
{
	private import std.range: empty, enumerate;
	
	private alias This = typeof(this);
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
	
	void clear()
	{
		set.clear;
	}
	
	bool empty() const
	{
		return set.empty;
	}
	
	auto range() inout
	{
		return set.byKey;
	}
	
	This dup() const
	{
		// no AA .dup overload for const -> mutable
		This result;
		foreach(v; range) result.add(v);
		return result;
	}
	
	This union_(const This rhs) const
	{
		This result = dup;
		foreach(v; rhs.range) result.add(v);
		return result;
	}
	
	This intersection(const This rhs) const
	{
		This result;
		foreach(v; rhs.range)
			if(v in this) result.add(v);
		return result;
	}
	
	This difference(const This rhs) const
	{
		This result = dup;
		foreach(v; rhs.range) result.remove(v);
		return result;
	}
	
	void opOpAssign(string op)(T v)
	if(op == "+" || op == "-")
	{
		static if(op == "+")
			add(v);
		else
			remove(v);
	}
	
	bool opBinaryRight(string op)(T v) const
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
	
	static bool setEqual(Set!int set, int[] expected)
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
	
	auto set2 = set.dup;
	assert(setEqual(set2, [1, 3]));
	set2.remove(3);
	assert(setEqual(set2, [1]));
	assert(setEqual(set, [1, 3]));
	
	set2.clear;
	assert(set2.empty);
	
	set.clear;
	foreach(v; 0 .. 3) set += v;
	foreach(v; 1 .. 4) set2 += v;
	assert(setEqual(set.union_(set2), [0, 1, 2, 3]));
	assert(setEqual(set2.union_(set), [0, 1, 2, 3]));
	assert(setEqual(set.intersection(set2), [1, 2]));
	assert(setEqual(set2.intersection(set), [1, 2]));
	assert(setEqual(set.difference(set2), [0]));
	assert(setEqual(set2.difference(set), [3]));
}