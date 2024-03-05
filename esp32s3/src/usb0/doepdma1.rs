#[doc = "Register `DOEPDMA1` reader"]
pub type R = crate::R<DOEPDMA1_SPEC>;
#[doc = "Register `DOEPDMA1` writer"]
pub type W = crate::W<DOEPDMA1_SPEC>;
#[doc = "Field `DMAADDR1` reader - "]
pub type DMAADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR1` writer - "]
pub type DMAADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr1(&self) -> DMAADDR1_R {
        DMAADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA1")
            .field("dmaaddr1", &format_args!("{}", self.dmaaddr1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr1(&mut self) -> DMAADDR1_W<DOEPDMA1_SPEC> {
        DMAADDR1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA1_SPEC;
impl crate::RegisterSpec for DOEPDMA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma1::R`](R) reader structure"]
impl crate::Readable for DOEPDMA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma1::W`](W) writer structure"]
impl crate::Writable for DOEPDMA1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMA1 to value 0"]
impl crate::Resettable for DOEPDMA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
