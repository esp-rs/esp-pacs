///Register `SPI_FMEM_PMS%s_SIZE` reader
pub type R = crate::R<SPI_FMEM_PMS_SIZE_SPEC>;
///Register `SPI_FMEM_PMS%s_SIZE` writer
pub type W = crate::W<SPI_FMEM_PMS_SIZE_SPEC>;
///Field `SPI_FMEM_PMS_SIZE` reader - SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
pub type SPI_FMEM_PMS_SIZE_R = crate::FieldReader<u16>;
///Field `SPI_FMEM_PMS_SIZE` writer - SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
pub type SPI_FMEM_PMS_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    pub fn spi_fmem_pms_size(&self) -> SPI_FMEM_PMS_SIZE_R {
        SPI_FMEM_PMS_SIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FMEM_PMS_SIZE")
            .field("spi_fmem_pms_size", &self.spi_fmem_pms_size())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - SPI1 flash PMS section %s address region is (SPI_FMEM_PMS%s_ADDR_S, SPI_FMEM_PMS%s_ADDR_S + SPI_FMEM_PMS%s_SIZE)
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_pms_size(&mut self) -> SPI_FMEM_PMS_SIZE_W<SPI_FMEM_PMS_SIZE_SPEC> {
        SPI_FMEM_PMS_SIZE_W::new(self, 0)
    }
}
/**SPI1 flash PMS section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI_FMEM_PMS_SIZE_SPEC;
impl crate::RegisterSpec for SPI_FMEM_PMS_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi_fmem_pms_size::R`](R) reader structure
impl crate::Readable for SPI_FMEM_PMS_SIZE_SPEC {}
///`write(|w| ..)` method takes [`spi_fmem_pms_size::W`](W) writer structure
impl crate::Writable for SPI_FMEM_PMS_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_FMEM_PMS%s_SIZE to value 0x1000
impl crate::Resettable for SPI_FMEM_PMS_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
