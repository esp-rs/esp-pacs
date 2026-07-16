#[doc = "Register `MODSLP_INTR_WAKE_CONFIG` reader"]
pub type R = crate::R<MODSLP_INTR_WAKE_CONFIG_SPEC>;
#[doc = "Register `MODSLP_INTR_WAKE_CONFIG` writer"]
pub type W = crate::W<MODSLP_INTR_WAKE_CONFIG_SPEC>;
#[doc = "Field `MODSLP_INTR_WAKE_CONFIG` reader - "]
pub type MODSLP_INTR_WAKE_CONFIG_R = crate::FieldReader<u16>;
#[doc = "Field `MODSLP_INTR_WAKE_CONFIG` writer - "]
pub type MODSLP_INTR_WAKE_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn modslp_intr_wake_config(&self) -> MODSLP_INTR_WAKE_CONFIG_R {
        MODSLP_INTR_WAKE_CONFIG_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODSLP_INTR_WAKE_CONFIG")
            .field("modslp_intr_wake_config", &self.modslp_intr_wake_config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn modslp_intr_wake_config(
        &mut self,
    ) -> MODSLP_INTR_WAKE_CONFIG_W<'_, MODSLP_INTR_WAKE_CONFIG_SPEC> {
        MODSLP_INTR_WAKE_CONFIG_W::new(self, 0)
    }
}
#[doc = "MODSLP_INTR_WAKE_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_intr_wake_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_intr_wake_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODSLP_INTR_WAKE_CONFIG_SPEC;
impl crate::RegisterSpec for MODSLP_INTR_WAKE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modslp_intr_wake_config::R`](R) reader structure"]
impl crate::Readable for MODSLP_INTR_WAKE_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modslp_intr_wake_config::W`](W) writer structure"]
impl crate::Writable for MODSLP_INTR_WAKE_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODSLP_INTR_WAKE_CONFIG to value 0"]
impl crate::Resettable for MODSLP_INTR_WAKE_CONFIG_SPEC {}
