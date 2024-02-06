#[doc = "Register `DIEPDMA2` reader"]
pub type R = crate::R<DIEPDMA2_SPEC>;
#[doc = "Register `DIEPDMA2` writer"]
pub type W = crate::W<DIEPDMA2_SPEC>;
#[doc = "Field `D_DMAADDR2` reader - "]
pub type D_DMAADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `D_DMAADDR2` writer - "]
pub type D_DMAADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr2(&self) -> D_DMAADDR2_R {
        D_DMAADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA2")
            .field("d_dmaaddr2", &format_args!("{}", self.d_dmaaddr2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_dmaaddr2(&mut self) -> D_DMAADDR2_W<DIEPDMA2_SPEC> {
        D_DMAADDR2_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA2_SPEC;
impl crate::RegisterSpec for DIEPDMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma2::R`](R) reader structure"]
impl crate::Readable for DIEPDMA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma2::W`](W) writer structure"]
impl crate::Writable for DIEPDMA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPDMA2 to value 0"]
impl crate::Resettable for DIEPDMA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
