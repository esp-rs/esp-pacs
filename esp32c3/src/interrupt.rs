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
    ///7 - RWBT
    RWBT = 7,
    ///8 - RWBLE
    RWBLE = 8,
    ///9 - RWBT_NMI
    RWBT_NMI = 9,
    ///10 - RWBLE_NMI
    RWBLE_NMI = 10,
    ///11 - I2C_MASTER
    I2C_MASTER = 11,
    ///12 - SLC0
    SLC0 = 12,
    ///13 - SLC1
    SLC1 = 13,
    ///14 - APB_CTRL
    APB_CTRL = 14,
    ///15 - UHCI0
    UHCI0 = 15,
    ///16 - GPIO
    GPIO = 16,
    ///17 - GPIO_NMI
    GPIO_NMI = 17,
    ///18 - SPI1
    SPI1 = 18,
    ///19 - SPI2
    SPI2 = 19,
    ///20 - I2S0
    I2S0 = 20,
    ///21 - UART0
    UART0 = 21,
    ///22 - UART1
    UART1 = 22,
    ///23 - LEDC
    LEDC = 23,
    ///24 - EFUSE
    EFUSE = 24,
    ///25 - TWAI0
    TWAI0 = 25,
    ///26 - USB_DEVICE
    USB_DEVICE = 26,
    ///27 - RTC_CORE
    RTC_CORE = 27,
    ///28 - RMT
    RMT = 28,
    ///29 - I2C_EXT0
    I2C_EXT0 = 29,
    ///30 - TIMER1
    TIMER1 = 30,
    ///31 - TIMER2
    TIMER2 = 31,
    ///32 - TG0_T0_LEVEL
    TG0_T0_LEVEL = 32,
    ///33 - TG0_WDT_LEVEL
    TG0_WDT_LEVEL = 33,
    ///34 - TG1_T0_LEVEL
    TG1_T0_LEVEL = 34,
    ///35 - TG1_WDT_LEVEL
    TG1_WDT_LEVEL = 35,
    ///36 - CACHE_IA
    CACHE_IA = 36,
    ///37 - SYSTIMER_TARGET0
    SYSTIMER_TARGET0 = 37,
    ///38 - SYSTIMER_TARGET1
    SYSTIMER_TARGET1 = 38,
    ///39 - SYSTIMER_TARGET2
    SYSTIMER_TARGET2 = 39,
    ///40 - SPI_MEM_REJECT_CACHE
    SPI_MEM_REJECT_CACHE = 40,
    ///41 - ICACHE_PRELOAD0
    ICACHE_PRELOAD0 = 41,
    ///42 - ICACHE_SYNC0
    ICACHE_SYNC0 = 42,
    ///43 - APB_ADC
    APB_ADC = 43,
    ///44 - DMA_CH0
    DMA_CH0 = 44,
    ///45 - DMA_CH1
    DMA_CH1 = 45,
    ///46 - DMA_CH2
    DMA_CH2 = 46,
    ///47 - RSA
    RSA = 47,
    ///48 - AES
    AES = 48,
    ///49 - SHA
    SHA = 49,
    ///50 - FROM_CPU_INTR0
    FROM_CPU_INTR0 = 50,
    ///51 - FROM_CPU_INTR1
    FROM_CPU_INTR1 = 51,
    ///52 - FROM_CPU_INTR2
    FROM_CPU_INTR2 = 52,
    ///53 - FROM_CPU_INTR3
    FROM_CPU_INTR3 = 53,
    ///54 - ASSIST_DEBUG
    ASSIST_DEBUG = 54,
    ///55 - DMA_APBPERI_PMS
    DMA_APBPERI_PMS = 55,
    ///56 - CORE0_IRAM0_PMS
    CORE0_IRAM0_PMS = 56,
    ///57 - CORE0_DRAM0_PMS
    CORE0_DRAM0_PMS = 57,
    ///58 - CORE0_PIF_PMS
    CORE0_PIF_PMS = 58,
    ///59 - CORE0_PIF_PMS_SIZE
    CORE0_PIF_PMS_SIZE = 59,
    ///60 - BAK_PMS_VIOLATE
    BAK_PMS_VIOLATE = 60,
    ///61 - CACHE_CORE0_ACS
    CACHE_CORE0_ACS = 61,
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
            7 => Ok(Interrupt::RWBT),
            8 => Ok(Interrupt::RWBLE),
            9 => Ok(Interrupt::RWBT_NMI),
            10 => Ok(Interrupt::RWBLE_NMI),
            11 => Ok(Interrupt::I2C_MASTER),
            12 => Ok(Interrupt::SLC0),
            13 => Ok(Interrupt::SLC1),
            14 => Ok(Interrupt::APB_CTRL),
            15 => Ok(Interrupt::UHCI0),
            16 => Ok(Interrupt::GPIO),
            17 => Ok(Interrupt::GPIO_NMI),
            18 => Ok(Interrupt::SPI1),
            19 => Ok(Interrupt::SPI2),
            20 => Ok(Interrupt::I2S0),
            21 => Ok(Interrupt::UART0),
            22 => Ok(Interrupt::UART1),
            23 => Ok(Interrupt::LEDC),
            24 => Ok(Interrupt::EFUSE),
            25 => Ok(Interrupt::TWAI0),
            26 => Ok(Interrupt::USB_DEVICE),
            27 => Ok(Interrupt::RTC_CORE),
            28 => Ok(Interrupt::RMT),
            29 => Ok(Interrupt::I2C_EXT0),
            30 => Ok(Interrupt::TIMER1),
            31 => Ok(Interrupt::TIMER2),
            32 => Ok(Interrupt::TG0_T0_LEVEL),
            33 => Ok(Interrupt::TG0_WDT_LEVEL),
            34 => Ok(Interrupt::TG1_T0_LEVEL),
            35 => Ok(Interrupt::TG1_WDT_LEVEL),
            36 => Ok(Interrupt::CACHE_IA),
            37 => Ok(Interrupt::SYSTIMER_TARGET0),
            38 => Ok(Interrupt::SYSTIMER_TARGET1),
            39 => Ok(Interrupt::SYSTIMER_TARGET2),
            40 => Ok(Interrupt::SPI_MEM_REJECT_CACHE),
            41 => Ok(Interrupt::ICACHE_PRELOAD0),
            42 => Ok(Interrupt::ICACHE_SYNC0),
            43 => Ok(Interrupt::APB_ADC),
            44 => Ok(Interrupt::DMA_CH0),
            45 => Ok(Interrupt::DMA_CH1),
            46 => Ok(Interrupt::DMA_CH2),
            47 => Ok(Interrupt::RSA),
            48 => Ok(Interrupt::AES),
            49 => Ok(Interrupt::SHA),
            50 => Ok(Interrupt::FROM_CPU_INTR0),
            51 => Ok(Interrupt::FROM_CPU_INTR1),
            52 => Ok(Interrupt::FROM_CPU_INTR2),
            53 => Ok(Interrupt::FROM_CPU_INTR3),
            54 => Ok(Interrupt::ASSIST_DEBUG),
            55 => Ok(Interrupt::DMA_APBPERI_PMS),
            56 => Ok(Interrupt::CORE0_IRAM0_PMS),
            57 => Ok(Interrupt::CORE0_DRAM0_PMS),
            58 => Ok(Interrupt::CORE0_PIF_PMS),
            59 => Ok(Interrupt::CORE0_PIF_PMS_SIZE),
            60 => Ok(Interrupt::BAK_PMS_VIOLATE),
            61 => Ok(Interrupt::CACHE_CORE0_ACS),
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
