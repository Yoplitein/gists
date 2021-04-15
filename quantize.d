import std.traits;

To quantize(To, From)(From val)
if(isFloatingPoint!From && isIntegral!To && isSigned!To)
in(val >= -1.0 && val <= 1.0)
{
	return cast(To)(
		(To.max - To.min) // unsigned equivalent's .max
		* ((val + 1) / 2) // val from [-1.0, 1.0] -> [0.0, 1.0]
		+ To.min // shift back into signed value range
	);
}

unittest
{
	const float[byte] tests = [
		-128: -1.0,
		-96: -0.75,
		-64: -0.5,
		-32: -0.25,
		0: -0.0,
		31: 0.25,
		63: 0.5,
		95: 0.75,
		127: 1.0,
	];
	foreach(k, v; tests)
		assert(quantize!byte(v) == k);
}