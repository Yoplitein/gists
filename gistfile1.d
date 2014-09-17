void ensure_throws(ExceptionType = Exception)(void delegate() callable, string message = "")
{
    try
    {
        callable();
        assert(false, message);
    }
    catch(ExceptionType err) {}
    catch(Exception)
    {
        assert(false, "callable threw an unexpected exception");
    }
}