#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `LP_HUK_INTR_ST` reader - need_des"]
pub type LP_HUK_INTR_ST_R = crate::BitReader;
#[doc = "Field `SYSREG_INTR_ST` reader - need_des"]
pub type SYSREG_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_SW_INTR_ST` reader - need_des"]
pub type LP_SW_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_EFUSE_INTR_ST` reader - need_des"]
pub type LP_EFUSE_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_UART_INTR_ST` reader - need_des"]
pub type LP_UART_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_TSENS_INTR_ST` reader - need_des"]
pub type LP_TSENS_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_TOUCH_INTR_ST` reader - need_des"]
pub type LP_TOUCH_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_SPI_INTR_ST` reader - need_des"]
pub type LP_SPI_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_I2S_INTR_ST` reader - need_des"]
pub type LP_I2S_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_I2C_INTR_ST` reader - need_des"]
pub type LP_I2C_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_GPIO_INTR_ST` reader - need_des"]
pub type LP_GPIO_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_ADC_INTR_ST` reader - need_des"]
pub type LP_ADC_INTR_ST_R = crate::BitReader;
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
#[doc = "Field `LP_TIMER_REG_1_INTR_ST` reader - need_des"]
pub type LP_TIMER_REG_1_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_TIMER_REG_0_INTR_ST` reader - need_des"]
pub type LP_TIMER_REG_0_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_WDT_INTR_ST` reader - need_des"]
pub type LP_WDT_INTR_ST_R = crate::BitReader;
#[doc = "Field `LP_RTC_INTR_ST` reader - need_des"]
pub type LP_RTC_INTR_ST_R = crate::BitReader;
#[doc = "Field `HP_INTR_ST` reader - need_des"]
pub type HP_INTR_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn lp_huk_intr_st(&self) -> LP_HUK_INTR_ST_R {
        LP_HUK_INTR_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn sysreg_intr_st(&self) -> SYSREG_INTR_ST_R {
        SYSREG_INTR_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn lp_sw_intr_st(&self) -> LP_SW_INTR_ST_R {
        LP_SW_INTR_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn lp_efuse_intr_st(&self) -> LP_EFUSE_INTR_ST_R {
        LP_EFUSE_INTR_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn lp_uart_intr_st(&self) -> LP_UART_INTR_ST_R {
        LP_UART_INTR_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn lp_tsens_intr_st(&self) -> LP_TSENS_INTR_ST_R {
        LP_TSENS_INTR_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn lp_touch_intr_st(&self) -> LP_TOUCH_INTR_ST_R {
        LP_TOUCH_INTR_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn lp_spi_intr_st(&self) -> LP_SPI_INTR_ST_R {
        LP_SPI_INTR_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_intr_st(&self) -> LP_I2S_INTR_ST_R {
        LP_I2S_INTR_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_intr_st(&self) -> LP_I2C_INTR_ST_R {
        LP_I2C_INTR_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_gpio_intr_st(&self) -> LP_GPIO_INTR_ST_R {
        LP_GPIO_INTR_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn lp_adc_intr_st(&self) -> LP_ADC_INTR_ST_R {
        LP_ADC_INTR_ST_R::new(((self.bits >> 21) & 1) != 0)
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
    pub fn lp_timer_reg_1_intr_st(&self) -> LP_TIMER_REG_1_INTR_ST_R {
        LP_TIMER_REG_1_INTR_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_timer_reg_0_intr_st(&self) -> LP_TIMER_REG_0_INTR_ST_R {
        LP_TIMER_REG_0_INTR_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_wdt_intr_st(&self) -> LP_WDT_INTR_ST_R {
        LP_WDT_INTR_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_rtc_intr_st(&self) -> LP_RTC_INTR_ST_R {
        LP_RTC_INTR_ST_R::new(((self.bits >> 30) & 1) != 0)
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
            .field("lp_huk_intr_st", &self.lp_huk_intr_st())
            .field("sysreg_intr_st", &self.sysreg_intr_st())
            .field("lp_sw_intr_st", &self.lp_sw_intr_st())
            .field("lp_efuse_intr_st", &self.lp_efuse_intr_st())
            .field("lp_uart_intr_st", &self.lp_uart_intr_st())
            .field("lp_tsens_intr_st", &self.lp_tsens_intr_st())
            .field("lp_touch_intr_st", &self.lp_touch_intr_st())
            .field("lp_spi_intr_st", &self.lp_spi_intr_st())
            .field("lp_i2s_intr_st", &self.lp_i2s_intr_st())
            .field("lp_i2c_intr_st", &self.lp_i2c_intr_st())
            .field("lp_gpio_intr_st", &self.lp_gpio_intr_st())
            .field("lp_adc_intr_st", &self.lp_adc_intr_st())
            .field("anaperi_intr_st", &self.anaperi_intr_st())
            .field("pmu_reg_1_intr_st", &self.pmu_reg_1_intr_st())
            .field("pmu_reg_0_intr_st", &self.pmu_reg_0_intr_st())
            .field("mb_lp_intr_st", &self.mb_lp_intr_st())
            .field("mb_hp_intr_st", &self.mb_hp_intr_st())
            .field("lp_timer_reg_1_intr_st", &self.lp_timer_reg_1_intr_st())
            .field("lp_timer_reg_0_intr_st", &self.lp_timer_reg_0_intr_st())
            .field("lp_wdt_intr_st", &self.lp_wdt_intr_st())
            .field("lp_rtc_intr_st", &self.lp_rtc_intr_st())
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
