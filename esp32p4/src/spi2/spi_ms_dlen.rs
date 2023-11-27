#[doc = "Register `SPI_MS_DLEN` reader"]
pub type R = crate::R<SPI_MS_DLEN_SPEC>;
#[doc = "Register `SPI_MS_DLEN` writer"]
pub type W = crate::W<SPI_MS_DLEN_SPEC>;
#[doc = "Field `SPI_MS_DATA_BITLEN` reader - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_MS_DATA_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MS_DATA_BITLEN` writer - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_MS_DATA_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&self) -> SPI_MS_DATA_BITLEN_R {
        SPI_MS_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MS_DLEN")
            .field(
                "spi_ms_data_bitlen",
                &format_args!("{}", self.spi_ms_data_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MS_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:17 - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ms_data_bitlen(&mut self) -> SPI_MS_DATA_BITLEN_W<SPI_MS_DLEN_SPEC> {
        SPI_MS_DATA_BITLEN_W::new(self, 0)
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
#[doc = "SPI data bit length control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ms_dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ms_dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MS_DLEN_SPEC;
impl crate::RegisterSpec for SPI_MS_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ms_dlen::R`](R) reader structure"]
impl crate::Readable for SPI_MS_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ms_dlen::W`](W) writer structure"]
impl crate::Writable for SPI_MS_DLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MS_DLEN to value 0"]
impl crate::Resettable for SPI_MS_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
