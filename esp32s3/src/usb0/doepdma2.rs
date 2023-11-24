#[doc = "Register `DOEPDMA2` reader"]
pub type R = crate::R<DOEPDMA2_SPEC>;
#[doc = "Register `DOEPDMA2` writer"]
pub type W = crate::W<DOEPDMA2_SPEC>;
#[doc = "Field `DMAADDR2` reader - "]
pub type DMAADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR2` writer - "]
pub type DMAADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr2(&self) -> DMAADDR2_R {
        DMAADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA2")
            .field("dmaaddr2", &format_args!("{}", self.dmaaddr2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr2(&mut self) -> DMAADDR2_W<DOEPDMA2_SPEC> {
        DMAADDR2_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA2_SPEC;
impl crate::RegisterSpec for DOEPDMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma2::R`](R) reader structure"]
impl crate::Readable for DOEPDMA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma2::W`](W) writer structure"]
impl crate::Writable for DOEPDMA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA2 to value 0"]
impl crate::Resettable for DOEPDMA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
