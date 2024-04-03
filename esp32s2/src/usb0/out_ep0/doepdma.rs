#[doc = "Register `DOEPDMA` reader"]
pub type R = crate::R<DOEPDMA_SPEC>;
#[doc = "Register `DOEPDMA` writer"]
pub type W = crate::W<DOEPDMA_SPEC>;
#[doc = "Field `DMAADDR0` reader - "]
pub type DMAADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR0` writer - "]
pub type DMAADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr0(&self) -> DMAADDR0_R {
        DMAADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA")
            .field("dmaaddr0", &format_args!("{}", self.dmaaddr0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr0(&mut self) -> DMAADDR0_W<DOEPDMA_SPEC> {
        DMAADDR0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA_SPEC;
impl crate::RegisterSpec for DOEPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma::R`](R) reader structure"]
impl crate::Readable for DOEPDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma::W`](W) writer structure"]
impl crate::Writable for DOEPDMA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMA to value 0"]
impl crate::Resettable for DOEPDMA_SPEC {
    const RESET_VALUE: u32 = 0;
}
