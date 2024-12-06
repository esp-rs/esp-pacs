#[doc = r" Core interrupts. These interrupts are handled by the core itself."]
# [riscv :: pac_enum (unsafe CoreInterruptNumber)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreInterrupt {
    #[doc = "0 - User Software Interrupt"]
    UserSoft = 0,
    #[doc = "1 - Supervisor Software Interrupt"]
    SupervisorSoft = 1,
    #[doc = "3 - Machine Software Interrupt"]
    MachineSoft = 3,
    #[doc = "4 - User Timer Interrupt"]
    UserTimer = 4,
    #[doc = "5 - Supervisor Timer Interrupt"]
    SupervisorTimer = 5,
    #[doc = "7 - Machine Timer Interrupt"]
    MachineTimer = 7,
    #[doc = "8 - User External Interrupt"]
    UserExternal = 8,
    #[doc = "9 - Supervisor External Interrupt"]
    SupervisorExternal = 9,
    #[doc = "11 - Machine External Interrupt"]
    MachineExternal = 11,
}
pub use riscv::interrupt::Exception;
#[doc = r" Priority levels in the device"]
# [riscv :: pac_enum (unsafe PriorityNumber)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    #[doc = "1 - Priority level 1"]
    P1 = 1,
    #[doc = "2 - Priority level 2"]
    P2 = 2,
    #[doc = "3 - Priority level 3"]
    P3 = 3,
    #[doc = "4 - Priority level 4"]
    P4 = 4,
    #[doc = "5 - Priority level 5"]
    P5 = 5,
    #[doc = "6 - Priority level 6"]
    P6 = 6,
    #[doc = "7 - Priority level 7"]
    P7 = 7,
    #[doc = "8 - Priority level 8"]
    P8 = 8,
    #[doc = "9 - Priority level 9"]
    P9 = 9,
    #[doc = "10 - Priority level 10"]
    P10 = 10,
    #[doc = "11 - Priority level 11"]
    P11 = 11,
    #[doc = "12 - Priority level 12"]
    P12 = 12,
    #[doc = "13 - Priority level 13"]
    P13 = 13,
    #[doc = "14 - Priority level 14"]
    P14 = 14,
    #[doc = "15 - Priority level 15"]
    P15 = 15,
}
#[doc = r" HARTs in the device"]
# [riscv :: pac_enum (unsafe HartIdNumber)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hart {
    #[doc = "0 - Hart 0"]
    H0 = 0,
}
pub use riscv::{
    interrupt::{disable, enable, free, nested},
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "0 - PMU"]
    PMU = 0,
    #[doc = "1 - EFUSE"]
    EFUSE = 1,
    #[doc = "2 - LP_RTC_TIMER"]
    LP_RTC_TIMER = 2,
    #[doc = "3 - LP_BLE_TIMER"]
    LP_BLE_TIMER = 3,
    #[doc = "4 - LP_WDT"]
    LP_WDT = 4,
    #[doc = "5 - LP_PERI_TIMEOUT"]
    LP_PERI_TIMEOUT = 5,
    #[doc = "6 - LP_APM_M0"]
    LP_APM_M0 = 6,
    #[doc = "7 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 7,
    #[doc = "8 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 8,
    #[doc = "9 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 9,
    #[doc = "10 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 10,
    #[doc = "11 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 11,
    #[doc = "12 - TRACE"]
    TRACE = 12,
    #[doc = "13 - CACHE"]
    CACHE = 13,
    #[doc = "14 - CPU_PERI_TIMEOUT"]
    CPU_PERI_TIMEOUT = 14,
    #[doc = "15 - BT_MAC"]
    BT_MAC = 15,
    #[doc = "16 - BT_BB"]
    BT_BB = 16,
    #[doc = "17 - BT_BB_NMI"]
    BT_BB_NMI = 17,
    #[doc = "18 - COEX"]
    COEX = 18,
    #[doc = "19 - BLE_TIMER"]
    BLE_TIMER = 19,
    #[doc = "20 - BLE_SEC"]
    BLE_SEC = 20,
    #[doc = "21 - ZB_MAC"]
    ZB_MAC = 21,
    #[doc = "22 - GPIO"]
    GPIO = 22,
    #[doc = "23 - GPIO_NMI"]
    GPIO_NMI = 23,
    #[doc = "24 - PAU"]
    PAU = 24,
    #[doc = "25 - HP_PERI_TIMEOUT"]
    HP_PERI_TIMEOUT = 25,
    #[doc = "26 - HP_APM_M0"]
    HP_APM_M0 = 26,
    #[doc = "27 - HP_APM_M1"]
    HP_APM_M1 = 27,
    #[doc = "28 - HP_APM_M2"]
    HP_APM_M2 = 28,
    #[doc = "29 - HP_APM_M3"]
    HP_APM_M3 = 29,
    #[doc = "30 - MSPI"]
    MSPI = 30,
    #[doc = "31 - I2S0"]
    I2S0 = 31,
    #[doc = "32 - UHCI0"]
    UHCI0 = 32,
    #[doc = "33 - UART0"]
    UART0 = 33,
    #[doc = "34 - UART1"]
    UART1 = 34,
    #[doc = "35 - LEDC"]
    LEDC = 35,
    #[doc = "36 - TWAI0"]
    TWAI0 = 36,
    #[doc = "37 - USB_DEVICE"]
    USB_DEVICE = 37,
    #[doc = "38 - RMT"]
    RMT = 38,
    #[doc = "39 - I2C_EXT0"]
    I2C_EXT0 = 39,
    #[doc = "40 - I2C_EXT1"]
    I2C_EXT1 = 40,
    #[doc = "41 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 41,
    #[doc = "42 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 42,
    #[doc = "43 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 43,
    #[doc = "44 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 44,
    #[doc = "45 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 45,
    #[doc = "46 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 46,
    #[doc = "47 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 47,
    #[doc = "48 - APB_ADC"]
    APB_ADC = 48,
    #[doc = "49 - MCPWM0"]
    MCPWM0 = 49,
    #[doc = "50 - PCNT"]
    PCNT = 50,
    #[doc = "51 - PARL_IO_TX"]
    PARL_IO_TX = 51,
    #[doc = "52 - PARL_IO_RX"]
    PARL_IO_RX = 52,
    #[doc = "53 - DMA_IN_CH0"]
    DMA_IN_CH0 = 53,
    #[doc = "54 - DMA_IN_CH1"]
    DMA_IN_CH1 = 54,
    #[doc = "55 - DMA_IN_CH2"]
    DMA_IN_CH2 = 55,
    #[doc = "56 - DMA_OUT_CH0"]
    DMA_OUT_CH0 = 56,
    #[doc = "57 - DMA_OUT_CH1"]
    DMA_OUT_CH1 = 57,
    #[doc = "58 - DMA_OUT_CH2"]
    DMA_OUT_CH2 = 58,
    #[doc = "59 - SPI2"]
    SPI2 = 59,
    #[doc = "60 - AES"]
    AES = 60,
    #[doc = "61 - SHA"]
    SHA = 61,
    #[doc = "62 - RSA"]
    RSA = 62,
    #[doc = "63 - ECC"]
    ECC = 63,
    #[doc = "64 - ECDSA"]
    ECDSA = 64,
}
