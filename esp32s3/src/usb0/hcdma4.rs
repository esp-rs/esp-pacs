#[doc = "Register `HCDMA4` reader"]
pub type R = crate::R<HCDMA4_SPEC>;
#[doc = "Register `HCDMA4` writer"]
pub type W = crate::W<HCDMA4_SPEC>;
#[doc = "Field `H_DMAADDR4` reader - "]
pub type H_DMAADDR4_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR4` writer - "]
pub type H_DMAADDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr4(&self) -> H_DMAADDR4_R {
        H_DMAADDR4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA4")
            .field("h_dmaaddr4", &format_args!("{}", self.h_dmaaddr4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr4(&mut self) -> H_DMAADDR4_W<HCDMA4_SPEC> {
        H_DMAADDR4_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA4_SPEC;
impl crate::RegisterSpec for HCDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma4::R`](R) reader structure"]
impl crate::Readable for HCDMA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma4::W`](W) writer structure"]
impl crate::Writable for HCDMA4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA4 to value 0"]
impl crate::Resettable for HCDMA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
