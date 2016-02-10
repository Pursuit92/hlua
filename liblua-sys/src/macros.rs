#![allow(non_snake_case)]

/// Not actually macros. These are (some of) the things that bindgen
/// doesn't get. They were all originally C preprocessor macros.

use super::all::*;

/*
** basic types
*/
pub const LUA_TNONE: i32 = (-1);

pub const LUA_TNIL: i32 = 0;
pub const LUA_TBOOLEAN: i32 = 1;
pub const LUA_TLIGHTUSERDATA: i32 = 2;
pub const LUA_TNUMBER: i32 = 3;
pub const LUA_TSTRING: i32 = 4;
pub const LUA_TTABLE: i32 = 5;
pub const LUA_TFUNCTION: i32 = 6;
pub const LUA_TUSERDATA: i32 = 7;
pub const LUA_TTHREAD: i32 = 8;

pub const LUA_NUMTAGS: i32 = 9;

pub const LUA_OK: i32 = 0;
pub const LUA_YIELD: i32 = 1;
pub const LUA_ERRRUN: i32 = 2;
pub const LUA_ERRSYNTAX: i32 = 3;
pub const LUA_ERRMEM: i32 = 4;
pub const LUA_ERRGCMM: i32 = 5;
pub const LUA_ERRERR: i32 = 6;

// #define lua_getextraspace(L)	((void *)((char *)(L) - LUA_EXTRASPACE))

#[inline(always)]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: i32) -> lua_Number {
    lua_tonumberx(L, i, 0 as *mut i32)
}

#[inline(always)]
pub unsafe fn lua_tointeger(L: *mut lua_State, i: i32) -> lua_Integer {
    lua_tointegerx(L, i, 0 as *mut i32)
}

#[inline(always)]
pub unsafe fn lua_pop(L: *mut lua_State, n: i32) {
    lua_settop(L, -n-1)
}

#[inline(always)]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

#[inline(always)]
pub unsafe fn lua_register(L: *mut lua_State, n: i32, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n as *const ::std::os::raw::c_char);
}

#[inline(always)]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

#[inline(always)]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TFUNCTION
}

#[inline(always)]
pub unsafe fn lua_istable(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TTABLE
}

// #define lua_islightuserdata(L,n)	(lua_type(L, (n)) == LUA_TLIGHTUSERDATA)

#[inline(always)]
pub unsafe fn lua_isnil(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TNIL
}

#[inline(always)]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TBOOLEAN
}

// #define lua_isthread(L,n)	(lua_type(L, (n)) == LUA_TTHREAD)
// #define lua_isnone(L,n)		(lua_type(L, (n)) == LUA_TNONE)
// #define lua_isnoneornil(L, n)	(lua_type(L, (n)) <= 0)

// #define lua_pushliteral(L, s)	lua_pushstring(L, "" s)

// #define lua_pushglobaltable(L)  \
// 	lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS)

#[inline(always)]
pub unsafe fn lua_tostring(L: *mut lua_State, i: i32) -> *const ::std::os::raw::c_char {
    lua_tolstring(L, i, 0 as *mut u64)
}


#[inline(always)]
pub unsafe fn lua_insert(L: *mut lua_State, idx: i32) {
    lua_rotate(L, idx, 1)
}

// #define lua_remove(L,idx)	(lua_rotate(L, (idx), -1), lua_pop(L, 1))

// #define lua_replace(L,idx)	(lua_copy(L, -1, (idx)), lua_pop(L, 1))

// #define lua_pushunsigned(L,n)	lua_pushinteger(L, (lua_Integer)(n))
#[inline(always)]
pub unsafe fn lua_pushunsigned(L: *mut lua_State, n: u64) {
    lua_pushinteger(L, n as i64)
}

#[inline(always)]
pub unsafe fn lua_tounsignedx(L: *mut lua_State, i: i32, is: *mut i32) -> lua_Unsigned {
    lua_tointegerx(L, i, is) as lua_Unsigned
}

// #define lua_tounsigned(L,i)	lua_tounsignedx(L,(i),NULL)

#[inline(always)]
pub unsafe fn lua_call(L: *mut lua_State, nargs: ::std::os::raw::c_int,
                 nresults: ::std::os::raw::c_int) {
    lua_callk(L, nargs, nresults, 0, None)
}

#[inline(always)]
pub unsafe fn lua_pcall(L: *mut lua_State, nargs: ::std::os::raw::c_int,
                 nresults: ::std::os::raw::c_int,
                 errfunc: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    lua_pcallk(L, nargs, nresults, errfunc, 0, None)
}


pub const LUAI_MAXSTACK: i32 = 1000000;
pub const LUA_REGISTRYINDEX: i32 = (-LUAI_MAXSTACK - 1000);

#[inline(always)]
pub unsafe fn lua_upvalueindex(i: i32) -> i32 {
    LUA_REGISTRYINDEX - i
}
