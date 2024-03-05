#[doc = "Register `SEARCH_ENABLE` reader"]
pub type R = crate::R<SEARCH_ENABLE_SPEC>;
#[doc = "Register `SEARCH_ENABLE` writer"]
pub type W = crate::W<SEARCH_ENABLE_SPEC>;
#[doc = "Field `SEARCH_ENABLE` reader - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
pub type SEARCH_ENABLE_R = crate::BitReader;
#[doc = "Field `SEARCH_ENABLE` writer - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
pub type SEARCH_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
    #[inline(always)]
    pub fn search_enable(&self) -> SEARCH_ENABLE_R {
        SEARCH_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEARCH_ENABLE")
            .field(
                "search_enable",
                &format_args!("{}", self.search_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEARCH_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
    #[inline(always)]
    #[must_use]
    pub fn search_enable(&mut self) -> SEARCH_ENABLE_W<SEARCH_ENABLE_SPEC> {
        SEARCH_ENABLE_W::new(self, 0)
    }
}
#[doc = "RSA search option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEARCH_ENABLE_SPEC;
impl crate::RegisterSpec for SEARCH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`search_enable::R`](R) reader structure"]
impl crate::Readable for SEARCH_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`search_enable::W`](W) writer structure"]
impl crate::Writable for SEARCH_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEARCH_ENABLE to value 0"]
impl crate::Resettable for SEARCH_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
