#[doc = "Register `CACHE_TRACE_ENA` reader"]
pub type R = crate::R<CACHE_TRACE_ENA_SPEC>;
#[doc = "Register `CACHE_TRACE_ENA` writer"]
pub type W = crate::W<CACHE_TRACE_ENA_SPEC>;
#[doc = "Field `CACHE_TRACE_ENA` reader - The bit is used to enable L1-Cache trace for the performance counter and fail tracer"]
pub type CACHE_TRACE_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_TRACE_ENA` writer - The bit is used to enable L1-Cache trace for the performance counter and fail tracer"]
pub type CACHE_TRACE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_CACHE_TRACE_ENA` reader - The bit is used to enable L2-Cache trace for the performance counter and fail tracer"]
pub type CACHE_L2_CACHE_TRACE_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable L1-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn cache_trace_ena(&self) -> CACHE_TRACE_ENA_R {
        CACHE_TRACE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable L2-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn cache_l2_cache_trace_ena(&self) -> CACHE_L2_CACHE_TRACE_ENA_R {
        CACHE_L2_CACHE_TRACE_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TRACE_ENA")
            .field("cache_trace_ena", &self.cache_trace_ena())
            .field("cache_l2_cache_trace_ena", &self.cache_l2_cache_trace_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable L1-Cache trace for the performance counter and fail tracer"]
    #[inline(always)]
    pub fn cache_trace_ena(&mut self) -> CACHE_TRACE_ENA_W<CACHE_TRACE_ENA_SPEC> {
        CACHE_TRACE_ENA_W::new(self, 0)
    }
}
#[doc = "Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_trace_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_trace_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_TRACE_ENA_SPEC;
impl crate::RegisterSpec for CACHE_TRACE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_trace_ena::R`](R) reader structure"]
impl crate::Readable for CACHE_TRACE_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_trace_ena::W`](W) writer structure"]
impl crate::Writable for CACHE_TRACE_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_TRACE_ENA to value 0"]
impl crate::Resettable for CACHE_TRACE_ENA_SPEC {}
