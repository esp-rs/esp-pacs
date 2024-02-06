#[doc = "Register `HCDMA2` reader"]
pub type R = crate::R<HCDMA2_SPEC>;
#[doc = "Register `HCDMA2` writer"]
pub type W = crate::W<HCDMA2_SPEC>;
#[doc = "Field `H_DMAADDR2` reader - "]
pub type H_DMAADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR2` writer - "]
pub type H_DMAADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr2(&self) -> H_DMAADDR2_R {
        H_DMAADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA2")
            .field("h_dmaaddr2", &format_args!("{}", self.h_dmaaddr2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr2(&mut self) -> H_DMAADDR2_W<HCDMA2_SPEC> {
        H_DMAADDR2_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA2_SPEC;
impl crate::RegisterSpec for HCDMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma2::R`](R) reader structure"]
impl crate::Readable for HCDMA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma2::W`](W) writer structure"]
impl crate::Writable for HCDMA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA2 to value 0"]
impl crate::Resettable for HCDMA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
