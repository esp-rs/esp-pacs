#[doc = "Register `DOEPDMA3` reader"]
pub type R = crate::R<DOEPDMA3_SPEC>;
#[doc = "Register `DOEPDMA3` writer"]
pub type W = crate::W<DOEPDMA3_SPEC>;
#[doc = "Field `DMAADDR3` reader - "]
pub type DMAADDR3_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR3` writer - "]
pub type DMAADDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr3(&self) -> DMAADDR3_R {
        DMAADDR3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA3")
            .field("dmaaddr3", &format_args!("{}", self.dmaaddr3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr3(&mut self) -> DMAADDR3_W<DOEPDMA3_SPEC> {
        DMAADDR3_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA3_SPEC;
impl crate::RegisterSpec for DOEPDMA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma3::R`](R) reader structure"]
impl crate::Readable for DOEPDMA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma3::W`](W) writer structure"]
impl crate::Writable for DOEPDMA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMA3 to value 0"]
impl crate::Resettable for DOEPDMA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
