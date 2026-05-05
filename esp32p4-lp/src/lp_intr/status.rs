#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `HUK_INTR_ST` reader - need_des"]
pub type HUK_INTR_ST_R = crate::BitReader;
#[doc = "Field `SYSREG_INTR_ST` reader - need_des"]
pub type SYSREG_INTR_ST_R = crate::BitReader;
#[doc = "Field `SW_INTR_ST` reader - need_des"]
pub type SW_INTR_ST_R = crate::BitReader;
#[doc = "Field `EFUSE_INTR_ST` reader - need_des"]
pub type EFUSE_INTR_ST_R = crate::BitReader;
#[doc = "Field `UART_INTR_ST` reader - need_des"]
pub type UART_INTR_ST_R = crate::BitReader;
#[doc = "Field `TSENS_INTR_ST` reader - need_des"]
pub type TSENS_INTR_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_INTR_ST` reader - need_des"]
pub type TOUCH_INTR_ST_R = crate::BitReader;
#[doc = "Field `SPI_INTR_ST` reader - need_des"]
pub type SPI_INTR_ST_R = crate::BitReader;
#[doc = "Field `I2S_INTR_ST` reader - need_des"]
pub type I2S_INTR_ST_R = crate::BitReader;
#[doc = "Field `I2C_INTR_ST` reader - need_des"]
pub type I2C_INTR_ST_R = crate::BitReader;
#[doc = "Field `GPIO_INTR_ST` reader - need_des"]
pub type GPIO_INTR_ST_R = crate::BitReader;
#[doc = "Field `ADC_INTR_ST` reader - need_des"]
pub type ADC_INTR_ST_R = crate::BitReader;
#[doc = "Field `ANAPERI_INTR_ST` reader - need_des"]
pub type ANAPERI_INTR_ST_R = crate::BitReader;
#[doc = "Field `PMU_REG_1_INTR_ST` reader - need_des"]
pub type PMU_REG_1_INTR_ST_R = crate::BitReader;
#[doc = "Field `PMU_REG_0_INTR_ST` reader - need_des"]
pub type PMU_REG_0_INTR_ST_R = crate::BitReader;
#[doc = "Field `MB_LP_INTR_ST` reader - need_des"]
pub type MB_LP_INTR_ST_R = crate::BitReader;
#[doc = "Field `MB_HP_INTR_ST` reader - need_des"]
pub type MB_HP_INTR_ST_R = crate::BitReader;
#[doc = "Field `TIMER_REG_1_INTR_ST` reader - need_des"]
pub type TIMER_REG_1_INTR_ST_R = crate::BitReader;
#[doc = "Field `TIMER_REG_0_INTR_ST` reader - need_des"]
pub type TIMER_REG_0_INTR_ST_R = crate::BitReader;
#[doc = "Field `WDT_INTR_ST` reader - need_des"]
pub type WDT_INTR_ST_R = crate::BitReader;
#[doc = "Field `RTC_INTR_ST` reader - need_des"]
pub type RTC_INTR_ST_R = crate::BitReader;
#[doc = "Field `HP_INTR_ST` reader - need_des"]
pub type HP_INTR_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn huk_intr_st(&self) -> HUK_INTR_ST_R {
        HUK_INTR_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn sysreg_intr_st(&self) -> SYSREG_INTR_ST_R {
        SYSREG_INTR_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn sw_intr_st(&self) -> SW_INTR_ST_R {
        SW_INTR_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn efuse_intr_st(&self) -> EFUSE_INTR_ST_R {
        EFUSE_INTR_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn uart_intr_st(&self) -> UART_INTR_ST_R {
        UART_INTR_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn tsens_intr_st(&self) -> TSENS_INTR_ST_R {
        TSENS_INTR_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_intr_st(&self) -> TOUCH_INTR_ST_R {
        TOUCH_INTR_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn spi_intr_st(&self) -> SPI_INTR_ST_R {
        SPI_INTR_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn i2s_intr_st(&self) -> I2S_INTR_ST_R {
        I2S_INTR_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn i2c_intr_st(&self) -> I2C_INTR_ST_R {
        I2C_INTR_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn gpio_intr_st(&self) -> GPIO_INTR_ST_R {
        GPIO_INTR_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn adc_intr_st(&self) -> ADC_INTR_ST_R {
        ADC_INTR_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn anaperi_intr_st(&self) -> ANAPERI_INTR_ST_R {
        ANAPERI_INTR_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn pmu_reg_1_intr_st(&self) -> PMU_REG_1_INTR_ST_R {
        PMU_REG_1_INTR_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn pmu_reg_0_intr_st(&self) -> PMU_REG_0_INTR_ST_R {
        PMU_REG_0_INTR_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn mb_lp_intr_st(&self) -> MB_LP_INTR_ST_R {
        MB_LP_INTR_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn mb_hp_intr_st(&self) -> MB_HP_INTR_ST_R {
        MB_HP_INTR_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn timer_reg_1_intr_st(&self) -> TIMER_REG_1_INTR_ST_R {
        TIMER_REG_1_INTR_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn timer_reg_0_intr_st(&self) -> TIMER_REG_0_INTR_ST_R {
        TIMER_REG_0_INTR_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn wdt_intr_st(&self) -> WDT_INTR_ST_R {
        WDT_INTR_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn rtc_intr_st(&self) -> RTC_INTR_ST_R {
        RTC_INTR_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_intr_st(&self) -> HP_INTR_ST_R {
        HP_INTR_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("huk_intr_st", &self.huk_intr_st())
            .field("sysreg_intr_st", &self.sysreg_intr_st())
            .field("sw_intr_st", &self.sw_intr_st())
            .field("efuse_intr_st", &self.efuse_intr_st())
            .field("uart_intr_st", &self.uart_intr_st())
            .field("tsens_intr_st", &self.tsens_intr_st())
            .field("touch_intr_st", &self.touch_intr_st())
            .field("spi_intr_st", &self.spi_intr_st())
            .field("i2s_intr_st", &self.i2s_intr_st())
            .field("i2c_intr_st", &self.i2c_intr_st())
            .field("gpio_intr_st", &self.gpio_intr_st())
            .field("adc_intr_st", &self.adc_intr_st())
            .field("anaperi_intr_st", &self.anaperi_intr_st())
            .field("pmu_reg_1_intr_st", &self.pmu_reg_1_intr_st())
            .field("pmu_reg_0_intr_st", &self.pmu_reg_0_intr_st())
            .field("mb_lp_intr_st", &self.mb_lp_intr_st())
            .field("mb_hp_intr_st", &self.mb_hp_intr_st())
            .field("timer_reg_1_intr_st", &self.timer_reg_1_intr_st())
            .field("timer_reg_0_intr_st", &self.timer_reg_0_intr_st())
            .field("wdt_intr_st", &self.wdt_intr_st())
            .field("rtc_intr_st", &self.rtc_intr_st())
            .field("hp_intr_st", &self.hp_intr_st())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
