///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - WIFI_MAC
    WIFI_MAC = 0,
    ///1 - WIFI_MAC_NMI
    WIFI_MAC_NMI = 1,
    ///2 - WIFI_PWR
    WIFI_PWR = 2,
    ///3 - WIFI_BB
    WIFI_BB = 3,
    ///4 - BT_MAC
    BT_MAC = 4,
    ///5 - BT_BB
    BT_BB = 5,
    ///6 - BT_BB_NMI
    BT_BB_NMI = 6,
    ///7 - LP_TIMER
    LP_TIMER = 7,
    ///8 - COEX
    COEX = 8,
    ///9 - BLE_TIMER
    BLE_TIMER = 9,
    ///10 - BLE_SEC
    BLE_SEC = 10,
    ///11 - I2C_MASTER
    I2C_MASTER = 11,
    ///12 - ZB_MAC
    ZB_MAC = 12,
    ///13 - PMU
    PMU = 13,
    ///14 - EFUSE
    EFUSE = 14,
    ///15 - LP_RTC_TIMER
    LP_RTC_TIMER = 15,
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
    ///22 - FROM_CPU_INTR0
    FROM_CPU_INTR0 = 22,
    ///23 - FROM_CPU_INTR1
    FROM_CPU_INTR1 = 23,
    ///24 - FROM_CPU_INTR2
    FROM_CPU_INTR2 = 24,
    ///25 - FROM_CPU_INTR3
    FROM_CPU_INTR3 = 25,
    ///26 - ASSIST_DEBUG
    ASSIST_DEBUG = 26,
    ///27 - TRACE
    TRACE = 27,
    ///28 - CACHE
    CACHE = 28,
    ///29 - CPU_PERI_TIMEOUT
    CPU_PERI_TIMEOUT = 29,
    ///30 - GPIO
    GPIO = 30,
    ///31 - GPIO_NMI
    GPIO_NMI = 31,
    ///32 - PAU
    PAU = 32,
    ///33 - HP_PERI_TIMEOUT
    HP_PERI_TIMEOUT = 33,
    ///34 - MODEM_PERI_TIMEOUT
    MODEM_PERI_TIMEOUT = 34,
    ///35 - HP_APM_M0
    HP_APM_M0 = 35,
    ///36 - HP_APM_M1
    HP_APM_M1 = 36,
    ///37 - HP_APM_M2
    HP_APM_M2 = 37,
    ///38 - HP_APM_M3
    HP_APM_M3 = 38,
    ///39 - LP_APM0
    LP_APM0 = 39,
    ///40 - MSPI
    MSPI = 40,
    ///41 - I2S0
    I2S0 = 41,
    ///42 - UHCI0
    UHCI0 = 42,
    ///43 - UART0
    UART0 = 43,
    ///44 - UART1
    UART1 = 44,
    ///45 - LEDC
    LEDC = 45,
    ///46 - TWAI0
    TWAI0 = 46,
    ///47 - TWAI1
    TWAI1 = 47,
    ///48 - USB_DEVICE
    USB_DEVICE = 48,
    ///49 - RMT
    RMT = 49,
    ///50 - I2C_EXT0
    I2C_EXT0 = 50,
    ///51 - TG0_T0_LEVEL
    TG0_T0_LEVEL = 51,
    ///52 - TG0_T1_LEVEL
    TG0_T1_LEVEL = 52,
    ///53 - TG0_WDT_LEVEL
    TG0_WDT_LEVEL = 53,
    ///54 - TG1_T0_LEVEL
    TG1_T0_LEVEL = 54,
    ///55 - TG1_T1_LEVEL
    TG1_T1_LEVEL = 55,
    ///56 - TG1_WDT_LEVEL
    TG1_WDT_LEVEL = 56,
    ///57 - SYSTIMER_TARGET0
    SYSTIMER_TARGET0 = 57,
    ///58 - SYSTIMER_TARGET1
    SYSTIMER_TARGET1 = 58,
    ///59 - SYSTIMER_TARGET2
    SYSTIMER_TARGET2 = 59,
    ///60 - APB_SARADC
    APB_SARADC = 60,
    ///61 - MCPWM0
    MCPWM0 = 61,
    ///62 - PCNT
    PCNT = 62,
    ///63 - PARL_IO
    PARL_IO = 63,
    ///64 - SLC0
    SLC0 = 64,
    ///65 - SLC1
    SLC1 = 65,
    ///66 - DMA_IN_CH0
    DMA_IN_CH0 = 66,
    ///67 - DMA_IN_CH1
    DMA_IN_CH1 = 67,
    ///68 - DMA_IN_CH2
    DMA_IN_CH2 = 68,
    ///69 - DMA_OUT_CH0
    DMA_OUT_CH0 = 69,
    ///70 - DMA_OUT_CH1
    DMA_OUT_CH1 = 70,
    ///71 - DMA_OUT_CH2
    DMA_OUT_CH2 = 71,
    ///72 - SPI2
    SPI2 = 72,
    ///73 - AES
    AES = 73,
    ///74 - SHA
    SHA = 74,
    ///75 - RSA
    RSA = 75,
    ///76 - ECC
    ECC = 76,
}
/// TryFromInterruptError
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    /// Attempt to convert a given value into an `Interrupt`
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::WIFI_MAC),
            1 => Ok(Interrupt::WIFI_MAC_NMI),
            2 => Ok(Interrupt::WIFI_PWR),
            3 => Ok(Interrupt::WIFI_BB),
            4 => Ok(Interrupt::BT_MAC),
            5 => Ok(Interrupt::BT_BB),
            6 => Ok(Interrupt::BT_BB_NMI),
            7 => Ok(Interrupt::LP_TIMER),
            8 => Ok(Interrupt::COEX),
            9 => Ok(Interrupt::BLE_TIMER),
            10 => Ok(Interrupt::BLE_SEC),
            11 => Ok(Interrupt::I2C_MASTER),
            12 => Ok(Interrupt::ZB_MAC),
            13 => Ok(Interrupt::PMU),
            14 => Ok(Interrupt::EFUSE),
            15 => Ok(Interrupt::LP_RTC_TIMER),
            16 => Ok(Interrupt::LP_UART),
            17 => Ok(Interrupt::LP_I2C),
            18 => Ok(Interrupt::LP_WDT),
            19 => Ok(Interrupt::LP_PERI_TIMEOUT),
            20 => Ok(Interrupt::LP_APM_M0),
            21 => Ok(Interrupt::LP_APM_M1),
            22 => Ok(Interrupt::FROM_CPU_INTR0),
            23 => Ok(Interrupt::FROM_CPU_INTR1),
            24 => Ok(Interrupt::FROM_CPU_INTR2),
            25 => Ok(Interrupt::FROM_CPU_INTR3),
            26 => Ok(Interrupt::ASSIST_DEBUG),
            27 => Ok(Interrupt::TRACE),
            28 => Ok(Interrupt::CACHE),
            29 => Ok(Interrupt::CPU_PERI_TIMEOUT),
            30 => Ok(Interrupt::GPIO),
            31 => Ok(Interrupt::GPIO_NMI),
            32 => Ok(Interrupt::PAU),
            33 => Ok(Interrupt::HP_PERI_TIMEOUT),
            34 => Ok(Interrupt::MODEM_PERI_TIMEOUT),
            35 => Ok(Interrupt::HP_APM_M0),
            36 => Ok(Interrupt::HP_APM_M1),
            37 => Ok(Interrupt::HP_APM_M2),
            38 => Ok(Interrupt::HP_APM_M3),
            39 => Ok(Interrupt::LP_APM0),
            40 => Ok(Interrupt::MSPI),
            41 => Ok(Interrupt::I2S0),
            42 => Ok(Interrupt::UHCI0),
            43 => Ok(Interrupt::UART0),
            44 => Ok(Interrupt::UART1),
            45 => Ok(Interrupt::LEDC),
            46 => Ok(Interrupt::TWAI0),
            47 => Ok(Interrupt::TWAI1),
            48 => Ok(Interrupt::USB_DEVICE),
            49 => Ok(Interrupt::RMT),
            50 => Ok(Interrupt::I2C_EXT0),
            51 => Ok(Interrupt::TG0_T0_LEVEL),
            52 => Ok(Interrupt::TG0_T1_LEVEL),
            53 => Ok(Interrupt::TG0_WDT_LEVEL),
            54 => Ok(Interrupt::TG1_T0_LEVEL),
            55 => Ok(Interrupt::TG1_T1_LEVEL),
            56 => Ok(Interrupt::TG1_WDT_LEVEL),
            57 => Ok(Interrupt::SYSTIMER_TARGET0),
            58 => Ok(Interrupt::SYSTIMER_TARGET1),
            59 => Ok(Interrupt::SYSTIMER_TARGET2),
            60 => Ok(Interrupt::APB_SARADC),
            61 => Ok(Interrupt::MCPWM0),
            62 => Ok(Interrupt::PCNT),
            63 => Ok(Interrupt::PARL_IO),
            64 => Ok(Interrupt::SLC0),
            65 => Ok(Interrupt::SLC1),
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
