#[doc = "Register `STORE1` reader"]
pub type R = crate::R<STORE1_SPEC>;
#[doc = "Register `STORE1` writer"]
pub type W = crate::W<STORE1_SPEC>;
#[doc = "Field `SCRATCH1` reader - reserved register"]
pub type SCRATCH1_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH1` writer - reserved register"]
pub type SCRATCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch1(&self) -> SCRATCH1_R {
        SCRATCH1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE1")
            .field("scratch1", &format_args!("{}", self.scratch1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    #[must_use]
    pub fn scratch1(&mut self) -> SCRATCH1_W<STORE1_SPEC> {
        SCRATCH1_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE1_SPEC;
impl crate::RegisterSpec for STORE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store1::R`](R) reader structure"]
impl crate::Readable for STORE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store1::W`](W) writer structure"]
impl crate::Writable for STORE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE1 to value 0"]
impl crate::Resettable for STORE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
