#[doc = "Register `SPI_W7` reader"]
pub type R = crate::R<SPI_W7_SPEC>;
#[doc = "Register `SPI_W7` writer"]
pub type W = crate::W<SPI_W7_SPEC>;
#[doc = "Field `SPI_BUF7` reader - 32-bit data buffer 0."]
pub type SPI_BUF7_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_BUF7` writer - 32-bit data buffer 0."]
pub type SPI_BUF7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf7(&self) -> SPI_BUF7_R {
        SPI_BUF7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W7")
            .field("spi_buf7", &self.spi_buf7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data buffer 0."]
    #[inline(always)]
    pub fn spi_buf7(&mut self) -> SPI_BUF7_W<SPI_W7_SPEC> {
        SPI_BUF7_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer7\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_w7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_w7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W7_SPEC;
impl crate::RegisterSpec for SPI_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w7::R`](R) reader structure"]
impl crate::Readable for SPI_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w7::W`](W) writer structure"]
impl crate::Writable for SPI_W7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_W7 to value 0"]
impl crate::Resettable for SPI_W7_SPEC {}
