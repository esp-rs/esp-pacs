#[doc = "Register `TRACE_INTR_MAP` reader"]
pub type R = crate::R<TRACE_INTR_MAP_SPEC>;
#[doc = "Register `TRACE_INTR_MAP` writer"]
pub type W = crate::W<TRACE_INTR_MAP_SPEC>;
#[doc = "Field `TRACE_INTR_MAP` reader - Need add description"]
pub type TRACE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TRACE_INTR_MAP` writer - Need add description"]
pub type TRACE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn trace_intr_map(&self) -> TRACE_INTR_MAP_R {
        TRACE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_INTR_MAP")
            .field(
                "trace_intr_map",
                &format_args!("{}", self.trace_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRACE_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn trace_intr_map(&mut self) -> TRACE_INTR_MAP_W<TRACE_INTR_MAP_SPEC> {
        TRACE_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_INTR_MAP_SPEC;
impl crate::RegisterSpec for TRACE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_intr_map::R`](R) reader structure"]
impl crate::Readable for TRACE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_intr_map::W`](W) writer structure"]
impl crate::Writable for TRACE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACE_INTR_MAP to value 0"]
impl crate::Resettable for TRACE_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
