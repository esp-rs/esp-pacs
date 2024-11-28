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
    #[doc = "0 - WIFI_MAC"]
    WIFI_MAC = 0,
    #[doc = "1 - WIFI_MAC_NMI"]
    WIFI_MAC_NMI = 1,
    #[doc = "2 - WIFI_PWR"]
    WIFI_PWR = 2,
    #[doc = "3 - WIFI_BB"]
    WIFI_BB = 3,
    #[doc = "4 - BT_MAC"]
    BT_MAC = 4,
    #[doc = "5 - BT_BB"]
    BT_BB = 5,
    #[doc = "6 - BT_BB_NMI"]
    BT_BB_NMI = 6,
    #[doc = "7 - LP_TIMER"]
    LP_TIMER = 7,
    #[doc = "8 - COEX"]
    COEX = 8,
    #[doc = "9 - BLE_TIMER"]
    BLE_TIMER = 9,
    #[doc = "10 - BLE_SEC"]
    BLE_SEC = 10,
    #[doc = "11 - I2C_MASTER"]
    I2C_MASTER = 11,
    #[doc = "12 - ZB_MAC"]
    ZB_MAC = 12,
    #[doc = "13 - PMU"]
    PMU = 13,
    #[doc = "14 - EFUSE"]
    EFUSE = 14,
    #[doc = "15 - LP_RTC_TIMER"]
    LP_RTC_TIMER = 15,
    #[doc = "16 - LP_UART"]
    LP_UART = 16,
    #[doc = "17 - LP_I2C"]
    LP_I2C = 17,
    #[doc = "18 - LP_WDT"]
    LP_WDT = 18,
    #[doc = "19 - LP_PERI_TIMEOUT"]
    LP_PERI_TIMEOUT = 19,
    #[doc = "20 - LP_APM_M0"]
    LP_APM_M0 = 20,
    #[doc = "21 - LP_APM_M1"]
    LP_APM_M1 = 21,
    #[doc = "22 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 22,
    #[doc = "23 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 23,
    #[doc = "24 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 24,
    #[doc = "25 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 25,
    #[doc = "26 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 26,
    #[doc = "27 - TRACE"]
    TRACE = 27,
    #[doc = "28 - CACHE"]
    CACHE = 28,
    #[doc = "29 - CPU_PERI_TIMEOUT"]
    CPU_PERI_TIMEOUT = 29,
    #[doc = "30 - GPIO"]
    GPIO = 30,
    #[doc = "31 - GPIO_NMI"]
    GPIO_NMI = 31,
    #[doc = "32 - PAU"]
    PAU = 32,
    #[doc = "33 - HP_PERI_TIMEOUT"]
    HP_PERI_TIMEOUT = 33,
    #[doc = "34 - MODEM_PERI_TIMEOUT"]
    MODEM_PERI_TIMEOUT = 34,
    #[doc = "35 - HP_APM_M0"]
    HP_APM_M0 = 35,
    #[doc = "36 - HP_APM_M1"]
    HP_APM_M1 = 36,
    #[doc = "37 - HP_APM_M2"]
    HP_APM_M2 = 37,
    #[doc = "38 - HP_APM_M3"]
    HP_APM_M3 = 38,
    #[doc = "39 - LP_APM0"]
    LP_APM0 = 39,
    #[doc = "40 - MSPI"]
    MSPI = 40,
    #[doc = "41 - I2S0"]
    I2S0 = 41,
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
    #[doc = "48 - USB_DEVICE"]
    USB_DEVICE = 48,
    #[doc = "49 - RMT"]
    RMT = 49,
    #[doc = "50 - I2C_EXT0"]
    I2C_EXT0 = 50,
    #[doc = "51 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 51,
    #[doc = "52 - TG0_T1_LEVEL"]
    TG0_T1_LEVEL = 52,
    #[doc = "53 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 53,
    #[doc = "54 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 54,
    #[doc = "55 - TG1_T1_LEVEL"]
    TG1_T1_LEVEL = 55,
    #[doc = "56 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 56,
    #[doc = "57 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 57,
    #[doc = "58 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 58,
    #[doc = "59 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 59,
    #[doc = "60 - APB_SARADC"]
    APB_SARADC = 60,
    #[doc = "61 - MCPWM0"]
    MCPWM0 = 61,
    #[doc = "62 - PCNT"]
    PCNT = 62,
    #[doc = "63 - PARL_IO"]
    PARL_IO = 63,
    #[doc = "64 - SLC0"]
    SLC0 = 64,
    #[doc = "65 - SLC1"]
    SLC1 = 65,
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
