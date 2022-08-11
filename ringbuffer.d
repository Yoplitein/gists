struct RingBuffer(T, size_t _capacity)
{
	import core.lifetime: move;
	
	private
	{
		T[capacity] buffer;
		size_t read;
		size_t write;
		size_t len; // keep it simple :^)
	}
	
	enum capacity = _capacity;
	
	size_t length() { return len; }
	
	void push(T rhs)
	{
		if(len == capacity) pop;
		buffer[write++] = move(rhs);
		write %= capacity;
		len++;
	}
	
	T pop()
	{
		scope(exit) read %= capacity;
		len--;
		return move(buffer[read++]);
	}
	
	auto range()
	{
		static struct Range
		{
			T[] buffer;
			size_t read, len;
			
			bool empty() { return len == 0; }
			
			ref T front()
			in(!empty)
			{
				return buffer[read];
			}
			
			void popFront()
			{
				len--;
				read = (read + 1) % capacity;
			}
		}
		return Range(buffer[], read, len);
	}
}

unittest
{
	import std.algorithm: equal;
	
	RingBuffer!(int, 3) ints;
	static assert(ints.capacity == 3);
	assert(ints.length == 0);
	
	ints.push(1);
	assert(ints.length == 1);
	assert(ints.range.equal([1]));
	
	ints.push(2);
	ints.push(3);
	assert(ints.length == 3);
	assert(ints.range.equal([1, 2, 3]));
	
	ints.push(4);
	assert(ints.length == 3);
	assert(ints.range.equal([2, 3, 4]));
	
	assert(ints.pop() == 2);
	assert(ints.pop() == 3);
	assert(ints.pop() == 4);
	assert(ints.length == 0);
}