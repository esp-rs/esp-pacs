#[doc = "Register `SPI_INTERRUPT_TYPE` reader"]
pub struct R(crate::R<SPI_INTERRUPT_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_INTERRUPT_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_INTERRUPT_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_INTERRUPT_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `spi0` reader - SPI0 interrupt"]
pub type SPI0_R = crate::BitReader;
#[doc = "Field `spi1` reader - SPI1 interrupt"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `i2s` reader - I2S interrupt"]
pub type I2S_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - SPI0 interrupt"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI1 interrupt"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S interrupt"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INTERRUPT_TYPE")
            .field("spi0", &format_args!("{}", self.spi0().bit()))
            .field("spi1", &format_args!("{}", self.spi1().bit()))
            .field("i2s", &format_args!("{}", self.i2s().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_INTERRUPT_TYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI interrupt type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_interrupt_type](index.html) module"]
pub struct SPI_INTERRUPT_TYPE_SPEC;
impl crate::RegisterSpec for SPI_INTERRUPT_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_interrupt_type::R](R) reader structure"]
impl crate::Readable for SPI_INTERRUPT_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_INTERRUPT_TYPE to value 0"]
impl crate::Resettable for SPI_INTERRUPT_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
