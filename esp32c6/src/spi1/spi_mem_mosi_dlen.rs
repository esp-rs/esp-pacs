#[doc = "Register `SPI_MEM_MOSI_DLEN` reader"]
pub type R = crate::R<SPI_MEM_MOSI_DLEN_SPEC>;
#[doc = "Register `SPI_MEM_MOSI_DLEN` writer"]
pub type W = crate::W<SPI_MEM_MOSI_DLEN_SPEC>;
#[doc = "Field `SPI_MEM_USR_MOSI_DBITLEN` reader - The length in bits of write-data. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_MOSI_DBITLEN_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_USR_MOSI_DBITLEN` writer - The length in bits of write-data. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_MOSI_DBITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_mosi_dbitlen(&self) -> SPI_MEM_USR_MOSI_DBITLEN_R {
        SPI_MEM_USR_MOSI_DBITLEN_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MOSI_DLEN")
            .field(
                "spi_mem_usr_mosi_dbitlen",
                &format_args!("{}", self.spi_mem_usr_mosi_dbitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_MOSI_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_mosi_dbitlen(
        &mut self,
    ) -> SPI_MEM_USR_MOSI_DBITLEN_W<SPI_MEM_MOSI_DLEN_SPEC> {
        SPI_MEM_USR_MOSI_DBITLEN_W::new(self, 0)
    }
}
#[doc = "SPI1 send data bit length control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mosi_dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mosi_dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MOSI_DLEN_SPEC;
impl crate::RegisterSpec for SPI_MEM_MOSI_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_mosi_dlen::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MOSI_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_mosi_dlen::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MOSI_DLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MOSI_DLEN to value 0"]
impl crate::Resettable for SPI_MEM_MOSI_DLEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
