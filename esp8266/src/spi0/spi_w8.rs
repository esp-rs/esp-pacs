#[doc = "Register `SPI_W8` reader"]
pub type R = crate::R<SPI_W8_SPEC>;
#[doc = "Register `SPI_W8` writer"]
pub type W = crate::W<SPI_W8_SPEC>;
#[doc = "Field `spi_w8` reader - the data inside the buffer of the SPI module, word 8"]
pub type SPI_W8_R = crate::FieldReader<u32>;
#[doc = "Field `spi_w8` writer - the data inside the buffer of the SPI module, word 8"]
pub type SPI_W8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 8"]
    #[inline(always)]
    pub fn spi_w8(&self) -> SPI_W8_R {
        SPI_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W8")
            .field("spi_w8", &format_args!("{}", self.spi_w8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi_w8(&mut self) -> SPI_W8_W<SPI_W8_SPEC, 0> {
        SPI_W8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "the data inside the buffer of the SPI module, word 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W8_SPEC;
impl crate::RegisterSpec for SPI_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w8::R`](R) reader structure"]
impl crate::Readable for SPI_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w8::W`](W) writer structure"]
impl crate::Writable for SPI_W8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_W8 to value 0"]
impl crate::Resettable for SPI_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
