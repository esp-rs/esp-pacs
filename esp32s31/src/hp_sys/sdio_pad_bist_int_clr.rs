#[doc = "Register `SDIO_PAD_BIST_INT_CLR` writer"]
pub type W = crate::W<SDIO_PAD_BIST_INT_CLR_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_OK_INT_CLR` writer - Write 1 to clr sdio pad bist ok interrupt"]
pub type SDIO_PAD_BIST_OK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_FAIL_INT_CLR` writer - Write 1 to clr sdio pad bist fail interrupt"]
pub type SDIO_PAD_BIST_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_PAD_BIST_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clr sdio pad bist ok interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_ok_int_clr(
        &mut self,
    ) -> SDIO_PAD_BIST_OK_INT_CLR_W<'_, SDIO_PAD_BIST_INT_CLR_SPEC> {
        SDIO_PAD_BIST_OK_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clr sdio pad bist fail interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_fail_int_clr(
        &mut self,
    ) -> SDIO_PAD_BIST_FAIL_INT_CLR_W<'_, SDIO_PAD_BIST_INT_CLR_SPEC> {
        SDIO_PAD_BIST_FAIL_INT_CLR_W::new(self, 1)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_INT_CLR_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sdio_pad_bist_int_clr::W`](W) writer structure"]
impl crate::Writable for SDIO_PAD_BIST_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_PAD_BIST_INT_CLR to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_INT_CLR_SPEC {}
