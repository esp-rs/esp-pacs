#[doc = "Register `CHNL1_CFG2` reader"]
pub type R = crate::R<CHNL1_CFG2_SPEC>;
#[doc = "Register `CHNL1_CFG2` writer"]
pub type W = crate::W<CHNL1_CFG2_SPEC>;
#[doc = "Field `CHNL1_FRAC_RECIPL` reader - Write the bits with ((2^19+L)/(2L)) round down in channel1."]
pub type CHNL1_FRAC_RECIPL_R = crate::FieldReader<u32>;
#[doc = "Field `CHNL1_FRAC_RECIPL` writer - Write the bits with ((2^19+L)/(2L)) round down in channel1."]
pub type CHNL1_FRAC_RECIPL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Write the bits with ((2^19+L)/(2L)) round down in channel1."]
    #[inline(always)]
    pub fn chnl1_frac_recipl(&self) -> CHNL1_FRAC_RECIPL_R {
        CHNL1_FRAC_RECIPL_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_CFG2")
            .field("chnl1_frac_recipl", &self.chnl1_frac_recipl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Write the bits with ((2^19+L)/(2L)) round down in channel1."]
    #[inline(always)]
    pub fn chnl1_frac_recipl(&mut self) -> CHNL1_FRAC_RECIPL_W<'_, CHNL1_CFG2_SPEC> {
        CHNL1_FRAC_RECIPL_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_CFG2_SPEC;
impl crate::RegisterSpec for CHNL1_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_cfg2::R`](R) reader structure"]
impl crate::Readable for CHNL1_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl1_cfg2::W`](W) writer structure"]
impl crate::Writable for CHNL1_CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_CFG2 to value 0"]
impl crate::Resettable for CHNL1_CFG2_SPEC {}
