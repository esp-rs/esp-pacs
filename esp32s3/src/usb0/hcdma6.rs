#[doc = "Register `HCDMA6` reader"]
pub type R = crate::R<HCDMA6_SPEC>;
#[doc = "Register `HCDMA6` writer"]
pub type W = crate::W<HCDMA6_SPEC>;
#[doc = "Field `H_DMAADDR6` reader - "]
pub type H_DMAADDR6_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR6` writer - "]
pub type H_DMAADDR6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr6(&self) -> H_DMAADDR6_R {
        H_DMAADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA6")
            .field("h_dmaaddr6", &format_args!("{}", self.h_dmaaddr6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr6(&mut self) -> H_DMAADDR6_W<HCDMA6_SPEC, 0> {
        H_DMAADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA6_SPEC;
impl crate::RegisterSpec for HCDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma6::R`](R) reader structure"]
impl crate::Readable for HCDMA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma6::W`](W) writer structure"]
impl crate::Writable for HCDMA6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA6 to value 0"]
impl crate::Resettable for HCDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
