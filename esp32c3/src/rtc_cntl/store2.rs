#[doc = "Register `STORE2` reader"]
pub type R = crate::R<STORE2_SPEC>;
#[doc = "Register `STORE2` writer"]
pub type W = crate::W<STORE2_SPEC>;
#[doc = "Field `SCRATCH2` reader - reserved register"]
pub type SCRATCH2_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH2` writer - reserved register"]
pub type SCRATCH2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch2(&self) -> SCRATCH2_R {
        SCRATCH2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE2")
            .field("scratch2", &format_args!("{}", self.scratch2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch2(&mut self) -> SCRATCH2_W<STORE2_SPEC> {
        SCRATCH2_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE2_SPEC;
impl crate::RegisterSpec for STORE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store2::R`](R) reader structure"]
impl crate::Readable for STORE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store2::W`](W) writer structure"]
impl crate::Writable for STORE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE2 to value 0"]
impl crate::Resettable for STORE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
