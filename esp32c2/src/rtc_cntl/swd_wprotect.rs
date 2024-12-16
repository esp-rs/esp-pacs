#[doc = "Register `SWD_WPROTECT` reader"]
pub type R = crate::R<SWD_WPROTECT_SPEC>;
#[doc = "Register `SWD_WPROTECT` writer"]
pub type W = crate::W<SWD_WPROTECT_SPEC>;
#[doc = "Field `SWD_WKEY` reader - Need add desc"]
pub type SWD_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `SWD_WKEY` writer - Need add desc"]
pub type SWD_WKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_WPROTECT")
            .field("swd_wkey", &self.swd_wkey())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn swd_wkey(&mut self) -> SWD_WKEY_W<SWD_WPROTECT_SPEC> {
        SWD_WKEY_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_wprotect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_wprotect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWD_WPROTECT_SPEC;
impl crate::RegisterSpec for SWD_WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_wprotect::R`](R) reader structure"]
impl crate::Readable for SWD_WPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swd_wprotect::W`](W) writer structure"]
impl crate::Writable for SWD_WPROTECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWD_WPROTECT to value 0"]
impl crate::Resettable for SWD_WPROTECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
