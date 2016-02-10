extern crate pkg_config;
extern crate gcc;

fn main() {
    match pkg_config::find_library("lua5.2") {
        Ok(_) => return,
        Err(..) => {}
    };

    gcc::Config::new()
        .file("lua-5.3.2/src/lapi.c")
        .file("lua-5.3.2/src/lauxlib.c")
        .file("lua-5.3.2/src/lbaselib.c")
        .file("lua-5.3.2/src/lbitlib.c")
        .file("lua-5.3.2/src/lcode.c")
        .file("lua-5.3.2/src/lcorolib.c")
        .file("lua-5.3.2/src/lctype.c")
        .file("lua-5.3.2/src/ldblib.c")
        .file("lua-5.3.2/src/ldebug.c")
        .file("lua-5.3.2/src/ldo.c")
        .file("lua-5.3.2/src/ldump.c")
        .file("lua-5.3.2/src/lfunc.c")
        .file("lua-5.3.2/src/lgc.c")
        .file("lua-5.3.2/src/linit.c")
        .file("lua-5.3.2/src/liolib.c")
        .file("lua-5.3.2/src/llex.c")
        .file("lua-5.3.2/src/lmathlib.c")
        .file("lua-5.3.2/src/lmem.c")
        .file("lua-5.3.2/src/loadlib.c")
        .file("lua-5.3.2/src/lobject.c")
        .file("lua-5.3.2/src/lopcodes.c")
        .file("lua-5.3.2/src/loslib.c")
        .file("lua-5.3.2/src/lparser.c")
        .file("lua-5.3.2/src/lstate.c")
        .file("lua-5.3.2/src/lstring.c")
        .file("lua-5.3.2/src/lstrlib.c")
        .file("lua-5.3.2/src/ltable.c")
        .file("lua-5.3.2/src/ltablib.c")
        .file("lua-5.3.2/src/ltm.c")
        .file("lua-5.3.2/src/lua.c")
        .file("lua-5.3.2/src/luac.c")
        .file("lua-5.3.2/src/lundump.c")
        .file("lua-5.3.2/src/lutf8lib.c")
        .file("lua-5.3.2/src/lvm.c")
        .file("lua-5.3.2/src/lzio.c")
        .define("LUA_COMPAT_ALL", None)
        .include("lua/src")
        .compile("liblua.a");
}
