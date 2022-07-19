#[doc = "Register `RTC_ANA_CONF` reader"]
pub struct R(crate::R<RTC_ANA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ANA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ANA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ANA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ANA_CONF` writer"]
pub struct W(crate::W<RTC_ANA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ANA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_ANA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ANA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_RESET_POR_FORCE_PD` reader - Need add desc"]
pub type I2C_RESET_POR_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `I2C_RESET_POR_FORCE_PD` writer - Need add desc"]
pub type I2C_RESET_POR_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 18>;
#[doc = "Field `I2C_RESET_POR_FORCE_PU` reader - Need add desc"]
pub type I2C_RESET_POR_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `I2C_RESET_POR_FORCE_PU` writer - Need add desc"]
pub type I2C_RESET_POR_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 19>;
#[doc = "Field `SAR_I2C_PU` reader - PLLA force power up"]
pub type SAR_I2C_PU_R = crate::BitReader<bool>;
#[doc = "Field `SAR_I2C_PU` writer - PLLA force power up"]
pub type SAR_I2C_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 22>;
#[doc = "Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_CAL_SLP_START` writer - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 25>;
#[doc = "Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up"]
pub type TXRF_I2C_PU_R = crate::BitReader<bool>;
#[doc = "Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up"]
pub type TXRF_I2C_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 27>;
#[doc = "Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up"]
pub type RFRX_PBUS_PU_R = crate::BitReader<bool>;
#[doc = "Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up"]
pub type RFRX_PBUS_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 28>;
#[doc = "Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up"]
pub type CKGEN_I2C_PU_R = crate::BitReader<bool>;
#[doc = "Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up"]
pub type CKGEN_I2C_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 30>;
#[doc = "Field `PLL_I2C_PU` reader - Need add desc"]
pub type PLL_I2C_PU_R = crate::BitReader<bool>;
#[doc = "Field `PLL_I2C_PU` writer - Need add desc"]
pub type PLL_I2C_PU_W<'a> = crate::BitWriter<'a, u32, RTC_ANA_CONF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&self) -> I2C_RESET_POR_FORCE_PD_R {
        I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Need add desc"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&self) -> I2C_RESET_POR_FORCE_PU_R {
        I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    pub fn sar_i2c_pu(&self) -> SAR_I2C_PU_R {
        SAR_I2C_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&mut self) -> I2C_RESET_POR_FORCE_PD_W {
        I2C_RESET_POR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - Need add desc"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&mut self) -> I2C_RESET_POR_FORCE_PU_W {
        I2C_RESET_POR_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    pub fn sar_i2c_pu(&mut self) -> SAR_I2C_PU_W {
        SAR_I2C_PU_W::new(self)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W {
        BBPLL_CAL_SLP_START_W::new(self)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W {
        TXRF_I2C_PU_W::new(self)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W {
        RFRX_PBUS_PU_W::new(self)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W {
        CKGEN_I2C_PU_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W {
        PLL_I2C_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ana_conf](index.html) module"]
pub struct RTC_ANA_CONF_SPEC;
impl crate::RegisterSpec for RTC_ANA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ana_conf::R](R) reader structure"]
impl crate::Readable for RTC_ANA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ana_conf::W](W) writer structure"]
impl crate::Writable for RTC_ANA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ANA_CONF to value 0x0044_0000"]
impl crate::Resettable for RTC_ANA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0044_0000
    }
}
