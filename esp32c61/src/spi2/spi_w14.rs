#[doc = "Register `SPI_W14` reader"]
pub type R = crate::R<SPI_W14_SPEC>;
#[doc = "Register `SPI_W14` writer"]
pub type W = crate::W<SPI_W14_SPEC>;
#[doc = "Field `SPI_BUF14` reader - 32-bit data buffer 0."]
pub type SPI_BUF14_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_BUF14` writer - 32-bit data buffer 0."]
pub type SPI_BUF14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf14(&self) -> SPI_BUF14_R {
        SPI_BUF14_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W14")
            .field("spi_buf14", &self.spi_buf14())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf14(&mut self) -> SPI_BUF14_W<SPI_W14_SPEC> {
        SPI_BUF14_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer14\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_w14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_w14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W14_SPEC;
impl crate::RegisterSpec for SPI_W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w14::R`](R) reader structure"]
impl crate::Readable for SPI_W14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w14::W`](W) writer structure"]
impl crate::Writable for SPI_W14_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_W14 to value 0"]
impl crate::Resettable for SPI_W14_SPEC {}
