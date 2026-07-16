#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `LP_DAC_INTR_EN` reader - "]
pub type LP_DAC_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_DAC_INTR_EN` writer - "]
pub type LP_DAC_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SW_INVALID_SLEEP_INTR_EN` reader - "]
pub type LP_SW_INVALID_SLEEP_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_SW_INVALID_SLEEP_INTR_EN` writer - "]
pub type LP_SW_INVALID_SLEEP_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AHB_PDMA_OUT_CH1_INTR_EN` reader - "]
pub type LP_AHB_PDMA_OUT_CH1_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_PDMA_OUT_CH1_INTR_EN` writer - "]
pub type LP_AHB_PDMA_OUT_CH1_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AHB_PDMA_OUT_CH0_INTR_EN` reader - "]
pub type LP_AHB_PDMA_OUT_CH0_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_PDMA_OUT_CH0_INTR_EN` writer - "]
pub type LP_AHB_PDMA_OUT_CH0_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AHB_PDMA_IN_CH1_INTR_EN` reader - "]
pub type LP_AHB_PDMA_IN_CH1_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_PDMA_IN_CH1_INTR_EN` writer - "]
pub type LP_AHB_PDMA_IN_CH1_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AHB_PDMA_IN_CH0_INTR_EN` reader - "]
pub type LP_AHB_PDMA_IN_CH0_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_PDMA_IN_CH0_INTR_EN` writer - "]
pub type LP_AHB_PDMA_IN_CH0_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PERI_PMS_INTR_EN` reader - "]
pub type LP_PERI_PMS_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_PERI_PMS_INTR_EN` writer - "]
pub type LP_PERI_PMS_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PERI_TIMEOUT_INTR_EN` reader - "]
pub type LP_PERI_TIMEOUT_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_PERI_TIMEOUT_INTR_EN` writer - "]
pub type LP_PERI_TIMEOUT_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TRNG_INTR_EN` reader - "]
pub type LP_TRNG_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_TRNG_INTR_EN` writer - "]
pub type LP_TRNG_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_HUK_INTR_EN` reader - "]
pub type LP_HUK_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_HUK_INTR_EN` writer - "]
pub type LP_HUK_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSREG_INTR_EN` reader - "]
pub type SYSREG_INTR_EN_R = crate::BitReader;
#[doc = "Field `SYSREG_INTR_EN` writer - "]
pub type SYSREG_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SW_INTR_EN` reader - "]
pub type LP_SW_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_SW_INTR_EN` writer - "]
pub type LP_SW_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_EFUSE_INTR_EN` reader - "]
pub type LP_EFUSE_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_EFUSE_INTR_EN` writer - "]
pub type LP_EFUSE_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_INTR_EN` reader - "]
pub type LP_UART_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_INTR_EN` writer - "]
pub type LP_UART_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TSENS_INTR_EN` reader - "]
pub type LP_TSENS_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_TSENS_INTR_EN` writer - "]
pub type LP_TSENS_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TOUCH_INTR_EN` reader - "]
pub type LP_TOUCH_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_TOUCH_INTR_EN` writer - "]
pub type LP_TOUCH_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_INTR_EN` reader - "]
pub type LP_SPI_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_SPI_INTR_EN` writer - "]
pub type LP_SPI_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2C_INTR_EN` reader - "]
pub type LP_I2C_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_I2C_INTR_EN` writer - "]
pub type LP_I2C_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO_INTR_EN` reader - "]
pub type LP_GPIO_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_GPIO_INTR_EN` writer - "]
pub type LP_GPIO_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ADC_INTR_EN` reader - "]
pub type LP_ADC_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_ADC_INTR_EN` writer - "]
pub type LP_ADC_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANAPERI_INTR_EN` reader - "]
pub type ANAPERI_INTR_EN_R = crate::BitReader;
#[doc = "Field `ANAPERI_INTR_EN` writer - "]
pub type ANAPERI_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_REG_1_INTR_EN` reader - "]
pub type PMU_REG_1_INTR_EN_R = crate::BitReader;
#[doc = "Field `PMU_REG_1_INTR_EN` writer - "]
pub type PMU_REG_1_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_REG_0_INTR_EN` reader - "]
pub type PMU_REG_0_INTR_EN_R = crate::BitReader;
#[doc = "Field `PMU_REG_0_INTR_EN` writer - "]
pub type PMU_REG_0_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB_LP_INTR_EN` reader - "]
pub type MB_LP_INTR_EN_R = crate::BitReader;
#[doc = "Field `MB_LP_INTR_EN` writer - "]
pub type MB_LP_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB_HP_INTR_EN` reader - "]
pub type MB_HP_INTR_EN_R = crate::BitReader;
#[doc = "Field `MB_HP_INTR_EN` writer - "]
pub type MB_HP_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_REG_1_INTR_EN` reader - "]
pub type LP_TIMER_REG_1_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_TIMER_REG_1_INTR_EN` writer - "]
pub type LP_TIMER_REG_1_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_REG_0_INTR_EN` reader - "]
pub type LP_TIMER_REG_0_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_TIMER_REG_0_INTR_EN` writer - "]
pub type LP_TIMER_REG_0_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_WDT_INTR_EN` reader - "]
pub type LP_WDT_INTR_EN_R = crate::BitReader;
#[doc = "Field `LP_WDT_INTR_EN` writer - "]
pub type LP_WDT_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APM_INTR_EN` reader - "]
pub type APM_INTR_EN_R = crate::BitReader;
#[doc = "Field `APM_INTR_EN` writer - "]
pub type APM_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_INTR_EN` reader - "]
pub type HP_INTR_EN_R = crate::BitReader;
#[doc = "Field `HP_INTR_EN` writer - "]
pub type HP_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lp_dac_intr_en(&self) -> LP_DAC_INTR_EN_R {
        LP_DAC_INTR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_sw_invalid_sleep_intr_en(&self) -> LP_SW_INVALID_SLEEP_INTR_EN_R {
        LP_SW_INVALID_SLEEP_INTR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_ahb_pdma_out_ch1_intr_en(&self) -> LP_AHB_PDMA_OUT_CH1_INTR_EN_R {
        LP_AHB_PDMA_OUT_CH1_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lp_ahb_pdma_out_ch0_intr_en(&self) -> LP_AHB_PDMA_OUT_CH0_INTR_EN_R {
        LP_AHB_PDMA_OUT_CH0_INTR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lp_ahb_pdma_in_ch1_intr_en(&self) -> LP_AHB_PDMA_IN_CH1_INTR_EN_R {
        LP_AHB_PDMA_IN_CH1_INTR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lp_ahb_pdma_in_ch0_intr_en(&self) -> LP_AHB_PDMA_IN_CH0_INTR_EN_R {
        LP_AHB_PDMA_IN_CH0_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lp_peri_pms_intr_en(&self) -> LP_PERI_PMS_INTR_EN_R {
        LP_PERI_PMS_INTR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lp_peri_timeout_intr_en(&self) -> LP_PERI_TIMEOUT_INTR_EN_R {
        LP_PERI_TIMEOUT_INTR_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lp_trng_intr_en(&self) -> LP_TRNG_INTR_EN_R {
        LP_TRNG_INTR_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lp_huk_intr_en(&self) -> LP_HUK_INTR_EN_R {
        LP_HUK_INTR_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sysreg_intr_en(&self) -> SYSREG_INTR_EN_R {
        SYSREG_INTR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lp_sw_intr_en(&self) -> LP_SW_INTR_EN_R {
        LP_SW_INTR_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lp_efuse_intr_en(&self) -> LP_EFUSE_INTR_EN_R {
        LP_EFUSE_INTR_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lp_uart_intr_en(&self) -> LP_UART_INTR_EN_R {
        LP_UART_INTR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lp_tsens_intr_en(&self) -> LP_TSENS_INTR_EN_R {
        LP_TSENS_INTR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lp_touch_intr_en(&self) -> LP_TOUCH_INTR_EN_R {
        LP_TOUCH_INTR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lp_spi_intr_en(&self) -> LP_SPI_INTR_EN_R {
        LP_SPI_INTR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn lp_i2c_intr_en(&self) -> LP_I2C_INTR_EN_R {
        LP_I2C_INTR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lp_gpio_intr_en(&self) -> LP_GPIO_INTR_EN_R {
        LP_GPIO_INTR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lp_adc_intr_en(&self) -> LP_ADC_INTR_EN_R {
        LP_ADC_INTR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn anaperi_intr_en(&self) -> ANAPERI_INTR_EN_R {
        ANAPERI_INTR_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pmu_reg_1_intr_en(&self) -> PMU_REG_1_INTR_EN_R {
        PMU_REG_1_INTR_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pmu_reg_0_intr_en(&self) -> PMU_REG_0_INTR_EN_R {
        PMU_REG_0_INTR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mb_lp_intr_en(&self) -> MB_LP_INTR_EN_R {
        MB_LP_INTR_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mb_hp_intr_en(&self) -> MB_HP_INTR_EN_R {
        MB_HP_INTR_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn lp_timer_reg_1_intr_en(&self) -> LP_TIMER_REG_1_INTR_EN_R {
        LP_TIMER_REG_1_INTR_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lp_timer_reg_0_intr_en(&self) -> LP_TIMER_REG_0_INTR_EN_R {
        LP_TIMER_REG_0_INTR_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lp_wdt_intr_en(&self) -> LP_WDT_INTR_EN_R {
        LP_WDT_INTR_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apm_intr_en(&self) -> APM_INTR_EN_R {
        APM_INTR_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn hp_intr_en(&self) -> HP_INTR_EN_R {
        HP_INTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field("lp_dac_intr_en", &self.lp_dac_intr_en())
            .field(
                "lp_sw_invalid_sleep_intr_en",
                &self.lp_sw_invalid_sleep_intr_en(),
            )
            .field(
                "lp_ahb_pdma_out_ch1_intr_en",
                &self.lp_ahb_pdma_out_ch1_intr_en(),
            )
            .field(
                "lp_ahb_pdma_out_ch0_intr_en",
                &self.lp_ahb_pdma_out_ch0_intr_en(),
            )
            .field(
                "lp_ahb_pdma_in_ch1_intr_en",
                &self.lp_ahb_pdma_in_ch1_intr_en(),
            )
            .field(
                "lp_ahb_pdma_in_ch0_intr_en",
                &self.lp_ahb_pdma_in_ch0_intr_en(),
            )
            .field("lp_peri_pms_intr_en", &self.lp_peri_pms_intr_en())
            .field("lp_peri_timeout_intr_en", &self.lp_peri_timeout_intr_en())
            .field("lp_trng_intr_en", &self.lp_trng_intr_en())
            .field("lp_huk_intr_en", &self.lp_huk_intr_en())
            .field("sysreg_intr_en", &self.sysreg_intr_en())
            .field("lp_sw_intr_en", &self.lp_sw_intr_en())
            .field("lp_efuse_intr_en", &self.lp_efuse_intr_en())
            .field("lp_uart_intr_en", &self.lp_uart_intr_en())
            .field("lp_tsens_intr_en", &self.lp_tsens_intr_en())
            .field("lp_touch_intr_en", &self.lp_touch_intr_en())
            .field("lp_spi_intr_en", &self.lp_spi_intr_en())
            .field("lp_i2c_intr_en", &self.lp_i2c_intr_en())
            .field("lp_gpio_intr_en", &self.lp_gpio_intr_en())
            .field("lp_adc_intr_en", &self.lp_adc_intr_en())
            .field("anaperi_intr_en", &self.anaperi_intr_en())
            .field("pmu_reg_1_intr_en", &self.pmu_reg_1_intr_en())
            .field("pmu_reg_0_intr_en", &self.pmu_reg_0_intr_en())
            .field("mb_lp_intr_en", &self.mb_lp_intr_en())
            .field("mb_hp_intr_en", &self.mb_hp_intr_en())
            .field("lp_timer_reg_1_intr_en", &self.lp_timer_reg_1_intr_en())
            .field("lp_timer_reg_0_intr_en", &self.lp_timer_reg_0_intr_en())
            .field("lp_wdt_intr_en", &self.lp_wdt_intr_en())
            .field("apm_intr_en", &self.apm_intr_en())
            .field("hp_intr_en", &self.hp_intr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lp_dac_intr_en(&mut self) -> LP_DAC_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_DAC_INTR_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_sw_invalid_sleep_intr_en(
        &mut self,
    ) -> LP_SW_INVALID_SLEEP_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_SW_INVALID_SLEEP_INTR_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_ahb_pdma_out_ch1_intr_en(
        &mut self,
    ) -> LP_AHB_PDMA_OUT_CH1_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_AHB_PDMA_OUT_CH1_INTR_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lp_ahb_pdma_out_ch0_intr_en(
        &mut self,
    ) -> LP_AHB_PDMA_OUT_CH0_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_AHB_PDMA_OUT_CH0_INTR_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lp_ahb_pdma_in_ch1_intr_en(&mut self) -> LP_AHB_PDMA_IN_CH1_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_AHB_PDMA_IN_CH1_INTR_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lp_ahb_pdma_in_ch0_intr_en(&mut self) -> LP_AHB_PDMA_IN_CH0_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_AHB_PDMA_IN_CH0_INTR_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lp_peri_pms_intr_en(&mut self) -> LP_PERI_PMS_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_PERI_PMS_INTR_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lp_peri_timeout_intr_en(&mut self) -> LP_PERI_TIMEOUT_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_PERI_TIMEOUT_INTR_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lp_trng_intr_en(&mut self) -> LP_TRNG_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_TRNG_INTR_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lp_huk_intr_en(&mut self) -> LP_HUK_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_HUK_INTR_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sysreg_intr_en(&mut self) -> SYSREG_INTR_EN_W<'_, ENABLE_SPEC> {
        SYSREG_INTR_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lp_sw_intr_en(&mut self) -> LP_SW_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_SW_INTR_EN_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lp_efuse_intr_en(&mut self) -> LP_EFUSE_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_EFUSE_INTR_EN_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lp_uart_intr_en(&mut self) -> LP_UART_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_UART_INTR_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lp_tsens_intr_en(&mut self) -> LP_TSENS_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_TSENS_INTR_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lp_touch_intr_en(&mut self) -> LP_TOUCH_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_TOUCH_INTR_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lp_spi_intr_en(&mut self) -> LP_SPI_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_SPI_INTR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn lp_i2c_intr_en(&mut self) -> LP_I2C_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_I2C_INTR_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lp_gpio_intr_en(&mut self) -> LP_GPIO_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_GPIO_INTR_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lp_adc_intr_en(&mut self) -> LP_ADC_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_ADC_INTR_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn anaperi_intr_en(&mut self) -> ANAPERI_INTR_EN_W<'_, ENABLE_SPEC> {
        ANAPERI_INTR_EN_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pmu_reg_1_intr_en(&mut self) -> PMU_REG_1_INTR_EN_W<'_, ENABLE_SPEC> {
        PMU_REG_1_INTR_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pmu_reg_0_intr_en(&mut self) -> PMU_REG_0_INTR_EN_W<'_, ENABLE_SPEC> {
        PMU_REG_0_INTR_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mb_lp_intr_en(&mut self) -> MB_LP_INTR_EN_W<'_, ENABLE_SPEC> {
        MB_LP_INTR_EN_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mb_hp_intr_en(&mut self) -> MB_HP_INTR_EN_W<'_, ENABLE_SPEC> {
        MB_HP_INTR_EN_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn lp_timer_reg_1_intr_en(&mut self) -> LP_TIMER_REG_1_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_TIMER_REG_1_INTR_EN_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lp_timer_reg_0_intr_en(&mut self) -> LP_TIMER_REG_0_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_TIMER_REG_0_INTR_EN_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lp_wdt_intr_en(&mut self) -> LP_WDT_INTR_EN_W<'_, ENABLE_SPEC> {
        LP_WDT_INTR_EN_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apm_intr_en(&mut self) -> APM_INTR_EN_W<'_, ENABLE_SPEC> {
        APM_INTR_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn hp_intr_en(&mut self) -> HP_INTR_EN_W<'_, ENABLE_SPEC> {
        HP_INTR_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE to value 0xffff_fffc"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u32 = 0xffff_fffc;
}
