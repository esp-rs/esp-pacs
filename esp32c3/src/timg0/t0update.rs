#[doc = "Register `T0UPDATE` reader"]
pub type R = crate::R<T0UPDATE_SPEC>;
#[doc = "Register `T0UPDATE` writer"]
pub type W = crate::W<T0UPDATE_SPEC>;
#[doc = "Field `UPDATE` reader - t0_update"]
pub type UPDATE_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - t0_update"]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - t0_update"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0UPDATE")
            .field("update", &format_args!("{}", self.update().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - t0_update"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<T0UPDATE_SPEC> {
        UPDATE_W::new(self, 31)
    }
}
#[doc = "TIMG_T0UPDATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0UPDATE_SPEC;
impl crate::RegisterSpec for T0UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0update::R`](R) reader structure"]
impl crate::Readable for T0UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0update::W`](W) writer structure"]
impl crate::Writable for T0UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0UPDATE to value 0"]
impl crate::Resettable for T0UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
