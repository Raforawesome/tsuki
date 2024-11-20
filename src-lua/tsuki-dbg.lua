---@meta

local tsuki_dbg = {}

---@return nil
function tsuki_dbg.print_hello_from_rust()
    TSUKI_DBG_INTERNAL.print_hello()
end

return tsuki_dbg
