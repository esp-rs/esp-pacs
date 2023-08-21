#[doc = "Register `DOEPDMA4` reader"]
pub type R = crate::R<DOEPDMA4_SPEC>;
#[doc = "Register `DOEPDMA4` writer"]
pub type W = crate::W<DOEPDMA4_SPEC>;
#[doc = "Field `DMAADDR4` reader - "]
pub type DMAADDR4_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR4` writer - "]
pub type DMAADDR4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr4(&self) -> DMAADDR4_R {
        DMAADDR4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA4")
            .field("dmaaddr4", &format_args!("{}", self.dmaaddr4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr4(&mut self) -> DMAADDR4_W<DOEPDMA4_SPEC, 0> {
        DMAADDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA4_SPEC;
impl crate::RegisterSpec for DOEPDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma4::R`](R) reader structure"]
impl crate::Readable for DOEPDMA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma4::W`](W) writer structure"]
impl crate::Writable for DOEPDMA4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA4 to value 0"]
impl crate::Resettable for DOEPDMA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
