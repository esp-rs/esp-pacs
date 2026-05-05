#[doc = "Register `DOUT_MODE` reader"]
pub type R = crate::R<DOUT_MODE_SPEC>;
#[doc = "Register `DOUT_MODE` writer"]
pub type W = crate::W<DOUT_MODE_SPEC>;
#[doc = "Field `LP_REG_DOUT0_MODE` reader - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT0_MODE_R = crate::BitReader;
#[doc = "Field `LP_REG_DOUT0_MODE` writer - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_DOUT1_MODE` reader - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT1_MODE_R = crate::BitReader;
#[doc = "Field `LP_REG_DOUT1_MODE` writer - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_DOUT2_MODE` reader - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT2_MODE_R = crate::BitReader;
#[doc = "Field `LP_REG_DOUT2_MODE` writer - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT2_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_DOUT3_MODE` reader - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT3_MODE_R = crate::BitReader;
#[doc = "Field `LP_REG_DOUT3_MODE` writer - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type LP_REG_DOUT3_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout0_mode(&self) -> LP_REG_DOUT0_MODE_R {
        LP_REG_DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout1_mode(&self) -> LP_REG_DOUT1_MODE_R {
        LP_REG_DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout2_mode(&self) -> LP_REG_DOUT2_MODE_R {
        LP_REG_DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout3_mode(&self) -> LP_REG_DOUT3_MODE_R {
        LP_REG_DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_MODE")
            .field("lp_reg_dout0_mode", &self.lp_reg_dout0_mode())
            .field("lp_reg_dout1_mode", &self.lp_reg_dout1_mode())
            .field("lp_reg_dout2_mode", &self.lp_reg_dout2_mode())
            .field("lp_reg_dout3_mode", &self.lp_reg_dout3_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout0_mode(&mut self) -> LP_REG_DOUT0_MODE_W<'_, DOUT_MODE_SPEC> {
        LP_REG_DOUT0_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout1_mode(&mut self) -> LP_REG_DOUT1_MODE_W<'_, DOUT_MODE_SPEC> {
        LP_REG_DOUT1_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout2_mode(&mut self) -> LP_REG_DOUT2_MODE_W<'_, DOUT_MODE_SPEC> {
        LP_REG_DOUT2_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - The output signal $n is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dout3_mode(&mut self) -> LP_REG_DOUT3_MODE_W<'_, DOUT_MODE_SPEC> {
        LP_REG_DOUT3_MODE_W::new(self, 3)
    }
}
#[doc = "SPI output delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_mode::R`](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dout_mode::W`](W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {}
