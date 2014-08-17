AttrType[] find_attributes(AttrType, alias Thing)()
{
    AttrType[] result;
    
    foreach(attr; __traits(getAttributes, Thing))
        static if(is(typeof(attr) == AttrType))
            result ~= [attr];
    
    return result;
}