#[doc = "Register `ANA_CONF` reader"]
pub type R = crate::R<ANA_CONF_SPEC>;
#[doc = "Register `ANA_CONF` writer"]
pub type W = crate::W<ANA_CONF_SPEC>;
#[doc = "Field `I2C_RESET_POR_FORCE_PD` reader - SLEEP_I2CPOR force pd"]
pub type I2C_RESET_POR_FORCE_PD_R = crate::BitReader;
#[doc = "Field `I2C_RESET_POR_FORCE_PD` writer - SLEEP_I2CPOR force pd"]
pub type I2C_RESET_POR_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RESET_POR_FORCE_PU` reader - SLEEP_I2CPOR force pu"]
pub type I2C_RESET_POR_FORCE_PU_R = crate::BitReader;
#[doc = "Field `I2C_RESET_POR_FORCE_PU` writer - SLEEP_I2CPOR force pu"]
pub type I2C_RESET_POR_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_RST_EN` reader - Set this bit to enable a reset when the system detects a glitch."]
pub type GLITCH_RST_EN_R = crate::BitReader;
#[doc = "Field `GLITCH_RST_EN` writer - Set this bit to enable a reset when the system detects a glitch."]
pub type GLITCH_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_I2C_FORCE_PD` reader - Sets this bit to FPD the SAR_I2C."]
pub type SAR_I2C_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SAR_I2C_FORCE_PD` writer - Sets this bit to FPD the SAR_I2C."]
pub type SAR_I2C_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_I2C_FORCE_PU` reader - Sets this bit to FPU the SAR_I2C."]
pub type SAR_I2C_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SAR_I2C_FORCE_PU` writer - Sets this bit to FPU the SAR_I2C."]
pub type SAR_I2C_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLA_FORCE_PD` reader - Sets this bit to FPD the PLLA."]
pub type PLLA_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PLLA_FORCE_PD` writer - Sets this bit to FPD the PLLA."]
pub type PLLA_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLA_FORCE_PU` reader - Sets this bit to FPU the PLLA."]
pub type PLLA_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PLLA_FORCE_PU` writer - Sets this bit to FPU the PLLA."]
pub type PLLA_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_SLP_START` writer - start BBPLL calibration during sleep"]
pub type BBPLL_CAL_SLP_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTMON_PU` reader - 1: PVTMON power up , otherwise power down"]
pub type PVTMON_PU_R = crate::BitReader;
#[doc = "Field `PVTMON_PU` writer - 1: PVTMON power up , otherwise power down"]
pub type PVTMON_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up , otherwise power down"]
pub type TXRF_I2C_PU_R = crate::BitReader;
#[doc = "Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up , otherwise power down"]
pub type TXRF_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up , otherwise power down"]
pub type RFRX_PBUS_PU_R = crate::BitReader;
#[doc = "Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up , otherwise power down"]
pub type RFRX_PBUS_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up , otherwise power down"]
pub type CKGEN_I2C_PU_R = crate::BitReader;
#[doc = "Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up , otherwise power down"]
pub type CKGEN_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_I2C_PU` reader - 1. PLL_I2C power up ,otherwise power down"]
pub type PLL_I2C_PU_R = crate::BitReader;
#[doc = "Field `PLL_I2C_PU` writer - 1. PLL_I2C power up ,otherwise power down"]
pub type PLL_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - SLEEP_I2CPOR force pd"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&self) -> I2C_RESET_POR_FORCE_PD_R {
        I2C_RESET_POR_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SLEEP_I2CPOR force pu"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&self) -> I2C_RESET_POR_FORCE_PU_R {
        I2C_RESET_POR_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable a reset when the system detects a glitch."]
    #[inline(always)]
    pub fn glitch_rst_en(&self) -> GLITCH_RST_EN_R {
        GLITCH_RST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sets this bit to FPD the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pd(&self) -> SAR_I2C_FORCE_PD_R {
        SAR_I2C_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sets this bit to FPU the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pu(&self) -> SAR_I2C_FORCE_PU_R {
        SAR_I2C_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sets this bit to FPD the PLLA."]
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sets this bit to FPU the PLLA."]
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: PVTMON power up , otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up , otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1. PLL_I2C power up ,otherwise power down"]
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF")
            .field("i2c_reset_por_force_pd", &self.i2c_reset_por_force_pd())
            .field("i2c_reset_por_force_pu", &self.i2c_reset_por_force_pu())
            .field("glitch_rst_en", &self.glitch_rst_en())
            .field("sar_i2c_force_pd", &self.sar_i2c_force_pd())
            .field("sar_i2c_force_pu", &self.sar_i2c_force_pu())
            .field("plla_force_pd", &self.plla_force_pd())
            .field("plla_force_pu", &self.plla_force_pu())
            .field("bbpll_cal_slp_start", &self.bbpll_cal_slp_start())
            .field("pvtmon_pu", &self.pvtmon_pu())
            .field("txrf_i2c_pu", &self.txrf_i2c_pu())
            .field("rfrx_pbus_pu", &self.rfrx_pbus_pu())
            .field("ckgen_i2c_pu", &self.ckgen_i2c_pu())
            .field("pll_i2c_pu", &self.pll_i2c_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18 - SLEEP_I2CPOR force pd"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pd(&mut self) -> I2C_RESET_POR_FORCE_PD_W<ANA_CONF_SPEC> {
        I2C_RESET_POR_FORCE_PD_W::new(self, 18)
    }
    #[doc = "Bit 19 - SLEEP_I2CPOR force pu"]
    #[inline(always)]
    pub fn i2c_reset_por_force_pu(&mut self) -> I2C_RESET_POR_FORCE_PU_W<ANA_CONF_SPEC> {
        I2C_RESET_POR_FORCE_PU_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to enable a reset when the system detects a glitch."]
    #[inline(always)]
    pub fn glitch_rst_en(&mut self) -> GLITCH_RST_EN_W<ANA_CONF_SPEC> {
        GLITCH_RST_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Sets this bit to FPD the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pd(&mut self) -> SAR_I2C_FORCE_PD_W<ANA_CONF_SPEC> {
        SAR_I2C_FORCE_PD_W::new(self, 21)
    }
    #[doc = "Bit 22 - Sets this bit to FPU the SAR_I2C."]
    #[inline(always)]
    pub fn sar_i2c_force_pu(&mut self) -> SAR_I2C_FORCE_PU_W<ANA_CONF_SPEC> {
        SAR_I2C_FORCE_PU_W::new(self, 22)
    }
    #[doc = "Bit 23 - Sets this bit to FPD the PLLA."]
    #[inline(always)]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W<ANA_CONF_SPEC> {
        PLLA_FORCE_PD_W::new(self, 23)
    }
    #[doc = "Bit 24 - Sets this bit to FPU the PLLA."]
    #[inline(always)]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W<ANA_CONF_SPEC> {
        PLLA_FORCE_PU_W::new(self, 24)
    }
    #[doc = "Bit 25 - start BBPLL calibration during sleep"]
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W<ANA_CONF_SPEC> {
        BBPLL_CAL_SLP_START_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: PVTMON power up , otherwise power down"]
    #[inline(always)]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W<ANA_CONF_SPEC> {
        PVTMON_PU_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: TXRF_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W<ANA_CONF_SPEC> {
        TXRF_I2C_PU_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: RFRX_PBUS power up , otherwise power down"]
    #[inline(always)]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W<ANA_CONF_SPEC> {
        RFRX_PBUS_PU_W::new(self, 28)
    }
    #[doc = "Bit 30 - 1: CKGEN_I2C power up , otherwise power down"]
    #[inline(always)]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W<ANA_CONF_SPEC> {
        CKGEN_I2C_PU_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1. PLL_I2C power up ,otherwise power down"]
    #[inline(always)]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W<ANA_CONF_SPEC> {
        PLL_I2C_PU_W::new(self, 31)
    }
}
#[doc = "Configures the power options for I2C and PLLA\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CONF_SPEC;
impl crate::RegisterSpec for ANA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_conf::R`](R) reader structure"]
impl crate::Readable for ANA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_conf::W`](W) writer structure"]
impl crate::Writable for ANA_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_CONF to value 0x00a4_0000"]
impl crate::Resettable for ANA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x00a4_0000;
}
