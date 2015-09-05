function class(ctor, members)
    local Class = {}
    Class.__index = Class
    
    if members then
        if type(members) ~= "table" then
            error("Passed non-table type as class members")
        end
        
        for k, v in pairs(members) do
            if k == "__index" then
                error("Cannot overwrite __index on class")
            end
            
            Class[k] = v
        end
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

local Vector = class(
    function(self, x, y)
        self.x = x
        self.y = y
    end,
    {
        __eq = function(self, other)
            return
                self.x == other.x and
                self.y == other.y
        end
    }
)
local vec1 = Vector(0, 0)
local vec2 = Vector(1, 1)

assert(vec1 == Vector(0, 0))
assert(vec2 == Vector(1, 1))
assert(vec1 ~= vec2)