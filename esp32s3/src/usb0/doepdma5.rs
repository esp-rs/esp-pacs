#[doc = "Register `DOEPDMA5` reader"]
pub type R = crate::R<DOEPDMA5_SPEC>;
#[doc = "Register `DOEPDMA5` writer"]
pub type W = crate::W<DOEPDMA5_SPEC>;
#[doc = "Field `DMAADDR5` reader - "]
pub type DMAADDR5_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR5` writer - "]
pub type DMAADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr5(&self) -> DMAADDR5_R {
        DMAADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA5")
            .field("dmaaddr5", &format_args!("{}", self.dmaaddr5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr5(&mut self) -> DMAADDR5_W<DOEPDMA5_SPEC> {
        DMAADDR5_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA5_SPEC;
impl crate::RegisterSpec for DOEPDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma5::R`](R) reader structure"]
impl crate::Readable for DOEPDMA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma5::W`](W) writer structure"]
impl crate::Writable for DOEPDMA5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA5 to value 0"]
impl crate::Resettable for DOEPDMA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
