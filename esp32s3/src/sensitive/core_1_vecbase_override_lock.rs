#[doc = "Register `CORE_1_VECBASE_OVERRIDE_LOCK` reader"]
pub type R = crate::R<CORE_1_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "Register `CORE_1_VECBASE_OVERRIDE_LOCK` writer"]
pub type W = crate::W<CORE_1_VECBASE_OVERRIDE_LOCK_SPEC>;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_LOCK` reader - Set 1 to lock core1 vecbase configuration register"]
pub type CORE_1_VECBASE_OVERRIDE_LOCK_R = crate::BitReader;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_LOCK` writer - Set 1 to lock core1 vecbase configuration register"]
pub type CORE_1_VECBASE_OVERRIDE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock core1 vecbase configuration register"]
    #[inline(always)]
    pub fn core_1_vecbase_override_lock(&self) -> CORE_1_VECBASE_OVERRIDE_LOCK_R {
        CORE_1_VECBASE_OVERRIDE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_VECBASE_OVERRIDE_LOCK")
            .field(
                "core_1_vecbase_override_lock",
                &self.core_1_vecbase_override_lock(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock core1 vecbase configuration register"]
    #[inline(always)]
    pub fn core_1_vecbase_override_lock(
        &mut self,
    ) -> CORE_1_VECBASE_OVERRIDE_LOCK_W<CORE_1_VECBASE_OVERRIDE_LOCK_SPEC> {
        CORE_1_VECBASE_OVERRIDE_LOCK_W::new(self, 0)
    }
}
#[doc = "core1 vecbase override configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_vecbase_override_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_vecbase_override_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_VECBASE_OVERRIDE_LOCK_SPEC;
impl crate::RegisterSpec for CORE_1_VECBASE_OVERRIDE_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_vecbase_override_lock::R`](R) reader structure"]
impl crate::Readable for CORE_1_VECBASE_OVERRIDE_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_vecbase_override_lock::W`](W) writer structure"]
impl crate::Writable for CORE_1_VECBASE_OVERRIDE_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_VECBASE_OVERRIDE_LOCK to value 0"]
impl crate::Resettable for CORE_1_VECBASE_OVERRIDE_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
