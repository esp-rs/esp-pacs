#[doc = "Register `CORE0_TRACE_INT_MAP` reader"]
pub type R = crate::R<CORE0_TRACE_INT_MAP_SPEC>;
#[doc = "Register `CORE0_TRACE_INT_MAP` writer"]
pub type W = crate::W<CORE0_TRACE_INT_MAP_SPEC>;
#[doc = "Field `CORE1_CORE0_TRACE_INT_MAP` reader - NA"]
pub type CORE1_CORE0_TRACE_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE1_CORE0_TRACE_INT_MAP` writer - NA"]
pub type CORE1_CORE0_TRACE_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_core0_trace_int_map(&self) -> CORE1_CORE0_TRACE_INT_MAP_R {
        CORE1_CORE0_TRACE_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_TRACE_INT_MAP")
            .field(
                "core1_core0_trace_int_map",
                &self.core1_core0_trace_int_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core1_core0_trace_int_map(
        &mut self,
    ) -> CORE1_CORE0_TRACE_INT_MAP_W<CORE0_TRACE_INT_MAP_SPEC> {
        CORE1_CORE0_TRACE_INT_MAP_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_trace_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_trace_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_TRACE_INT_MAP_SPEC;
impl crate::RegisterSpec for CORE0_TRACE_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_trace_int_map::R`](R) reader structure"]
impl crate::Readable for CORE0_TRACE_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core0_trace_int_map::W`](W) writer structure"]
impl crate::Writable for CORE0_TRACE_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE0_TRACE_INT_MAP to value 0"]
impl crate::Resettable for CORE0_TRACE_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
