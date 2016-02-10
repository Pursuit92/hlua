use ffi;

use std::ffi::CString;
use std::io::Cursor;
use std::io::Read;
use std::io::Error as IoError;
use std::mem;
use std::ptr;

use AsMutLua;

use LuaRead;
use LuaError;
use PushGuard;

///
pub struct LuaFunction<L> {
    variable: L
}

struct ReadData<R> {
    reader: R,
    buffer: [u8 ; 128],
    triggered_error: Option<IoError>,
}

unsafe extern fn reader<R>(_: *mut ffi::lua_State, data_raw: *mut ::std::os::raw::c_void, size: *mut ffi::size_t)
                    -> *const ::std::os::raw::c_char
                    where R: Read
{
    let data: &mut ReadData<R> = mem::transmute(data_raw);

    if data.triggered_error.is_some() {
        (*size) = 0;
        return data.buffer.as_ptr() as *const ::std::os::raw::c_char;
    }

    match data.reader.read(&mut data.buffer) {
        Ok(len) =>
            (*size) = len as ffi::size_t,
        Err(e) => {
            (*size) = 0;
            data.triggered_error = Some(e)
        },
    };

    data.buffer.as_ptr() as *const ::std::os::raw::c_char
}

impl<L> LuaFunction<L> where L: AsMutLua {
    /// Calls the `LuaFunction`.
    pub fn call<'a, V>(&'a mut self) -> Result<V, LuaError>
        where V: LuaRead<PushGuard<&'a mut L>>
    {
        // calling pcall pops the parameters and pushes output
        let (pcall_return_value, pushed_value) = unsafe {
            // lua_pcall pops the function, so we have to make a copy of it
            ffi::lua_pushvalue(self.variable.as_mut_lua().0, -1);
            let pcall_return_value = ffi::lua_pcall(self.variable.as_mut_lua().0, 0, 1, 0);     // TODO: arguments
            (pcall_return_value, PushGuard { lua: &mut self.variable, size: 1 })
        };

        // if pcall succeeded, returning
        if pcall_return_value == 0 {
            return match LuaRead::lua_read(pushed_value) {
                Err(_) => Err(LuaError::WrongType),
                Ok(x) => Ok(x)
            };
        }

        // an error occured during execution
        if pcall_return_value == ffi::LUA_ERRMEM {
            panic!("lua_pcall returned LUA_ERRMEM");
        }

        if pcall_return_value == ffi::LUA_ERRRUN {
            let error_msg: String = LuaRead::lua_read(pushed_value).ok().expect("can't find error \
                                                                                 message at the top of \
                                                                                 the Lua stack");
            return Err(LuaError::ExecutionError(error_msg));
        }

        panic!("Unknown error code returned by lua_pcall: {}", pcall_return_value)
    }

    /// Builds a new `LuaFunction` from the code of a reader.
    pub fn load_from_reader<R>(mut lua: L, code: R) -> Result<LuaFunction<PushGuard<L>>, LuaError>
                               where R: Read
    {
        let readdata = ReadData {
            reader: code,
            buffer: unsafe { ::std::mem::uninitialized() },
            triggered_error: None,
        };

        let (load_return_value, pushed_value) = unsafe {
            let chunk_name = CString::new("chunk").unwrap();
            let code = ffi::lua_load(lua.as_mut_lua().0, Some(reader::<R>), mem::transmute(&readdata),
                                     chunk_name.as_ptr(), ptr::null());
            (code, PushGuard { lua: lua, size: 1 })
        };

        if readdata.triggered_error.is_some() {
            let error = readdata.triggered_error.unwrap();
            return Err(LuaError::ReadError(error));
        }

        if load_return_value == 0 {
            return Ok(LuaFunction{
                variable: pushed_value,
            });
        }

        let error_msg: String = LuaRead::lua_read(pushed_value).ok().expect("can't find error message \
                                                                             at the top of the Lua \
                                                                             stack");

        if load_return_value == ffi::LUA_ERRMEM {
            panic!("LUA_ERRMEM");
        }

        if load_return_value == ffi::LUA_ERRSYNTAX {
            return Err(LuaError::SyntaxError(error_msg));
        }

        panic!("Unknown error while calling lua_load");
    }

    /// Builds a new `LuaFunction` from a raw string.
    pub fn load(lua: L, code: &str) -> Result<LuaFunction<PushGuard<L>>, LuaError> {
        let code: Vec<_> = code.bytes().collect();
        let reader = Cursor::new(code);
        LuaFunction::load_from_reader(lua, reader)
    }
}

// TODO: return Result<Ret, ExecutionError> instead
/*impl<'a, 'lua, Ret: CopyRead> ::std::ops::FnMut<(), Ret> for LuaFunction<'a,'lua> {
    fn call_mut(&mut self, _: ()) -> Ret {
        self.call().unwrap()
    }
}*/

impl<L> LuaRead<L> for LuaFunction<L> where L: AsMutLua {
    fn lua_read_at_position(mut lua: L, index: i32) -> Result<LuaFunction<L>, L> {
        assert!(index == -1);   // FIXME:
        if unsafe { ffi::lua_iscfunction(lua.as_mut_lua().0, -1) } != 0 {
            Ok(LuaFunction { variable: lua })
        } else {
            Err(lua)
        }
    }
}
