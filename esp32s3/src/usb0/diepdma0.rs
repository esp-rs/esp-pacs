#[doc = "Register `DIEPDMA0` reader"]
pub type R = crate::R<DIEPDMA0_SPEC>;
#[doc = "Register `DIEPDMA0` writer"]
pub type W = crate::W<DIEPDMA0_SPEC>;
#[doc = "Field `D_DMAADDR0` reader - "]
pub type D_DMAADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `D_DMAADDR0` writer - "]
pub type D_DMAADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr0(&self) -> D_DMAADDR0_R {
        D_DMAADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA0")
            .field("d_dmaaddr0", &format_args!("{}", self.d_dmaaddr0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_dmaaddr0(&mut self) -> D_DMAADDR0_W<DIEPDMA0_SPEC> {
        D_DMAADDR0_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA0_SPEC;
impl crate::RegisterSpec for DIEPDMA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma0::R`](R) reader structure"]
impl crate::Readable for DIEPDMA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma0::W`](W) writer structure"]
impl crate::Writable for DIEPDMA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPDMA0 to value 0"]
impl crate::Resettable for DIEPDMA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
