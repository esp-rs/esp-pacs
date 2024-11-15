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
    #[doc = "7 - RWBT"]
    RWBT = 7,
    #[doc = "8 - RWBLE"]
    RWBLE = 8,
    #[doc = "9 - RWBT_NMI"]
    RWBT_NMI = 9,
    #[doc = "10 - RWBLE_NMI"]
    RWBLE_NMI = 10,
    #[doc = "11 - I2C_MASTER"]
    I2C_MASTER = 11,
    #[doc = "12 - SLC0"]
    SLC0 = 12,
    #[doc = "13 - SLC1"]
    SLC1 = 13,
    #[doc = "14 - APB_CTRL"]
    APB_CTRL = 14,
    #[doc = "15 - UHCI0"]
    UHCI0 = 15,
    #[doc = "16 - GPIO"]
    GPIO = 16,
    #[doc = "17 - GPIO_NMI"]
    GPIO_NMI = 17,
    #[doc = "18 - SPI1"]
    SPI1 = 18,
    #[doc = "19 - SPI2"]
    SPI2 = 19,
    #[doc = "20 - I2S0"]
    I2S0 = 20,
    #[doc = "21 - UART0"]
    UART0 = 21,
    #[doc = "22 - UART1"]
    UART1 = 22,
    #[doc = "23 - LEDC"]
    LEDC = 23,
    #[doc = "24 - EFUSE"]
    EFUSE = 24,
    #[doc = "25 - TWAI0"]
    TWAI0 = 25,
    #[doc = "26 - USB_DEVICE"]
    USB_DEVICE = 26,
    #[doc = "27 - RTC_CORE"]
    RTC_CORE = 27,
    #[doc = "28 - RMT"]
    RMT = 28,
    #[doc = "29 - I2C_EXT0"]
    I2C_EXT0 = 29,
    #[doc = "30 - TIMER1"]
    TIMER1 = 30,
    #[doc = "31 - TIMER2"]
    TIMER2 = 31,
    #[doc = "32 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 32,
    #[doc = "33 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 33,
    #[doc = "34 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 34,
    #[doc = "35 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 35,
    #[doc = "36 - CACHE_IA"]
    CACHE_IA = 36,
    #[doc = "37 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 37,
    #[doc = "38 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 38,
    #[doc = "39 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 39,
    #[doc = "40 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 40,
    #[doc = "41 - ICACHE_PRELOAD0"]
    ICACHE_PRELOAD0 = 41,
    #[doc = "42 - ICACHE_SYNC0"]
    ICACHE_SYNC0 = 42,
    #[doc = "43 - APB_ADC"]
    APB_ADC = 43,
    #[doc = "44 - DMA_CH0"]
    DMA_CH0 = 44,
    #[doc = "45 - DMA_CH1"]
    DMA_CH1 = 45,
    #[doc = "46 - DMA_CH2"]
    DMA_CH2 = 46,
    #[doc = "47 - RSA"]
    RSA = 47,
    #[doc = "48 - AES"]
    AES = 48,
    #[doc = "49 - SHA"]
    SHA = 49,
    #[doc = "50 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 50,
    #[doc = "51 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 51,
    #[doc = "52 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 52,
    #[doc = "53 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 53,
    #[doc = "54 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 54,
    #[doc = "55 - DMA_APBPERI_PMS"]
    DMA_APBPERI_PMS = 55,
    #[doc = "56 - CORE0_IRAM0_PMS"]
    CORE0_IRAM0_PMS = 56,
    #[doc = "57 - CORE0_DRAM0_PMS"]
    CORE0_DRAM0_PMS = 57,
    #[doc = "58 - CORE0_PIF_PMS"]
    CORE0_PIF_PMS = 58,
    #[doc = "59 - CORE0_PIF_PMS_SIZE"]
    CORE0_PIF_PMS_SIZE = 59,
    #[doc = "60 - BAK_PMS_VIOLATE"]
    BAK_PMS_VIOLATE = 60,
    #[doc = "61 - CACHE_CORE0_ACS"]
    CACHE_CORE0_ACS = 61,
}
