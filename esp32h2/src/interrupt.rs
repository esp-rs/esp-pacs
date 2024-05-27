///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - PMU
    PMU = 0,
    ///1 - EFUSE
    EFUSE = 1,
    ///2 - LP_RTC_TIMER
    LP_RTC_TIMER = 2,
    ///3 - LP_BLE_TIMER
    LP_BLE_TIMER = 3,
    ///4 - LP_WDT
    LP_WDT = 4,
    ///5 - LP_PERI_TIMEOUT
    LP_PERI_TIMEOUT = 5,
    ///6 - LP_APM_M0
    LP_APM_M0 = 6,
    ///7 - FROM_CPU_INTR0
    FROM_CPU_INTR0 = 7,
    ///8 - FROM_CPU_INTR1
    FROM_CPU_INTR1 = 8,
    ///9 - FROM_CPU_INTR2
    FROM_CPU_INTR2 = 9,
    ///10 - FROM_CPU_INTR3
    FROM_CPU_INTR3 = 10,
    ///11 - ASSIST_DEBUG
    ASSIST_DEBUG = 11,
    ///12 - TRACE
    TRACE = 12,
    ///13 - CACHE
    CACHE = 13,
    ///14 - CPU_PERI_TIMEOUT
    CPU_PERI_TIMEOUT = 14,
    ///15 - BT_MAC
    BT_MAC = 15,
    ///16 - BT_BB
    BT_BB = 16,
    ///17 - BT_BB_NMI
    BT_BB_NMI = 17,
    ///18 - COEX
    COEX = 18,
    ///19 - BLE_TIMER
    BLE_TIMER = 19,
    ///20 - BLE_SEC
    BLE_SEC = 20,
    ///21 - ZB_MAC
    ZB_MAC = 21,
    ///22 - GPIO
    GPIO = 22,
    ///23 - GPIO_NMI
    GPIO_NMI = 23,
    ///24 - PAU
    PAU = 24,
    ///25 - HP_PERI_TIMEOUT
    HP_PERI_TIMEOUT = 25,
    ///26 - HP_APM_M0
    HP_APM_M0 = 26,
    ///27 - HP_APM_M1
    HP_APM_M1 = 27,
    ///28 - HP_APM_M2
    HP_APM_M2 = 28,
    ///29 - HP_APM_M3
    HP_APM_M3 = 29,
    ///30 - MSPI
    MSPI = 30,
    ///31 - I2S0
    I2S0 = 31,
    ///32 - UHCI0
    UHCI0 = 32,
    ///33 - UART0
    UART0 = 33,
    ///34 - UART1
    UART1 = 34,
    ///35 - LEDC
    LEDC = 35,
    ///36 - TWAI0
    TWAI0 = 36,
    ///37 - USB_DEVICE
    USB_DEVICE = 37,
    ///38 - RMT
    RMT = 38,
    ///39 - I2C_EXT0
    I2C_EXT0 = 39,
    ///40 - I2C_EXT1
    I2C_EXT1 = 40,
    ///41 - TG0_T0_LEVEL
    TG0_T0_LEVEL = 41,
    ///42 - TG0_WDT_LEVEL
    TG0_WDT_LEVEL = 42,
    ///43 - TG1_T0_LEVEL
    TG1_T0_LEVEL = 43,
    ///44 - TG1_WDT_LEVEL
    TG1_WDT_LEVEL = 44,
    ///45 - SYSTIMER_TARGET0
    SYSTIMER_TARGET0 = 45,
    ///46 - SYSTIMER_TARGET1
    SYSTIMER_TARGET1 = 46,
    ///47 - SYSTIMER_TARGET2
    SYSTIMER_TARGET2 = 47,
    ///48 - APB_ADC
    APB_ADC = 48,
    ///49 - MCPWM0
    MCPWM0 = 49,
    ///50 - PCNT
    PCNT = 50,
    ///51 - PARL_IO_TX
    PARL_IO_TX = 51,
    ///52 - PARL_IO_RX
    PARL_IO_RX = 52,
    ///53 - DMA_IN_CH0
    DMA_IN_CH0 = 53,
    ///54 - DMA_IN_CH1
    DMA_IN_CH1 = 54,
    ///55 - DMA_IN_CH2
    DMA_IN_CH2 = 55,
    ///56 - DMA_OUT_CH0
    DMA_OUT_CH0 = 56,
    ///57 - DMA_OUT_CH1
    DMA_OUT_CH1 = 57,
    ///58 - DMA_OUT_CH2
    DMA_OUT_CH2 = 58,
    ///59 - SPI2
    SPI2 = 59,
    ///60 - AES
    AES = 60,
    ///61 - SHA
    SHA = 61,
    ///62 - RSA
    RSA = 62,
    ///63 - ECC
    ECC = 63,
    ///64 - ECDSA
    ECDSA = 64,
}
/// TryFromInterruptError
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    /// Attempt to convert a given value into an `Interrupt`
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::PMU),
            1 => Ok(Interrupt::EFUSE),
            2 => Ok(Interrupt::LP_RTC_TIMER),
            3 => Ok(Interrupt::LP_BLE_TIMER),
            4 => Ok(Interrupt::LP_WDT),
            5 => Ok(Interrupt::LP_PERI_TIMEOUT),
            6 => Ok(Interrupt::LP_APM_M0),
            7 => Ok(Interrupt::FROM_CPU_INTR0),
            8 => Ok(Interrupt::FROM_CPU_INTR1),
            9 => Ok(Interrupt::FROM_CPU_INTR2),
            10 => Ok(Interrupt::FROM_CPU_INTR3),
            11 => Ok(Interrupt::ASSIST_DEBUG),
            12 => Ok(Interrupt::TRACE),
            13 => Ok(Interrupt::CACHE),
            14 => Ok(Interrupt::CPU_PERI_TIMEOUT),
            15 => Ok(Interrupt::BT_MAC),
            16 => Ok(Interrupt::BT_BB),
            17 => Ok(Interrupt::BT_BB_NMI),
            18 => Ok(Interrupt::COEX),
            19 => Ok(Interrupt::BLE_TIMER),
            20 => Ok(Interrupt::BLE_SEC),
            21 => Ok(Interrupt::ZB_MAC),
            22 => Ok(Interrupt::GPIO),
            23 => Ok(Interrupt::GPIO_NMI),
            24 => Ok(Interrupt::PAU),
            25 => Ok(Interrupt::HP_PERI_TIMEOUT),
            26 => Ok(Interrupt::HP_APM_M0),
            27 => Ok(Interrupt::HP_APM_M1),
            28 => Ok(Interrupt::HP_APM_M2),
            29 => Ok(Interrupt::HP_APM_M3),
            30 => Ok(Interrupt::MSPI),
            31 => Ok(Interrupt::I2S0),
            32 => Ok(Interrupt::UHCI0),
            33 => Ok(Interrupt::UART0),
            34 => Ok(Interrupt::UART1),
            35 => Ok(Interrupt::LEDC),
            36 => Ok(Interrupt::TWAI0),
            37 => Ok(Interrupt::USB_DEVICE),
            38 => Ok(Interrupt::RMT),
            39 => Ok(Interrupt::I2C_EXT0),
            40 => Ok(Interrupt::I2C_EXT1),
            41 => Ok(Interrupt::TG0_T0_LEVEL),
            42 => Ok(Interrupt::TG0_WDT_LEVEL),
            43 => Ok(Interrupt::TG1_T0_LEVEL),
            44 => Ok(Interrupt::TG1_WDT_LEVEL),
            45 => Ok(Interrupt::SYSTIMER_TARGET0),
            46 => Ok(Interrupt::SYSTIMER_TARGET1),
            47 => Ok(Interrupt::SYSTIMER_TARGET2),
            48 => Ok(Interrupt::APB_ADC),
            49 => Ok(Interrupt::MCPWM0),
            50 => Ok(Interrupt::PCNT),
            51 => Ok(Interrupt::PARL_IO_TX),
            52 => Ok(Interrupt::PARL_IO_RX),
            53 => Ok(Interrupt::DMA_IN_CH0),
            54 => Ok(Interrupt::DMA_IN_CH1),
            55 => Ok(Interrupt::DMA_IN_CH2),
            56 => Ok(Interrupt::DMA_OUT_CH0),
            57 => Ok(Interrupt::DMA_OUT_CH1),
            58 => Ok(Interrupt::DMA_OUT_CH2),
            59 => Ok(Interrupt::SPI2),
            60 => Ok(Interrupt::AES),
            61 => Ok(Interrupt::SHA),
            62 => Ok(Interrupt::RSA),
            63 => Ok(Interrupt::ECC),
            64 => Ok(Interrupt::ECDSA),
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
