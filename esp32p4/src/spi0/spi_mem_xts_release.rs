#[doc = "Register `SPI_MEM_XTS_RELEASE` writer"]
pub type W = crate::W<SPI_MEM_XTS_RELEASE_SPEC>;
#[doc = "Field `SPI_XTS_RELEASE` writer - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
pub type SPI_XTS_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_RELEASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_release(&mut self) -> SPI_XTS_RELEASE_W<SPI_MEM_XTS_RELEASE_SPEC> {
        SPI_XTS_RELEASE_W::new(self, 0)
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
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_RELEASE_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_release::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_RELEASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_RELEASE to value 0"]
impl crate::Resettable for SPI_MEM_XTS_RELEASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
