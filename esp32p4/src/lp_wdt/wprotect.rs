#[doc = "Register `WPROTECT` reader"]
pub type R = crate::R<WPROTECT_SPEC>;
#[doc = "Register `WPROTECT` writer"]
pub type W = crate::W<WPROTECT_SPEC>;
#[doc = "Field `WDT_WKEY` reader - need_des"]
pub type WDT_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_WKEY` writer - need_des"]
pub type WDT_WKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPROTECT")
            .field("wdt_wkey", &self.wdt_wkey())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<WPROTECT_SPEC> {
        WDT_WKEY_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`wprotect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wprotect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPROTECT_SPEC;
impl crate::RegisterSpec for WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wprotect::R`](R) reader structure"]
impl crate::Readable for WPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wprotect::W`](W) writer structure"]
impl crate::Writable for WPROTECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPROTECT to value 0"]
impl crate::Resettable for WPROTECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
