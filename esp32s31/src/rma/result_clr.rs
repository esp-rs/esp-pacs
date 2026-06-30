#[doc = "Register `RESULT_CLR` writer"]
pub type W = crate::W<RESULT_CLR_SPEC>;
#[doc = "Field `REUSE_JTAG_CLR` writer - Set this bit to clear the reuse jtag state."]
pub type REUSE_JTAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REUSE_DOWNLOAD_CLR` writer - Set this bit to clear the reuse download state."]
pub type REUSE_DOWNLOAD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SPI_BOOT_CLR` writer - Set this bit to clear the force spi boot state."]
pub type FORCE_SPI_BOOT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESULT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the reuse jtag state."]
    #[inline(always)]
    pub fn reuse_jtag_clr(&mut self) -> REUSE_JTAG_CLR_W<'_, RESULT_CLR_SPEC> {
        REUSE_JTAG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the reuse download state."]
    #[inline(always)]
    pub fn reuse_download_clr(&mut self) -> REUSE_DOWNLOAD_CLR_W<'_, RESULT_CLR_SPEC> {
        REUSE_DOWNLOAD_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the force spi boot state."]
    #[inline(always)]
    pub fn force_spi_boot_clr(&mut self) -> FORCE_SPI_BOOT_CLR_W<'_, RESULT_CLR_SPEC> {
        FORCE_SPI_BOOT_CLR_W::new(self, 2)
    }
}
#[doc = "RMA clr result reg.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_CLR_SPEC;
impl crate::RegisterSpec for RESULT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`result_clr::W`](W) writer structure"]
impl crate::Writable for RESULT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_CLR to value 0"]
impl crate::Resettable for RESULT_CLR_SPEC {}
