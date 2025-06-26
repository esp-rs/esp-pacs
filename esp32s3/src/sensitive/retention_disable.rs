#[doc = "Register `RETENTION_DISABLE` reader"]
pub type R = crate::R<RETENTION_DISABLE_SPEC>;
#[doc = "Register `RETENTION_DISABLE` writer"]
pub type W = crate::W<RETENTION_DISABLE_SPEC>;
#[doc = "Field `RETENTION_DISABLE` reader - Set 1 to disable retention function and lock disable state."]
pub type RETENTION_DISABLE_R = crate::BitReader;
#[doc = "Field `RETENTION_DISABLE` writer - Set 1 to disable retention function and lock disable state."]
pub type RETENTION_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    pub fn retention_disable(&self) -> RETENTION_DISABLE_R {
        RETENTION_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_DISABLE")
            .field("retention_disable", &self.retention_disable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    pub fn retention_disable(&mut self) -> RETENTION_DISABLE_W<RETENTION_DISABLE_SPEC> {
        RETENTION_DISABLE_W::new(self, 0)
    }
}
#[doc = "Retention configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`retention_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`retention_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_DISABLE_SPEC;
impl crate::RegisterSpec for RETENTION_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_disable::R`](R) reader structure"]
impl crate::Readable for RETENTION_DISABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_disable::W`](W) writer structure"]
impl crate::Writable for RETENTION_DISABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RETENTION_DISABLE to value 0"]
impl crate::Resettable for RETENTION_DISABLE_SPEC {}
