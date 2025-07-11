#[doc = "Register `SPI_MS_DLEN` reader"]
pub type R = crate::R<SPI_MS_DLEN_SPEC>;
#[doc = "Register `SPI_MS_DLEN` writer"]
pub type W = crate::W<SPI_MS_DLEN_SPEC>;
#[doc = "Field `SPI_MS_DATA_BITLEN` reader - Configures the data bit length of SPI transfer in DMA-controlled master transfer or in CPU-controlled master transfer. Or configures the bit length of SPI RX transfer in DMA-controlled slave transfer. This value shall be (expected bit_num - 1). Can be configured in CONF state."]
pub type SPI_MS_DATA_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MS_DATA_BITLEN` writer - Configures the data bit length of SPI transfer in DMA-controlled master transfer or in CPU-controlled master transfer. Or configures the bit length of SPI RX transfer in DMA-controlled slave transfer. This value shall be (expected bit_num - 1). Can be configured in CONF state."]
pub type SPI_MS_DATA_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Configures the data bit length of SPI transfer in DMA-controlled master transfer or in CPU-controlled master transfer. Or configures the bit length of SPI RX transfer in DMA-controlled slave transfer. This value shall be (expected bit_num - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&self) -> SPI_MS_DATA_BITLEN_R {
        SPI_MS_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MS_DLEN")
            .field("spi_ms_data_bitlen", &self.spi_ms_data_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Configures the data bit length of SPI transfer in DMA-controlled master transfer or in CPU-controlled master transfer. Or configures the bit length of SPI RX transfer in DMA-controlled slave transfer. This value shall be (expected bit_num - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ms_data_bitlen(&mut self) -> SPI_MS_DATA_BITLEN_W<SPI_MS_DLEN_SPEC> {
        SPI_MS_DATA_BITLEN_W::new(self, 0)
    }
}
#[doc = "SPI data bit length control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ms_dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ms_dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MS_DLEN_SPEC;
impl crate::RegisterSpec for SPI_MS_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ms_dlen::R`](R) reader structure"]
impl crate::Readable for SPI_MS_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ms_dlen::W`](W) writer structure"]
impl crate::Writable for SPI_MS_DLEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MS_DLEN to value 0"]
impl crate::Resettable for SPI_MS_DLEN_SPEC {}
