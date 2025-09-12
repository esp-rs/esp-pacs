#[doc = "Register `POWER_GLITCH_CNTL` reader"]
pub type R = crate::R<POWER_GLITCH_CNTL_SPEC>;
#[doc = "Register `POWER_GLITCH_CNTL` writer"]
pub type W = crate::W<POWER_GLITCH_CNTL_SPEC>;
#[doc = "Field `POWER_GLITCH_RESET_ENA` reader - need_des"]
pub type POWER_GLITCH_RESET_ENA_R = crate::FieldReader;
#[doc = "Field `POWER_GLITCH_RESET_ENA` writer - need_des"]
pub type POWER_GLITCH_RESET_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(&self) -> POWER_GLITCH_RESET_ENA_R {
        POWER_GLITCH_RESET_ENA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_GLITCH_CNTL")
            .field("power_glitch_reset_ena", &self.power_glitch_reset_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(
        &mut self,
    ) -> POWER_GLITCH_RESET_ENA_W<'_, POWER_GLITCH_CNTL_SPEC> {
        POWER_GLITCH_RESET_ENA_W::new(self, 28)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_glitch_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_glitch_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_GLITCH_CNTL_SPEC;
impl crate::RegisterSpec for POWER_GLITCH_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_glitch_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_GLITCH_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_glitch_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_GLITCH_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_GLITCH_CNTL to value 0"]
impl crate::Resettable for POWER_GLITCH_CNTL_SPEC {}
