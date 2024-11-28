local fs = {}

---@class Path
---@field private segments string[]
---@field private segments_readonly_ string[]
---# tsuki.fs.Path
---A class to represent file paths and allow
---ergonomic path traversal.
fs.Path = {}

---@return Path new_path The newly created path from given string.
---@param from string The string to parse a path from.
---```lua
---local fs = require('fs')
---local Path = fs.Path
---local home_path = Path.new("/home/arch/projects/tsuki")
---```
function fs.Path.new(from)
    local new_path = setmetatable({
        -- set all default constructor properties here
        segments = FS_INTERNAL.split_path(from),
    }, {
        __index = fs.Path,          -- Inherit methods from Path
        __newindex = function() end -- Disable adding new behaviour
    })
    return new_path
end

---@return boolean exists Whether or not there is a directory or file at this path.
---Example:
---```lua
---local path = Path.new("/path/to/file")
---print(path:exists())
---```
function fs.Path:exists()
    return FS_INTERNAL.raw_path_exists("/" .. table.concat(self.segments, "/"))
end

---@return string[] segments A table containing the segments of this path.
---Example:
---```lua
---local path = Path.new("/home/arch/luafile")
---for _, v in pairs(path:get_segments()) do
---    print(v)  -- Should print (in order):
---end           -- "home", "arch", "luafile"
---```
function fs.Path:get_segments()
    return self.segments
end

---Special iterator function to traverse a `Path`
---class as a string without needing to allocate a
---new string.
---
---Does not allocate a new string, and instead feeds
---the contents of its internal segments array as if
---it were a continuous string.
function fs.Path:iter()
    local p1 = 1
    local p2 = 0
    local d = true
    return function()
        if d then
            d = false
            return '/'
        end
        p2 = p2 + 1
        if p2 > #self.segments[p1] then
            p1 = p1 + 1
            p2 = 0
            d = true
        end
        if p1 <= #self.segments and #self.segments[p1] > 0 then
            return self.segments[p1]:sub(p2, p2)
        end
    end
end

return fs
