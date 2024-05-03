#[doc = "Register `PRO_TRACE_1` reader"]
pub type R = crate::R<PRO_TRACE_1_SPEC>;
#[doc = "Register `PRO_TRACE_1` writer"]
pub type W = crate::W<PRO_TRACE_1_SPEC>;
#[doc = "Field `PRO_TRACE_DISABLE` reader - Setting to 1 disables the trace memory function."]
pub type PRO_TRACE_DISABLE_R = crate::BitReader;
#[doc = "Field `PRO_TRACE_DISABLE` writer - Setting to 1 disables the trace memory function."]
pub type PRO_TRACE_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting to 1 disables the trace memory function."]
    #[inline(always)]
    pub fn pro_trace_disable(&self) -> PRO_TRACE_DISABLE_R {
        PRO_TRACE_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TRACE_1")
            .field("pro_trace_disable", &self.pro_trace_disable().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_TRACE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 disables the trace memory function."]
    #[inline(always)]
    #[must_use]
    pub fn pro_trace_disable(&mut self) -> PRO_TRACE_DISABLE_W<PRO_TRACE_1_SPEC> {
        PRO_TRACE_DISABLE_W::new(self, 0)
    }
}
#[doc = "Trace memory permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_trace_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_trace_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_TRACE_1_SPEC;
impl crate::RegisterSpec for PRO_TRACE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_trace_1::R`](R) reader structure"]
impl crate::Readable for PRO_TRACE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_trace_1::W`](W) writer structure"]
impl crate::Writable for PRO_TRACE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_TRACE_1 to value 0"]
impl crate::Resettable for PRO_TRACE_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
