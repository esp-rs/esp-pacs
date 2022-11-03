#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "13 - PMU"]
    PMU = 13,
    #[doc = "14 - EFUSE"]
    EFUSE = 14,
    #[doc = "26 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 26,
    #[doc = "27 - TRACE"]
    TRACE = 27,
    #[doc = "30 - GPIO_PRO"]
    GPIO_PRO = 30,
    #[doc = "31 - GPIO_PRO_NMI"]
    GPIO_PRO_NMI = 31,
    #[doc = "32 - PAU"]
    PAU = 32,
    #[doc = "41 - I2S1"]
    I2S1 = 41,
    #[doc = "42 - UHCI0"]
    UHCI0 = 42,
    #[doc = "43 - UART0"]
    UART0 = 43,
    #[doc = "44 - UART1"]
    UART1 = 44,
    #[doc = "45 - LEDC"]
    LEDC = 45,
    #[doc = "46 - TWAI0"]
    TWAI0 = 46,
    #[doc = "47 - TWAI1"]
    TWAI1 = 47,
    #[doc = "48 - USB"]
    USB = 48,
    #[doc = "49 - RMT"]
    RMT = 49,
    #[doc = "50 - I2C_EXT0"]
    I2C_EXT0 = 50,
    #[doc = "51 - TG0_T0"]
    TG0_T0 = 51,
    #[doc = "52 - TG0_T1"]
    TG0_T1 = 52,
    #[doc = "53 - TG0_WDT"]
    TG0_WDT = 53,
    #[doc = "54 - TG1_T0"]
    TG1_T0 = 54,
    #[doc = "55 - TG1_T1"]
    TG1_T1 = 55,
    #[doc = "56 - TG1_WDT"]
    TG1_WDT = 56,
    #[doc = "57 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 57,
    #[doc = "58 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 58,
    #[doc = "59 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 59,
    #[doc = "60 - APB_SARADC"]
    APB_SARADC = 60,
    #[doc = "62 - PCNT"]
    PCNT = 62,
    #[doc = "63 - PARL_IO"]
    PARL_IO = 63,
    #[doc = "66 - DMA_IN_CH0"]
    DMA_IN_CH0 = 66,
    #[doc = "67 - DMA_IN_CH1"]
    DMA_IN_CH1 = 67,
    #[doc = "68 - DMA_IN_CH2"]
    DMA_IN_CH2 = 68,
    #[doc = "69 - DMA_OUT_CH0"]
    DMA_OUT_CH0 = 69,
    #[doc = "70 - DMA_OUT_CH1"]
    DMA_OUT_CH1 = 70,
    #[doc = "71 - DMA_OUT_CH2"]
    DMA_OUT_CH2 = 71,
    #[doc = "72 - SPI2"]
    SPI2 = 72,
    #[doc = "73 - AES"]
    AES = 73,
    #[doc = "74 - SHA"]
    SHA = 74,
    #[doc = "75 - RSA"]
    RSA = 75,
    #[doc = "76 - ECC"]
    ECC = 76,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            13 => Ok(Interrupt::PMU),
            14 => Ok(Interrupt::EFUSE),
            26 => Ok(Interrupt::ASSIST_DEBUG),
            27 => Ok(Interrupt::TRACE),
            30 => Ok(Interrupt::GPIO_PRO),
            31 => Ok(Interrupt::GPIO_PRO_NMI),
            32 => Ok(Interrupt::PAU),
            41 => Ok(Interrupt::I2S1),
            42 => Ok(Interrupt::UHCI0),
            43 => Ok(Interrupt::UART0),
            44 => Ok(Interrupt::UART1),
            45 => Ok(Interrupt::LEDC),
            46 => Ok(Interrupt::TWAI0),
            47 => Ok(Interrupt::TWAI1),
            48 => Ok(Interrupt::USB),
            49 => Ok(Interrupt::RMT),
            50 => Ok(Interrupt::I2C_EXT0),
            51 => Ok(Interrupt::TG0_T0),
            52 => Ok(Interrupt::TG0_T1),
            53 => Ok(Interrupt::TG0_WDT),
            54 => Ok(Interrupt::TG1_T0),
            55 => Ok(Interrupt::TG1_T1),
            56 => Ok(Interrupt::TG1_WDT),
            57 => Ok(Interrupt::SYSTIMER_TARGET0),
            58 => Ok(Interrupt::SYSTIMER_TARGET1),
            59 => Ok(Interrupt::SYSTIMER_TARGET2),
            60 => Ok(Interrupt::APB_SARADC),
            62 => Ok(Interrupt::PCNT),
            63 => Ok(Interrupt::PARL_IO),
            66 => Ok(Interrupt::DMA_IN_CH0),
            67 => Ok(Interrupt::DMA_IN_CH1),
            68 => Ok(Interrupt::DMA_IN_CH2),
            69 => Ok(Interrupt::DMA_OUT_CH0),
            70 => Ok(Interrupt::DMA_OUT_CH1),
            71 => Ok(Interrupt::DMA_OUT_CH2),
            72 => Ok(Interrupt::SPI2),
            73 => Ok(Interrupt::AES),
            74 => Ok(Interrupt::SHA),
            75 => Ok(Interrupt::RSA),
            76 => Ok(Interrupt::ECC),
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
