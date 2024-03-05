#[doc = "Register `STORE3` reader"]
pub type R = crate::R<STORE3_SPEC>;
#[doc = "Register `STORE3` writer"]
pub type W = crate::W<STORE3_SPEC>;
#[doc = "Field `SCRATCH3` reader - reserved register"]
pub type SCRATCH3_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH3` writer - reserved register"]
pub type SCRATCH3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE3")
            .field("scratch3", &format_args!("{}", self.scratch3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch3(&mut self) -> SCRATCH3_W<STORE3_SPEC> {
        SCRATCH3_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store3::R`](R) reader structure"]
impl crate::Readable for STORE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store3::W`](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {
    const RESET_VALUE: u32 = 0;
}
