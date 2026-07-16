#[doc = "Register `CHNL0_CFG1` reader"]
pub type R = crate::R<CHNL0_CFG1_SPEC>;
#[doc = "Register `CHNL0_CFG1` writer"]
pub type W = crate::W<CHNL0_CFG1_SPEC>;
#[doc = "Field `CHNL0_FRAC_M` reader - Write the bits to specify the denominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_m and reg_chnl0_frac_l are relatively prime."]
pub type CHNL0_FRAC_M_R = crate::FieldReader<u16>;
#[doc = "Field `CHNL0_FRAC_M` writer - Write the bits to specify the denominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_m and reg_chnl0_frac_l are relatively prime."]
pub type CHNL0_FRAC_M_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CHNL0_FRAC_L` reader - Write the bits to specify the nominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_l and reg_chnl0_frac_m are relatively prime."]
pub type CHNL0_FRAC_L_R = crate::FieldReader<u16>;
#[doc = "Field `CHNL0_FRAC_L` writer - Write the bits to specify the nominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_l and reg_chnl0_frac_m are relatively prime."]
pub type CHNL0_FRAC_L_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Write the bits to specify the denominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_m and reg_chnl0_frac_l are relatively prime."]
    #[inline(always)]
    pub fn chnl0_frac_m(&self) -> CHNL0_FRAC_M_R {
        CHNL0_FRAC_M_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Write the bits to specify the nominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_l and reg_chnl0_frac_m are relatively prime."]
    #[inline(always)]
    pub fn chnl0_frac_l(&self) -> CHNL0_FRAC_L_R {
        CHNL0_FRAC_L_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_CFG1")
            .field("chnl0_frac_m", &self.chnl0_frac_m())
            .field("chnl0_frac_l", &self.chnl0_frac_l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Write the bits to specify the denominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_m and reg_chnl0_frac_l are relatively prime."]
    #[inline(always)]
    pub fn chnl0_frac_m(&mut self) -> CHNL0_FRAC_M_W<'_, CHNL0_CFG1_SPEC> {
        CHNL0_FRAC_M_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Write the bits to specify the nominator of factor of fraction re-sampler in channel0, reg_chnl0_frac_l and reg_chnl0_frac_m are relatively prime."]
    #[inline(always)]
    pub fn chnl0_frac_l(&mut self) -> CHNL0_FRAC_L_W<'_, CHNL0_CFG1_SPEC> {
        CHNL0_FRAC_L_W::new(self, 10)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_CFG1_SPEC;
impl crate::RegisterSpec for CHNL0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_cfg1::R`](R) reader structure"]
impl crate::Readable for CHNL0_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_cfg1::W`](W) writer structure"]
impl crate::Writable for CHNL0_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_CFG1 to value 0"]
impl crate::Resettable for CHNL0_CFG1_SPEC {}
