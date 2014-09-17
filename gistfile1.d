void ensure_throws(ExceptionType = Exception)(void delegate() callable, string message = "")
{
    import core.exception: AssertError;
    
    try
    {
        callable();
        assert(false, message);
    }
    catch(ExceptionType err) {}
    catch(AssertError err)
        throw err;
    catch(Throwable err)
            assert(false, "callable threw an unexpected exception");
}