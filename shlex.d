string[] shlex(string line)
{
    string[] result;
    string current;
    char quoteChr = 0;
    bool escape;
    
    foreach(chr; line)
    {
        switch(chr)
        {
            case ' ':
                if(escape)
                    goto default;
                
                if(quoteChr != 0)
                    goto default;
                
                if(current.length > 0)
                    result ~= current;
                
                current.length = 0;
                
                break;
            case '"':
            case '\'':
                if(escape)
                    goto default;
                
                if(quoteChr == 0)
                {
                    quoteChr = chr;
                    
                    break;
                }
                
                if(quoteChr == chr)
                {
                    quoteChr = 0;
                    
                    break;
                }
                
                goto default;
            case '\\':
                escape = true;
                
                break;
            default:
                current ~= chr;
                escape = false;
        }
    }
    
    if(quoteChr != 0)
        throw new Exception("Mismatched quotes!");
    
    if(current.length > 0)
        result ~= current;
    
    return result;
}

unittest
{
    const input = `abc def ghi`;
    const output = shlex(input);
    
    assert(output == ["abc", "def", "ghi"], "basic");
}

unittest
{
    const input = ` abc  def  ghi `;
    const output = shlex(input);
    
    assert(output == ["abc", "def", "ghi"], "extraneous spaces");
}

unittest
{
    const input = `abc \"def ghi`;
    const output = shlex(input);
    
    assert(output == ["abc", "\"def", "ghi"], "escaping");
}

unittest
{
    const input = `abc "def ghi" jkl`;
    const output = shlex(input);
    
    assert(output == ["abc", "def ghi", "jkl"], "quoting");
}


unittest
{
    const input = `abc 'def ghi'"'"'jkl' mno`;
    const output = shlex(input);
    
    assert(output == ["abc", "def ghi'jkl", "mno"], "quoting concatenation");
}
