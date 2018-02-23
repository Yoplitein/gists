local SLOT_PICKAXE = 14
local SLOT_AXE = 15
local SLOT_COAL = 16
local SLOT_LAST = 12
local SLEEP_DELAY = 30

local component = require("component")
local robot = require("robot")
local inv = component.inventory_controller
local geo = component.geolyzer

function slotHasPurifiable(slot)
    local acceptable = {
        "stone",
        "log",
        "log2",
    }
    local stackInfo = inv.getStackInInternalSlot(slot)
    
    if not stackInfo then
        return false
    end
    
    for _, name in ipairs(acceptable) do
        if stackInfo.name == "minecraft:" .. name then
            return true
        end
    end
    
    return false
end

function place()
    local foundPurifiable = false
    
    for slot = 1, SLOT_LAST do
        if slotHasPurifiable(slot) then
            foundPurifiable = true
            
            robot.select(slot)
            
            break
        end
    end
    
    if foundPurifiable then
        robot.placeDown()
    end
    
    return foundPurifiable
end

function harvest(tool)
    local slot = -1
    
    if tool == "pickaxe" then
        slot = SLOT_PICKAXE
    elseif tool == "axe" then
        slot = SLOT_AXE
    else
        error("harvest: unknown tool: ", tool)
    end
    
    robot.select(slot)
    inv.equip()
    robot.select(1)
    robot.swingDown()
    robot.select(slot)
    inv.equip()
end

function check()
    local blockInfo = geo.analyze(0) --down
    local isAir = blockInfo.name == "minecraft:air"
    local harvested = false
    
    if not blockInfo then
        return
    end
    
    if blockInfo.name:find("botania:") == 1 then
        local tool
        
        if blockInfo.name == "botania:livingwood" then
            tool = "axe"
        elseif blockInfo.name == "botania:livingrock" then
            tool = "pickaxe"
        else
            error("Don't know how to harvest", blockInfo.name)
        end
        
        harvest(tool)
        
        harvested = true
    end
    
    if isAir or harvested then
        return place()
    end
    
    return not isAir
end

function dropoff()
    for slot = 1, SLOT_LAST do
        local stackInfo = inv.getStackInInternalSlot(slot)
        
        if stackInfo and stackInfo.name:find("botania:") == 1 then
            robot.select(slot)
            robot.dropUp()
        end
    end
end

function patrol()
    local steps = {
        robot.forward,
        robot.forward,
        robot.forward,
        robot.forward,
        robot.forward,
        robot.turnLeft,
        robot.forward,
        robot.forward,
        robot.turnLeft,
        robot.forward,
        robot.forward,
        robot.turnLeft,
        robot.forward,
        robot.turnRight,
        robot.forward,
        robot.turnRight,
        robot.forward,
        robot.turnLeft,
        robot.forward,
        robot.forward,
        robot.turnLeft,
        robot.forward,
        dropoff,
        robot.forward,
        robot.turnLeft,
    }
    local airFound = 0
    
    for n, func in ipairs(steps) do
        func()
        
        if func == robot.forward then
            if not check() then
                airFound = airFound + 1
            end
        end
        
        print("found", airFound, "so far")
    end
    
    if airFound >= 16 then
        print("Nothing left to do!")
        
        return true
    end
    
    return false
end

function main()
    while true do
        print("Patroling")
        
        if patrol() then
            break
        end
        
        print("Done patroling, sleeping for", SLEEP_DELAY, "seconds")
        os.sleep(SLEEP_DELAY)
    end
end

main()
