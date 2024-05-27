///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - TOUCH_DONE_INT
    TOUCH_DONE_INT = 0,
    ///1 - TOUCH_INACTIVE_INT
    TOUCH_INACTIVE_INT = 1,
    ///2 - TOUCH_ACTIVE_INT
    TOUCH_ACTIVE_INT = 2,
    ///3 - SARADC1_DONE_INT
    SARADC1_DONE_INT = 3,
    ///4 - SARADC2_DONE_INT
    SARADC2_DONE_INT = 4,
    ///5 - TSENS_DONE_INT
    TSENS_DONE_INT = 5,
    ///6 - RISCV_START_INT
    RISCV_START_INT = 6,
    ///7 - SW_INT
    SW_INT = 7,
    ///8 - SWD_INT
    SWD_INT = 8,
    ///9 - TOUCH_TIME_OUT_INT
    TOUCH_TIME_OUT_INT = 9,
    ///10 - TOUCH_APPROACH_LOOP_DONE_INT
    TOUCH_APPROACH_LOOP_DONE_INT = 10,
    ///11 - TOUCH_SCAN_DONE_INT
    TOUCH_SCAN_DONE_INT = 11,
}
/// TryFromInterruptError
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    /// Attempt to convert a given value into an `Interrupt`
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::TOUCH_DONE_INT),
            1 => Ok(Interrupt::TOUCH_INACTIVE_INT),
            2 => Ok(Interrupt::TOUCH_ACTIVE_INT),
            3 => Ok(Interrupt::SARADC1_DONE_INT),
            4 => Ok(Interrupt::SARADC2_DONE_INT),
            5 => Ok(Interrupt::TSENS_DONE_INT),
            6 => Ok(Interrupt::RISCV_START_INT),
            7 => Ok(Interrupt::SW_INT),
            8 => Ok(Interrupt::SWD_INT),
            9 => Ok(Interrupt::TOUCH_TIME_OUT_INT),
            10 => Ok(Interrupt::TOUCH_APPROACH_LOOP_DONE_INT),
            11 => Ok(Interrupt::TOUCH_SCAN_DONE_INT),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
/// Assigns a handler to an interrupt
///
/// This macro takes two arguments: the name of an interrupt and the path to the
/// function that will be used as the handler of that interrupt. That function
/// must have signature `fn()`.
///
/// Optionally, a third argument may be used to declare interrupt local data.
/// The handler will have exclusive access to these *local* variables on each
/// invocation. If the third argument is used then the signature of the handler
/// function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument
/// passed to the macro.
///
/// # Example
///
/// ``` ignore
/// interrupt!(TIM2, periodic);
///
/// fn periodic() {
///     print!(".");
/// }
///
/// interrupt!(TIM3, tick, locals: {
///     tick: bool = false;
/// });
///
/// fn tick(locals: &mut TIM3::Locals) {
///     locals.tick = !locals.tick;
///
///     if locals.tick {
///         println!("Tick");
///     } else {
///         println!("Tock");
///     }
/// }
/// ```
macro_rules! interrupt {
    ($NAME:ident, $path:path, locals : { $($lvar:ident : $lty:ty = $lval:expr;)* }) => {
        #[allow(non_snake_case)] mod $NAME { pub struct Locals { $(pub $lvar : $lty,)* }
        } #[allow(non_snake_case)] #[no_mangle] pub extern "C" fn $NAME () { let _ =
        $crate ::interrupt::Interrupt:: $NAME; static mut LOCALS : self:: $NAME ::Locals
        = self:: $NAME ::Locals { $($lvar : $lval,)* }; let f : fn (& mut self:: $NAME
        ::Locals) = $path; f(unsafe { & mut LOCALS }); }
    };
    ($NAME:ident, $path:path) => {
        #[allow(non_snake_case)] #[no_mangle] pub extern "C" fn $NAME () { let _ = $crate
        ::interrupt::Interrupt:: $NAME; let f : fn () = $path; f(); }
    };
}
