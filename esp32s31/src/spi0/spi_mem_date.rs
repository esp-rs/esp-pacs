#[doc = "Register `SPI_MEM_DATE` reader"]
pub type R = crate::R<SPI_MEM_DATE_SPEC>;
#[doc = "Register `SPI_MEM_DATE` writer"]
pub type W = crate::W<SPI_MEM_DATE_SPEC>;
#[doc = "Field `SPI_MEM_DATE` reader - SPI0 register version."]
pub type SPI_MEM_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_DATE` writer - SPI0 register version."]
pub type SPI_MEM_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - SPI0 register version."]
    #[inline(always)]
    pub fn spi_mem_date(&self) -> SPI_MEM_DATE_R {
        SPI_MEM_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DATE")
            .field("spi_mem_date", &self.spi_mem_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - SPI0 register version."]
    #[inline(always)]
    pub fn spi_mem_date(&mut self) -> SPI_MEM_DATE_W<'_, SPI_MEM_DATE_SPEC> {
        SPI_MEM_DATE_W::new(self, 0)
    }
}
#[doc = "SPI0 version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DATE_SPEC;
impl crate::RegisterSpec for SPI_MEM_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_date::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_date::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_DATE to value 0x0250_9030"]
impl crate::Resettable for SPI_MEM_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0250_9030;
}
