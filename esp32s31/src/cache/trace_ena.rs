#[doc = "Register `TRACE_ENA` reader"]
pub type R = crate::R<TRACE_ENA_SPEC>;
#[doc = "Register `TRACE_ENA` writer"]
pub type W = crate::W<TRACE_ENA_SPEC>;
#[doc = "Field `L1_CACHE_TRACE_ENA` reader - Configures L1-Cache trace for the performance counter and fail tracer"]
pub type L1_CACHE_TRACE_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_TRACE_ENA` writer - Configures L1-Cache trace for the performance counter and fail tracer"]
pub type L1_CACHE_TRACE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_TRACE_ENA` reader - Configures L2-Cache trace for the performance counter and fail tracer"]
pub type L2_CACHE_TRACE_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures L1-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn l1_cache_trace_ena(&self) -> L1_CACHE_TRACE_ENA_R {
        L1_CACHE_TRACE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures L2-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn l2_cache_trace_ena(&self) -> L2_CACHE_TRACE_ENA_R {
        L2_CACHE_TRACE_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_ENA")
            .field("l1_cache_trace_ena", &self.l1_cache_trace_ena())
            .field("l2_cache_trace_ena", &self.l2_cache_trace_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures L1-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn l1_cache_trace_ena(&mut self) -> L1_CACHE_TRACE_ENA_W<'_, TRACE_ENA_SPEC> {
        L1_CACHE_TRACE_ENA_W::new(self, 0)
    }
}
#[doc = "Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_ENA_SPEC;
impl crate::RegisterSpec for TRACE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_ena::R`](R) reader structure"]
impl crate::Readable for TRACE_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_ena::W`](W) writer structure"]
impl crate::Writable for TRACE_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACE_ENA to value 0"]
impl crate::Resettable for TRACE_ENA_SPEC {}
