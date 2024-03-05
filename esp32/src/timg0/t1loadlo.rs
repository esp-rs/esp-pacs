#[doc = "Register `T1LOADLO` reader"]
pub type R = crate::R<T1LOADLO_SPEC>;
#[doc = "Register `T1LOADLO` writer"]
pub type W = crate::W<T1LOADLO_SPEC>;
#[doc = "Field `LOAD_LO` reader - Lower 32 bits of the value that will load into timer 1 time-base counter"]
pub type LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - Lower 32 bits of the value that will load into timer 1 time-base counter"]
pub type LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1LOADLO")
            .field("load_lo", &format_args!("{}", self.load_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1LOADLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    #[must_use]
    pub fn load_lo(&mut self) -> LOAD_LO_W<T1LOADLO_SPEC> {
        LOAD_LO_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1LOADLO_SPEC;
impl crate::RegisterSpec for T1LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1loadlo::R`](R) reader structure"]
impl crate::Readable for T1LOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1loadlo::W`](W) writer structure"]
impl crate::Writable for T1LOADLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T1LOADLO to value 0"]
impl crate::Resettable for T1LOADLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
