#[doc = "Register `SDIO_DATE` reader"]
pub type R = crate::R<SDIO_DATE_SPEC>;
#[doc = "Register `SDIO_DATE` writer"]
pub type W = crate::W<SDIO_DATE_SPEC>;
#[doc = "Field `SDIO_DATE` reader - sdio version date."]
pub type SDIO_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SDIO_DATE` writer - sdio version date."]
pub type SDIO_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sdio version date."]
    #[inline(always)]
    pub fn sdio_date(&self) -> SDIO_DATE_R {
        SDIO_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_DATE")
            .field("sdio_date", &format_args!("{}", self.sdio_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - sdio version date."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_date(&mut self) -> SDIO_DATE_W<SDIO_DATE_SPEC> {
        SDIO_DATE_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_DATE_SPEC;
impl crate::RegisterSpec for SDIO_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_date::R`](R) reader structure"]
impl crate::Readable for SDIO_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_date::W`](W) writer structure"]
impl crate::Writable for SDIO_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_DATE to value 0x0220_3150"]
impl crate::Resettable for SDIO_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0220_3150;
}
