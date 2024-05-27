///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///1 - LP_WDT
    LP_WDT = 1,
    ///2 - LP_TIMER0
    LP_TIMER0 = 2,
    ///3 - LP_TIMER1
    LP_TIMER1 = 3,
    ///6 - PMU0
    PMU0 = 6,
    ///7 - PMU1
    PMU1 = 7,
    ///8 - LP_ANA
    LP_ANA = 8,
    ///9 - LP_ADC
    LP_ADC = 9,
    ///10 - LP_GPIO
    LP_GPIO = 10,
    ///11 - LP_I2C0
    LP_I2C0 = 11,
    ///12 - LP_I2S0
    LP_I2S0 = 12,
    ///14 - LP_TOUCH
    LP_TOUCH = 14,
    ///15 - LP_TSENS
    LP_TSENS = 15,
    ///16 - LP_UART
    LP_UART = 16,
    ///19 - LP_SYS
    LP_SYS = 19,
    ///20 - LP_HUK
    LP_HUK = 20,
    ///22 - USB_DEVICE
    USB_DEVICE = 22,
    ///24 - DMA
    DMA = 24,
    ///25 - SPI2
    SPI2 = 25,
    ///26 - SPI3
    SPI3 = 26,
    ///27 - I2S0
    I2S0 = 27,
    ///28 - I2S1
    I2S1 = 28,
    ///29 - I2S2
    I2S2 = 29,
    ///30 - UHCI0
    UHCI0 = 30,
    ///31 - UART0
    UART0 = 31,
    ///32 - UART1
    UART1 = 32,
    ///33 - UART2
    UART2 = 33,
    ///34 - UART3
    UART3 = 34,
    ///35 - UART4
    UART4 = 35,
    ///38 - PWM0
    PWM0 = 38,
    ///39 - PWM1
    PWM1 = 39,
    ///40 - TWAI0
    TWAI0 = 40,
    ///41 - TWAI1
    TWAI1 = 41,
    ///42 - TWAI2
    TWAI2 = 42,
    ///43 - RMT
    RMT = 43,
    ///44 - I2C0
    I2C0 = 44,
    ///45 - I2C1
    I2C1 = 45,
    ///46 - TG0_T0
    TG0_T0 = 46,
    ///47 - TG0_T1
    TG0_T1 = 47,
    ///48 - TG0_WDT
    TG0_WDT = 48,
    ///49 - TG1_T0
    TG1_T0 = 49,
    ///50 - TG1_T1
    TG1_T1 = 50,
    ///51 - TG1_WDT
    TG1_WDT = 51,
    ///52 - LEDC
    LEDC = 52,
    ///53 - SYSTIMER_TARGET0
    SYSTIMER_TARGET0 = 53,
    ///54 - SYSTIMER_TARGET1
    SYSTIMER_TARGET1 = 54,
    ///55 - SYSTIMER_TARGET2
    SYSTIMER_TARGET2 = 55,
    ///56 - AHB_PDMA_IN_CH0
    AHB_PDMA_IN_CH0 = 56,
    ///57 - AHB_PDMA_IN_CH1
    AHB_PDMA_IN_CH1 = 57,
    ///58 - AHB_PDMA_IN_CH2
    AHB_PDMA_IN_CH2 = 58,
    ///59 - AHB_PDMA_OUT_CH0
    AHB_PDMA_OUT_CH0 = 59,
    ///60 - AHB_PDMA_OUT_CH1
    AHB_PDMA_OUT_CH1 = 60,
    ///61 - AHB_PDMA_OUT_CH2
    AHB_PDMA_OUT_CH2 = 61,
    ///62 - AXI_PDMA_IN_CH0
    AXI_PDMA_IN_CH0 = 62,
    ///63 - AXI_PDMA_IN_CH1
    AXI_PDMA_IN_CH1 = 63,
    ///64 - AXI_PDMA_IN_CH2
    AXI_PDMA_IN_CH2 = 64,
    ///65 - AXI_PDMA_OUT_CH0
    AXI_PDMA_OUT_CH0 = 65,
    ///66 - AXI_PDMA_OUT_CH1
    AXI_PDMA_OUT_CH1 = 66,
    ///67 - AXI_PDMA_OUT_CH2
    AXI_PDMA_OUT_CH2 = 67,
    ///68 - RSA
    RSA = 68,
    ///69 - AES
    AES = 69,
    ///70 - SHA
    SHA = 70,
    ///71 - ECC
    ECC = 71,
    ///74 - GPIO
    GPIO = 74,
    ///75 - GPIO_INT1
    GPIO_INT1 = 75,
    ///76 - GPIO_INT2
    GPIO_INT2 = 76,
    ///77 - GPIO_INT3
    GPIO_INT3 = 77,
    ///78 - GPIO_PAD_COMP
    GPIO_PAD_COMP = 78,
    ///83 - CACHE
    CACHE = 83,
    ///85 - CSI_BRIDGE
    CSI_BRIDGE = 85,
    ///86 - DSI_BRIDGE
    DSI_BRIDGE = 86,
    ///87 - CSI
    CSI = 87,
    ///88 - DSI
    DSI = 88,
    ///95 - JPEG
    JPEG = 95,
    ///96 - PPA
    PPA = 96,
    ///100 - ISP
    ISP = 100,
    ///101 - I3C
    I3C = 101,
    ///102 - I3C_SLV
    I3C_SLV = 102,
    ///110 - HP_SYS
    HP_SYS = 110,
    ///111 - PCNT
    PCNT = 111,
    ///112 - PAU
    PAU = 112,
    ///113 - PARLIO_RX
    PARLIO_RX = 113,
    ///114 - PARLIO_TX
    PARLIO_TX = 114,
    ///115 - H264_DMA2D_OUT_CH0
    H264_DMA2D_OUT_CH0 = 115,
    ///116 - H264_DMA2D_OUT_CH1
    H264_DMA2D_OUT_CH1 = 116,
    ///117 - H264_DMA2D_OUT_CH2
    H264_DMA2D_OUT_CH2 = 117,
    ///118 - H264_DMA2D_OUT_CH3
    H264_DMA2D_OUT_CH3 = 118,
    ///119 - H264_DMA2D_OUT_CH4
    H264_DMA2D_OUT_CH4 = 119,
    ///120 - H264_DMA2D_IN_CH0
    H264_DMA2D_IN_CH0 = 120,
    ///121 - H264_DMA2D_IN_CH1
    H264_DMA2D_IN_CH1 = 121,
    ///122 - H264_DMA2D_IN_CH2
    H264_DMA2D_IN_CH2 = 122,
    ///123 - H264_DMA2D_IN_CH3
    H264_DMA2D_IN_CH3 = 123,
    ///124 - H264_DMA2D_IN_CH4
    H264_DMA2D_IN_CH4 = 124,
    ///125 - H264_DMA2D_IN_CH5
    H264_DMA2D_IN_CH5 = 125,
    ///126 - H264_REG
    H264_REG = 126,
    ///127 - ASSIST_DEBUG
    ASSIST_DEBUG = 127,
}
/// TryFromInterruptError
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    /// Attempt to convert a given value into an `Interrupt`
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::LP_WDT),
            2 => Ok(Interrupt::LP_TIMER0),
            3 => Ok(Interrupt::LP_TIMER1),
            6 => Ok(Interrupt::PMU0),
            7 => Ok(Interrupt::PMU1),
            8 => Ok(Interrupt::LP_ANA),
            9 => Ok(Interrupt::LP_ADC),
            10 => Ok(Interrupt::LP_GPIO),
            11 => Ok(Interrupt::LP_I2C0),
            12 => Ok(Interrupt::LP_I2S0),
            14 => Ok(Interrupt::LP_TOUCH),
            15 => Ok(Interrupt::LP_TSENS),
            16 => Ok(Interrupt::LP_UART),
            19 => Ok(Interrupt::LP_SYS),
            20 => Ok(Interrupt::LP_HUK),
            22 => Ok(Interrupt::USB_DEVICE),
            24 => Ok(Interrupt::DMA),
            25 => Ok(Interrupt::SPI2),
            26 => Ok(Interrupt::SPI3),
            27 => Ok(Interrupt::I2S0),
            28 => Ok(Interrupt::I2S1),
            29 => Ok(Interrupt::I2S2),
            30 => Ok(Interrupt::UHCI0),
            31 => Ok(Interrupt::UART0),
            32 => Ok(Interrupt::UART1),
            33 => Ok(Interrupt::UART2),
            34 => Ok(Interrupt::UART3),
            35 => Ok(Interrupt::UART4),
            38 => Ok(Interrupt::PWM0),
            39 => Ok(Interrupt::PWM1),
            40 => Ok(Interrupt::TWAI0),
            41 => Ok(Interrupt::TWAI1),
            42 => Ok(Interrupt::TWAI2),
            43 => Ok(Interrupt::RMT),
            44 => Ok(Interrupt::I2C0),
            45 => Ok(Interrupt::I2C1),
            46 => Ok(Interrupt::TG0_T0),
            47 => Ok(Interrupt::TG0_T1),
            48 => Ok(Interrupt::TG0_WDT),
            49 => Ok(Interrupt::TG1_T0),
            50 => Ok(Interrupt::TG1_T1),
            51 => Ok(Interrupt::TG1_WDT),
            52 => Ok(Interrupt::LEDC),
            53 => Ok(Interrupt::SYSTIMER_TARGET0),
            54 => Ok(Interrupt::SYSTIMER_TARGET1),
            55 => Ok(Interrupt::SYSTIMER_TARGET2),
            56 => Ok(Interrupt::AHB_PDMA_IN_CH0),
            57 => Ok(Interrupt::AHB_PDMA_IN_CH1),
            58 => Ok(Interrupt::AHB_PDMA_IN_CH2),
            59 => Ok(Interrupt::AHB_PDMA_OUT_CH0),
            60 => Ok(Interrupt::AHB_PDMA_OUT_CH1),
            61 => Ok(Interrupt::AHB_PDMA_OUT_CH2),
            62 => Ok(Interrupt::AXI_PDMA_IN_CH0),
            63 => Ok(Interrupt::AXI_PDMA_IN_CH1),
            64 => Ok(Interrupt::AXI_PDMA_IN_CH2),
            65 => Ok(Interrupt::AXI_PDMA_OUT_CH0),
            66 => Ok(Interrupt::AXI_PDMA_OUT_CH1),
            67 => Ok(Interrupt::AXI_PDMA_OUT_CH2),
            68 => Ok(Interrupt::RSA),
            69 => Ok(Interrupt::AES),
            70 => Ok(Interrupt::SHA),
            71 => Ok(Interrupt::ECC),
            74 => Ok(Interrupt::GPIO),
            75 => Ok(Interrupt::GPIO_INT1),
            76 => Ok(Interrupt::GPIO_INT2),
            77 => Ok(Interrupt::GPIO_INT3),
            78 => Ok(Interrupt::GPIO_PAD_COMP),
            83 => Ok(Interrupt::CACHE),
            85 => Ok(Interrupt::CSI_BRIDGE),
            86 => Ok(Interrupt::DSI_BRIDGE),
            87 => Ok(Interrupt::CSI),
            88 => Ok(Interrupt::DSI),
            95 => Ok(Interrupt::JPEG),
            96 => Ok(Interrupt::PPA),
            100 => Ok(Interrupt::ISP),
            101 => Ok(Interrupt::I3C),
            102 => Ok(Interrupt::I3C_SLV),
            110 => Ok(Interrupt::HP_SYS),
            111 => Ok(Interrupt::PCNT),
            112 => Ok(Interrupt::PAU),
            113 => Ok(Interrupt::PARLIO_RX),
            114 => Ok(Interrupt::PARLIO_TX),
            115 => Ok(Interrupt::H264_DMA2D_OUT_CH0),
            116 => Ok(Interrupt::H264_DMA2D_OUT_CH1),
            117 => Ok(Interrupt::H264_DMA2D_OUT_CH2),
            118 => Ok(Interrupt::H264_DMA2D_OUT_CH3),
            119 => Ok(Interrupt::H264_DMA2D_OUT_CH4),
            120 => Ok(Interrupt::H264_DMA2D_IN_CH0),
            121 => Ok(Interrupt::H264_DMA2D_IN_CH1),
            122 => Ok(Interrupt::H264_DMA2D_IN_CH2),
            123 => Ok(Interrupt::H264_DMA2D_IN_CH3),
            124 => Ok(Interrupt::H264_DMA2D_IN_CH4),
            125 => Ok(Interrupt::H264_DMA2D_IN_CH5),
            126 => Ok(Interrupt::H264_REG),
            127 => Ok(Interrupt::ASSIST_DEBUG),
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
