local fs = {}

---@class File
---@field name string
fs.File = {}

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
    local split = FS_INTERNAL.split_path(from)
    local new_path = setmetatable({
        segments = split,
        segments_readonly_ = setmetatable({}, {
            __index = split,
            __newindex = function()
                error("Attempt to modify read-only `Path.segments`")
            end
        })
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
    -- return FS_INTERNAL.raw_path_exists(self.raw)
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
    return self.segments_readonly_
end

return fs
