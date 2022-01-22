static void writefln(String fmt, Object... args)
{
	System.out.println(
		args.length == 0 ?
			fmt :
			String.format(fmt, args)
	);
}