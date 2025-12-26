#[doc = "Register `BLETIMGENCNTL` reader"]
pub type R = crate::R<BLETIMGENCNTL_SPEC>;
#[doc = "Register `BLETIMGENCNTL` writer"]
pub type W = crate::W<BLETIMGENCNTL_SPEC>;
#[doc = "Field `PREFETCH_TIME` reader - Exchange Table prefetch instant (us)"]
pub type PREFETCH_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `PREFETCH_TIME` writer - Exchange Table prefetch instant (us)"]
pub type PREFETCH_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PREFETCHABORT_TIME` reader - Pre-Fetch abort instant (us)"]
pub type PREFETCHABORT_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `PREFETCHABORT_TIME` writer - Pre-Fetch abort instant (us)"]
pub type PREFETCHABORT_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `APFM_EN` reader - Enable Anticipated Pre-Fetch Abort mechanism"]
pub type APFM_EN_R = crate::BitReader;
#[doc = "Field `APFM_EN` writer - Enable Anticipated Pre-Fetch Abort mechanism"]
pub type APFM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Exchange Table prefetch instant (us)"]
    #[inline(always)]
    pub fn prefetch_time(&self) -> PREFETCH_TIME_R {
        PREFETCH_TIME_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:25 - Pre-Fetch abort instant (us)"]
    #[inline(always)]
    pub fn prefetchabort_time(&self) -> PREFETCHABORT_TIME_R {
        PREFETCHABORT_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable Anticipated Pre-Fetch Abort mechanism"]
    #[inline(always)]
    pub fn apfm_en(&self) -> APFM_EN_R {
        APFM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLETIMGENCNTL")
            .field("prefetch_time", &self.prefetch_time())
            .field("prefetchabort_time", &self.prefetchabort_time())
            .field("apfm_en", &self.apfm_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Exchange Table prefetch instant (us)"]
    #[inline(always)]
    pub fn prefetch_time(&mut self) -> PREFETCH_TIME_W<'_, BLETIMGENCNTL_SPEC> {
        PREFETCH_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Pre-Fetch abort instant (us)"]
    #[inline(always)]
    pub fn prefetchabort_time(&mut self) -> PREFETCHABORT_TIME_W<'_, BLETIMGENCNTL_SPEC> {
        PREFETCHABORT_TIME_W::new(self, 16)
    }
    #[doc = "Bit 31 - Enable Anticipated Pre-Fetch Abort mechanism"]
    #[inline(always)]
    pub fn apfm_en(&mut self) -> APFM_EN_W<'_, BLETIMGENCNTL_SPEC> {
        APFM_EN_W::new(self, 31)
    }
}
#[doc = "Pre-Fecth mechanism control\n\nYou can [`read`](crate::Reg::read) this register and get [`bletimgencntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bletimgencntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLETIMGENCNTL_SPEC;
impl crate::RegisterSpec for BLETIMGENCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bletimgencntl::R`](R) reader structure"]
impl crate::Readable for BLETIMGENCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bletimgencntl::W`](W) writer structure"]
impl crate::Writable for BLETIMGENCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLETIMGENCNTL to value 0"]
impl crate::Resettable for BLETIMGENCNTL_SPEC {}
