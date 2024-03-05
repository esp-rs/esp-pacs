#[doc = "Register `STORE4` reader"]
pub type R = crate::R<STORE4_SPEC>;
#[doc = "Register `STORE4` writer"]
pub type W = crate::W<STORE4_SPEC>;
#[doc = "Field `SCRATCH4` reader - reserved register"]
pub type SCRATCH4_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH4` writer - reserved register"]
pub type SCRATCH4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch4(&self) -> SCRATCH4_R {
        SCRATCH4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE4")
            .field("scratch4", &format_args!("{}", self.scratch4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch4(&mut self) -> SCRATCH4_W<STORE4_SPEC> {
        SCRATCH4_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE4_SPEC;
impl crate::RegisterSpec for STORE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store4::R`](R) reader structure"]
impl crate::Readable for STORE4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store4::W`](W) writer structure"]
impl crate::Writable for STORE4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE4 to value 0"]
impl crate::Resettable for STORE4_SPEC {
    const RESET_VALUE: u32 = 0;
}
