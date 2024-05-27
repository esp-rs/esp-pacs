///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///7 - LP_TIMER
    LP_TIMER = 7,
    ///16 - LP_UART
    LP_UART = 16,
    ///17 - LP_I2C
    LP_I2C = 17,
    ///18 - LP_WDT
    LP_WDT = 18,
    ///19 - LP_PERI_TIMEOUT
    LP_PERI_TIMEOUT = 19,
    ///20 - LP_APM_M0
    LP_APM_M0 = 20,
    ///21 - LP_APM_M1
    LP_APM_M1 = 21,
}
/// TryFromInterruptError
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    /// Attempt to convert a given value into an `Interrupt`
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            7 => Ok(Interrupt::LP_TIMER),
            16 => Ok(Interrupt::LP_UART),
            17 => Ok(Interrupt::LP_I2C),
            18 => Ok(Interrupt::LP_WDT),
            19 => Ok(Interrupt::LP_PERI_TIMEOUT),
            20 => Ok(Interrupt::LP_APM_M0),
            21 => Ok(Interrupt::LP_APM_M1),
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
