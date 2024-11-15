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
    #[doc = "11 - I2C_MST"]
    I2C_MST = 11,
    #[doc = "12 - APB_CTRL"]
    APB_CTRL = 12,
    #[doc = "13 - GPIO"]
    GPIO = 13,
    #[doc = "14 - GPIO_NMI"]
    GPIO_NMI = 14,
    #[doc = "15 - SPI1"]
    SPI1 = 15,
    #[doc = "16 - SPI2"]
    SPI2 = 16,
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
    #[doc = "25 - CACHE_IA"]
    CACHE_IA = 25,
    #[doc = "26 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 26,
    #[doc = "27 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 27,
    #[doc = "28 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 28,
    #[doc = "29 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 29,
    #[doc = "30 - ICACHE_PRELOAD0"]
    ICACHE_PRELOAD0 = 30,
    #[doc = "31 - ICACHE_SYNC0"]
    ICACHE_SYNC0 = 31,
    #[doc = "32 - APB_ADC"]
    APB_ADC = 32,
    #[doc = "33 - DMA_CH0"]
    DMA_CH0 = 33,
    #[doc = "34 - SHA"]
    SHA = 34,
    #[doc = "35 - ECC"]
    ECC = 35,
    #[doc = "36 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 36,
    #[doc = "37 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 37,
    #[doc = "38 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 38,
    #[doc = "39 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 39,
    #[doc = "40 - Assist debug interrupt"]
    ASSIST_DEBUG = 40,
    #[doc = "41 - ETS_CORE0_PIF_PMS_SIZE"]
    ETS_CORE0_PIF_PMS_SIZE = 41,
}
