function class(ctor, members)
    local Class = {}
    Class.__index = Class
    
    for k, v in pairs(members) do
        if k == "__index" then
            error("Cannot overwrite __index on class")
        end
        
        Class[k] = v
    end
    
    setmetatable(
        Class,
        {
            __call = function(metatable, ...)
                local instance = {}
                
                setmetatable(instance, Class)
                ctor(instance, ...)
                
                return instance
            end
        }
    )
    
    return Class
end