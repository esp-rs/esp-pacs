#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "11 - I2C_MST"]
    I2C_MST = 11,
    #[doc = "13 - GPIO_INTR_PRO"]
    GPIO_INTR_PRO = 13,
    #[doc = "14 - GPIO_INTR_PRO_NMI"]
    GPIO_INTR_PRO_NMI = 14,
    #[doc = "15 - SPI_INTR_1"]
    SPI_INTR_1 = 15,
    #[doc = "16 - SPI_INTR_2"]
    SPI_INTR_2 = 16,
    #[doc = "17 - UART0"]
    UART0 = 17,
    #[doc = "18 - UART1"]
    UART1 = 18,
    #[doc = "19 - LEDC"]
    LEDC = 19,
    #[doc = "20 - EFUSE"]
    EFUSE = 20,
    #[doc = "21 - RTC_CORE"]
    RTC_CORE = 21,
    #[doc = "22 - I2C_EXT0"]
    I2C_EXT0 = 22,
    #[doc = "23 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 23,
    #[doc = "24 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 24,
    #[doc = "26 - SYSTIMER_TARGET0_EDGE"]
    SYSTIMER_TARGET0_EDGE = 26,
    #[doc = "27 - SYSTIMER_TARGET1_EDGE"]
    SYSTIMER_TARGET1_EDGE = 27,
    #[doc = "28 - SYSTIMER_TARGET2_EDGE"]
    SYSTIMER_TARGET2_EDGE = 28,
    #[doc = "29 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 29,
    #[doc = "32 - APB_ADC"]
    APB_ADC = 32,
    #[doc = "33 - DMA_CH0"]
    DMA_CH0 = 33,
    #[doc = "34 - SHA"]
    SHA = 34,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            11 => Ok(Interrupt::I2C_MST),
            13 => Ok(Interrupt::GPIO_INTR_PRO),
            14 => Ok(Interrupt::GPIO_INTR_PRO_NMI),
            15 => Ok(Interrupt::SPI_INTR_1),
            16 => Ok(Interrupt::SPI_INTR_2),
            17 => Ok(Interrupt::UART0),
            18 => Ok(Interrupt::UART1),
            19 => Ok(Interrupt::LEDC),
            20 => Ok(Interrupt::EFUSE),
            21 => Ok(Interrupt::RTC_CORE),
            22 => Ok(Interrupt::I2C_EXT0),
            23 => Ok(Interrupt::TG0_T0_LEVEL),
            24 => Ok(Interrupt::TG0_WDT_LEVEL),
            26 => Ok(Interrupt::SYSTIMER_TARGET0_EDGE),
            27 => Ok(Interrupt::SYSTIMER_TARGET1_EDGE),
            28 => Ok(Interrupt::SYSTIMER_TARGET2_EDGE),
            29 => Ok(Interrupt::SPI_MEM_REJECT_CACHE),
            32 => Ok(Interrupt::APB_ADC),
            33 => Ok(Interrupt::DMA_CH0),
            34 => Ok(Interrupt::SHA),
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
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
