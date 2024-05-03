#[doc = "Register `PIF_ACCESS_MONITOR_0` reader"]
pub type R = crate::R<PIF_ACCESS_MONITOR_0_SPEC>;
#[doc = "Register `PIF_ACCESS_MONITOR_0` writer"]
pub type W = crate::W<PIF_ACCESS_MONITOR_0_SPEC>;
#[doc = "Field `PIF_ACCESS_MONITOR_LOCK` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_LOCK_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_LOCK` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_lock(&self) -> PIF_ACCESS_MONITOR_LOCK_R {
        PIF_ACCESS_MONITOR_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_ACCESS_MONITOR_0")
            .field(
                "pif_access_monitor_lock",
                &self.pif_access_monitor_lock().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIF_ACCESS_MONITOR_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn pif_access_monitor_lock(
        &mut self,
    ) -> PIF_ACCESS_MONITOR_LOCK_W<PIF_ACCESS_MONITOR_0_SPEC> {
        PIF_ACCESS_MONITOR_LOCK_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pif_access_monitor_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pif_access_monitor_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIF_ACCESS_MONITOR_0_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pif_access_monitor_0::R`](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pif_access_monitor_0::W`](W) writer structure"]
impl crate::Writable for PIF_ACCESS_MONITOR_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_0 to value 0"]
impl crate::Resettable for PIF_ACCESS_MONITOR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
