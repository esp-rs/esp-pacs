#[doc = "Register `SPI_W8` reader"]
pub type R = crate::R<SPI_W8_SPEC>;
#[doc = "Register `SPI_W8` writer"]
pub type W = crate::W<SPI_W8_SPEC>;
#[doc = "Field `SPI_BUF8` reader - 32-bit data buffer 0."]
pub type SPI_BUF8_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_BUF8` writer - 32-bit data buffer 0."]
pub type SPI_BUF8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf8(&self) -> SPI_BUF8_R {
        SPI_BUF8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W8")
            .field("spi_buf8", &self.spi_buf8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf8(&mut self) -> SPI_BUF8_W<SPI_W8_SPEC> {
        SPI_BUF8_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer8\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_w8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_w8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W8_SPEC;
impl crate::RegisterSpec for SPI_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w8::R`](R) reader structure"]
impl crate::Readable for SPI_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w8::W`](W) writer structure"]
impl crate::Writable for SPI_W8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_W8 to value 0"]
impl crate::Resettable for SPI_W8_SPEC {}
