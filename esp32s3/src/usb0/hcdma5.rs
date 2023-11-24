#[doc = "Register `HCDMA5` reader"]
pub type R = crate::R<HCDMA5_SPEC>;
#[doc = "Register `HCDMA5` writer"]
pub type W = crate::W<HCDMA5_SPEC>;
#[doc = "Field `H_DMAADDR5` reader - "]
pub type H_DMAADDR5_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR5` writer - "]
pub type H_DMAADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr5(&self) -> H_DMAADDR5_R {
        H_DMAADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA5")
            .field("h_dmaaddr5", &format_args!("{}", self.h_dmaaddr5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr5(&mut self) -> H_DMAADDR5_W<HCDMA5_SPEC> {
        H_DMAADDR5_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA5_SPEC;
impl crate::RegisterSpec for HCDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma5::R`](R) reader structure"]
impl crate::Readable for HCDMA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma5::W`](W) writer structure"]
impl crate::Writable for HCDMA5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA5 to value 0"]
impl crate::Resettable for HCDMA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
