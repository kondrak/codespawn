-- include config file
dofile "examples/config.lua"

code = {
    func_ptr = { "var", "extern", "void*", "" },
    some_number = { "var", "", "int", "1" },
    ignored_var = { "var", "", "", "" },
}

function get_code()
    local code_str = ""
    for k, v in pairs(code) do
        code_str = code_str..k
    end
    
    return code_str
end