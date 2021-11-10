alias id(alias x) = x;

struct seq(_xs...)
{
    alias xs = _xs;
}

enum isSeq(alias x) = is(x: seq!xs, xs...);

template seqAppend(seqs...)
{
    static if(seqs.length == 0)
    	alias seqAppend = seq!();
    else
    {
        alias res = seq!();
        static foreach(_seq; seqs)
			res = seq!(res.xs, _seq.xs);
		alias seqAppend = res;
	}
}

template seqJoin(alias _seq, bool deep = false)
{
	alias res = seq!();
	static foreach(x; _seq.xs)
		static if(isSeq!x)
		{
			static if(deep)
				res = seq!(res.xs, seqJoin!x.xs);
			else
				res = seq!(res.xs, x.xs);
		}
		else
			res = seq!(res.xs, x);
	alias seqJoin = res;
}

template seqMap(alias fn, alias _seq)
{
    alias res = seq!();
    static foreach(x; _seq.xs)
		res = seq!(res.xs, fn!x);
    alias seqMap = res;
}

template seqFilter(alias pred, alias _seq)
{
    alias res = seq!();
    static foreach(x; _seq.xs)
		static if(pred!x)
			res = seq!(res.xs, x);
    alias seqFilter = res;
}

template seqReduce(alias fn, alias _seq, alias seed = void)
{
	static if(_seq.xs.length < 2)
		alias seqReduce = _seq;
	else
	{
		static if(is(seed == void))
		{
			alias val = id!(_seq.xs[0]);
			enum start = 1;
		}
		else
		{
			alias val = seed;
			enum start = 0;
		}
		
		static foreach(x; _seq.xs[start .. $])
			val = fn!(val, x);
		
		alias seqReduce = val;
	}
}