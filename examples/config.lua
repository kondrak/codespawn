rust = {
    num_tabs = 1,
    tab_char = '\t',

    -- type redefinitions
    type = { int   = "c_int",
             float = "c_float",
             void  = "c_void",
             char  = "c_char"
           },
    -- name redefinitions
    name = { GenericStruct = "RustStruct",
             OPTION        = "Option<",
             OPTION_FPTR   = "Option<extern "
           }
}

-- internal type and name arrays can be also set post init:
rust.type["int*"]   = "*mut c_int"
rust.type["float*"] = "*mut c_float"
rust.type["void*"]  = "*mut c_void"
rust.type["const float*"] = "*const c_float"
rust.type["const char*"]  = "*const c_char"
rust.name["DERIVE-DBG"] = "#[derive(Debug)]"
rust.name["C-ABI"]      = "#[repr(C)]"
rust.name["..."] = ""

cpp = {
    name = { OPTION = "",
             OPTION_FPTR = ""
           }
}