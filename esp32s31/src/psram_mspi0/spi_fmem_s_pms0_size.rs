#[doc = "Register `SPI_FMEM_S_PMS0_SIZE` reader"]
pub type R = crate::R<SPI_FMEM_S_PMS0_SIZE_SPEC>;
#[doc = "Register `SPI_FMEM_S_PMS0_SIZE` writer"]
pub type W = crate::W<SPI_FMEM_S_PMS0_SIZE_SPEC>;
#[doc = "Field `SPI_FMEM_S_PMS0_SIZE` reader - "]
pub type SPI_FMEM_S_PMS0_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_FMEM_S_PMS0_SIZE` writer - "]
pub type SPI_FMEM_S_PMS0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn spi_fmem_s_pms0_size(&self) -> SPI_FMEM_S_PMS0_SIZE_R {
        SPI_FMEM_S_PMS0_SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FMEM_S_PMS0_SIZE")
            .field("spi_fmem_s_pms0_size", &self.spi_fmem_s_pms0_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn spi_fmem_s_pms0_size(
        &mut self,
    ) -> SPI_FMEM_S_PMS0_SIZE_W<'_, SPI_FMEM_S_PMS0_SIZE_SPEC> {
        SPI_FMEM_S_PMS0_SIZE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms0_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms0_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_FMEM_S_PMS0_SIZE_SPEC;
impl crate::RegisterSpec for SPI_FMEM_S_PMS0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fmem_s_pms0_size::R`](R) reader structure"]
impl crate::Readable for SPI_FMEM_S_PMS0_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_fmem_s_pms0_size::W`](W) writer structure"]
impl crate::Writable for SPI_FMEM_S_PMS0_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_FMEM_S_PMS0_SIZE to value 0x1000"]
impl crate::Resettable for SPI_FMEM_S_PMS0_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
