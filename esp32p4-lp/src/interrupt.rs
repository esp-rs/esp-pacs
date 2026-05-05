#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "3 - LP software interrupt"]
    LP_SW       = 3,
    #[doc = "7 - LP UART interrupt"]
    LP_UART     = 7,
    #[doc = "11 - LP SPI interrupt"]
    LP_SPI      = 11,
    #[doc = "15 - LP TRNG interrupt"]
    LP_TRNG     = 15,
    #[doc = "16 - LP I2C interrupt"]
    LP_I2C      = 16,
    #[doc = "17 - LP GPIO/IO MUX interrupt"]
    LP_IO       = 17,
    #[doc = "18 - LP ADC interrupt"]
    LP_ADC      = 18,
    #[doc = "19 - LP Touch sensor interrupt"]
    LP_TOUCH    = 19,
    #[doc = "20 - LP temperature sensor interrupt"]
    LP_TSENS    = 20,
    #[doc = "21 - LP eFuse / HUK interrupt"]
    LP_EFUSE    = 21,
    #[doc = "22 - LP system register interrupt"]
    LP_SYSREG   = 22,
    #[doc = "23 - LP analog peripheral interrupt"]
    LP_ANA_PERI = 23,
    #[doc = "24 - LP PMU interrupt"]
    LP_PMU      = 24,
    #[doc = "25 - LP mailbox interrupt"]
    LP_MAILBOX  = 25,
    #[doc = "26 - LP timer interrupt"]
    LP_TIMER    = 26,
    #[doc = "27 - LP watchdog interrupt"]
    LP_WDT      = 27,
    #[doc = "28 - LP RTC interrupt"]
    LP_RTC      = 28,
}
#[doc = r" TryFromInterruptError"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            3 => Ok(Interrupt::LP_SW),
            7 => Ok(Interrupt::LP_UART),
            11 => Ok(Interrupt::LP_SPI),
            15 => Ok(Interrupt::LP_TRNG),
            16 => Ok(Interrupt::LP_I2C),
            17 => Ok(Interrupt::LP_IO),
            18 => Ok(Interrupt::LP_ADC),
            19 => Ok(Interrupt::LP_TOUCH),
            20 => Ok(Interrupt::LP_TSENS),
            21 => Ok(Interrupt::LP_EFUSE),
            22 => Ok(Interrupt::LP_SYSREG),
            23 => Ok(Interrupt::LP_ANA_PERI),
            24 => Ok(Interrupt::LP_PMU),
            25 => Ok(Interrupt::LP_MAILBOX),
            26 => Ok(Interrupt::LP_TIMER),
            27 => Ok(Interrupt::LP_WDT),
            28 => Ok(Interrupt::LP_RTC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
