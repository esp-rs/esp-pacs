#[doc = "Register `ANA_CONF` reader"]
pub struct R(crate::R<ANA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CONF` writer"]
pub struct W(crate::W<ANA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CONF_SPEC>;
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
impl From<crate::W<ANA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_POR_FORCE_PD` reader - force no bypass i2c power on reset"]
pub type RESET_POR_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RESET_POR_FORCE_PD` writer - force no bypass i2c power on reset"]
pub type RESET_POR_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `RESET_POR_FORCE_PU` reader - force bypass i2c power on reset"]
pub type RESET_POR_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RESET_POR_FORCE_PU` writer - force bypass i2c power on reset"]
pub type RESET_POR_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `GLITCH_RST_EN` reader - enable glitch reset"]
pub type GLITCH_RST_EN_R = crate::BitReader;
#[doc = "Field `GLITCH_RST_EN` writer - enable glitch reset"]
pub type GLITCH_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `SAR_I2C_PU` reader - PLLA force power up"]
pub type SAR_I2C_PU_R = crate::BitReader;
#[doc = "Field `SAR_I2C_PU` writer - PLLA force power up"]
pub type SAR_I2C_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `PLLA_FORCE_PD` reader - PLLA force power down"]
pub type PLLA_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PLLA_FORCE_PD` writer - PLLA force power down"]
pub type PLLA_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `PLLA_FORCE_PU` reader - PLLA force power up"]
pub type PLLA_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PLLA_FORCE_PU` writer - PLLA force power up"]
pub type PLLA_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_SLP_START` writer - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `PVTMON_PU` reader - 1: PVTMON power up"]
pub type PVTMON_PU_R = crate::BitReader;
#[doc = "Field `PVTMON_PU` writer - 1: PVTMON power up"]
pub type PVTMON_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up"]
pub type TXRF_I2C_PU_R = crate::BitReader;
#[doc = "Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up"]
pub type TXRF_I2C_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up"]
pub type RFRX_PBUS_PU_R = crate::BitReader;
#[doc = "Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up"]
pub type RFRX_PBUS_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up"]
pub type CKGEN_I2C_PU_R = crate::BitReader;
#[doc = "Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up"]
pub type CKGEN_I2C_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
#[doc = "Field `PLL_I2C_PU` reader - power up pll i2c"]
pub type PLL_I2C_PU_R = crate::BitReader;
#[doc = "Field `PLL_I2C_PU` writer - power up pll i2c"]
pub type PLL_I2C_PU_W<'a, const O: u8> = crate::BitWriter<'a, ANA_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 18 - force no bypass i2c power on reset"]
    #[inline(always)]
    pub fn reset_por_force_pd(&self) -> RESET_POR_FORCE_PD_R {
        RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - force bypass i2c power on reset"]
    #[inline(always)]
    pub fn reset_por_force_pu(&self) -> RESET_POR_FORCE_PU_R {
        RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable glitch reset"]
    #[inline(always)]
    pub fn glitch_rst_en(&self) -> GLITCH_RST_EN_R {
        GLITCH_RST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    pub fn sar_i2c_pu(&self) -> SAR_I2C_PU_R {
        SAR_I2C_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: PVTMON power up"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 1) != 0)
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
    #[doc = "Bit 31 - power up pll i2c"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF")
            .field(
                "reset_por_force_pd",
                &format_args!("{}", self.reset_por_force_pd().bit()),
            )
            .field(
                "reset_por_force_pu",
                &format_args!("{}", self.reset_por_force_pu().bit()),
            )
            .field(
                "glitch_rst_en",
                &format_args!("{}", self.glitch_rst_en().bit()),
            )
            .field("sar_i2c_pu", &format_args!("{}", self.sar_i2c_pu().bit()))
            .field(
                "plla_force_pd",
                &format_args!("{}", self.plla_force_pd().bit()),
            )
            .field(
                "plla_force_pu",
                &format_args!("{}", self.plla_force_pu().bit()),
            )
            .field(
                "bbpll_cal_slp_start",
                &format_args!("{}", self.bbpll_cal_slp_start().bit()),
            )
            .field("pvtmon_pu", &format_args!("{}", self.pvtmon_pu().bit()))
            .field("txrf_i2c_pu", &format_args!("{}", self.txrf_i2c_pu().bit()))
            .field(
                "rfrx_pbus_pu",
                &format_args!("{}", self.rfrx_pbus_pu().bit()),
            )
            .field(
                "ckgen_i2c_pu",
                &format_args!("{}", self.ckgen_i2c_pu().bit()),
            )
            .field("pll_i2c_pu", &format_args!("{}", self.pll_i2c_pu().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ANA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - force no bypass i2c power on reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset_por_force_pd(&mut self) -> RESET_POR_FORCE_PD_W<18> {
        RESET_POR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - force bypass i2c power on reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset_por_force_pu(&mut self) -> RESET_POR_FORCE_PU_W<19> {
        RESET_POR_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 20 - enable glitch reset"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_rst_en(&mut self) -> GLITCH_RST_EN_W<20> {
        GLITCH_RST_EN_W::new(self)
    }
    #[doc = "Bit 22 - PLLA force power up"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_pu(&mut self) -> SAR_I2C_PU_W<22> {
        SAR_I2C_PU_W::new(self)
    }
    #[doc = "Bit 23 - PLLA force power down"]
    #[inline(always)]
    #[must_use]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W<23> {
        PLLA_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 24 - PLLA force power up"]
    #[inline(always)]
    #[must_use]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W<24> {
        PLLA_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W<25> {
        BBPLL_CAL_SLP_START_W::new(self)
    }
    #[doc = "Bit 26 - 1: PVTMON power up"]
    #[inline(always)]
    #[must_use]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W<26> {
        PVTMON_PU_W::new(self)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up"]
    #[inline(always)]
    #[must_use]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W<27> {
        TXRF_I2C_PU_W::new(self)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up"]
    #[inline(always)]
    #[must_use]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W<28> {
        RFRX_PBUS_PU_W::new(self)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up"]
    #[inline(always)]
    #[must_use]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W<30> {
        CKGEN_I2C_PU_W::new(self)
    }
    #[doc = "Bit 31 - power up pll i2c"]
    #[inline(always)]
    #[must_use]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W<31> {
        PLL_I2C_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_conf](index.html) module"]
pub struct ANA_CONF_SPEC;
impl crate::RegisterSpec for ANA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_conf::R](R) reader structure"]
impl crate::Readable for ANA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_conf::W](W) writer structure"]
impl crate::Writable for ANA_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CONF to value 0x00c4_0000"]
impl crate::Resettable for ANA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c4_0000;
}
