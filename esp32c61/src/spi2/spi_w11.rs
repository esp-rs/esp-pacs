#[doc = "Register `SPI_W11` reader"]
pub type R = crate::R<SPI_W11_SPEC>;
#[doc = "Register `SPI_W11` writer"]
pub type W = crate::W<SPI_W11_SPEC>;
#[doc = "Field `SPI_BUF11` reader - 32-bit data buffer 0."]
pub type SPI_BUF11_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_BUF11` writer - 32-bit data buffer 0."]
pub type SPI_BUF11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf11(&self) -> SPI_BUF11_R {
        SPI_BUF11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W11")
            .field("spi_buf11", &self.spi_buf11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf11(&mut self) -> SPI_BUF11_W<SPI_W11_SPEC> {
        SPI_BUF11_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer11\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_w11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_w11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W11_SPEC;
impl crate::RegisterSpec for SPI_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w11::R`](R) reader structure"]
impl crate::Readable for SPI_W11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w11::W`](W) writer structure"]
impl crate::Writable for SPI_W11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_W11 to value 0"]
impl crate::Resettable for SPI_W11_SPEC {}
