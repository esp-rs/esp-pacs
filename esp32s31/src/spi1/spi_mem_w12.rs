#[doc = "Register `SPI_MEM_W12` reader"]
pub type R = crate::R<SPI_MEM_W12_SPEC>;
#[doc = "Register `SPI_MEM_W12` writer"]
pub type W = crate::W<SPI_MEM_W12_SPEC>;
#[doc = "Field `SPI_MEM_BUF12` reader - data buffer"]
pub type SPI_MEM_BUF12_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_BUF12` writer - data buffer"]
pub type SPI_MEM_BUF12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_mem_buf12(&self) -> SPI_MEM_BUF12_R {
        SPI_MEM_BUF12_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_W12")
            .field("spi_mem_buf12", &self.spi_mem_buf12())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_mem_buf12(&mut self) -> SPI_MEM_BUF12_W<'_, SPI_MEM_W12_SPEC> {
        SPI_MEM_BUF12_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer12\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_w12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_w12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_W12_SPEC;
impl crate::RegisterSpec for SPI_MEM_W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_w12::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_W12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_w12::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_W12_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_W12 to value 0"]
impl crate::Resettable for SPI_MEM_W12_SPEC {}
