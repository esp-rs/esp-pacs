#[doc = "Register `DOEPDMA` reader"]
pub type R = crate::R<DOEPDMA_SPEC>;
#[doc = "Register `DOEPDMA` writer"]
pub type W = crate::W<DOEPDMA_SPEC>;
#[doc = "Field `DMAADDR` reader - "]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - "]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, DOEPDMA_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA_SPEC;
impl crate::RegisterSpec for DOEPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma::R`](R) reader structure"]
impl crate::Readable for DOEPDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma::W`](W) writer structure"]
impl crate::Writable for DOEPDMA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA to value 0"]
impl crate::Resettable for DOEPDMA_SPEC {}
