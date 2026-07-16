#[doc = "Register `CHNL0_CFG2` reader"]
pub type R = crate::R<CHNL0_CFG2_SPEC>;
#[doc = "Register `CHNL0_CFG2` writer"]
pub type W = crate::W<CHNL0_CFG2_SPEC>;
#[doc = "Field `CHNL0_FRAC_RECIPL` reader - Write the bits with ((2^19+L)/(2L)) round down in channel0."]
pub type CHNL0_FRAC_RECIPL_R = crate::FieldReader<u32>;
#[doc = "Field `CHNL0_FRAC_RECIPL` writer - Write the bits with ((2^19+L)/(2L)) round down in channel0."]
pub type CHNL0_FRAC_RECIPL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Write the bits with ((2^19+L)/(2L)) round down in channel0."]
    #[inline(always)]
    pub fn chnl0_frac_recipl(&self) -> CHNL0_FRAC_RECIPL_R {
        CHNL0_FRAC_RECIPL_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_CFG2")
            .field("chnl0_frac_recipl", &self.chnl0_frac_recipl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Write the bits with ((2^19+L)/(2L)) round down in channel0."]
    #[inline(always)]
    pub fn chnl0_frac_recipl(&mut self) -> CHNL0_FRAC_RECIPL_W<'_, CHNL0_CFG2_SPEC> {
        CHNL0_FRAC_RECIPL_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_CFG2_SPEC;
impl crate::RegisterSpec for CHNL0_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_cfg2::R`](R) reader structure"]
impl crate::Readable for CHNL0_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_cfg2::W`](W) writer structure"]
impl crate::Writable for CHNL0_CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_CFG2 to value 0"]
impl crate::Resettable for CHNL0_CFG2_SPEC {}
