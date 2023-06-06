#[doc = "Register `SPI_MEM_REGISTERRND_ECO_LOW` reader"]
pub struct R(crate::R<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_REGISTERRND_ECO_LOW` reader - ECO low register"]
pub type SPI_MEM_REGISTERRND_ECO_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn spi_mem_registerrnd_eco_low(&self) -> SPI_MEM_REGISTERRND_ECO_LOW_R {
        SPI_MEM_REGISTERRND_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_REGISTERRND_ECO_LOW")
            .field(
                "spi_mem_registerrnd_eco_low",
                &format_args!("{}", self.spi_mem_registerrnd_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_REGISTERRND_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI ECO low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_registerrnd_eco_low](index.html) module"]
pub struct SPI_MEM_REGISTERRND_ECO_LOW_SPEC;
impl crate::RegisterSpec for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_registerrnd_eco_low::R](R) reader structure"]
impl crate::Readable for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_REGISTERRND_ECO_LOW to value 0x037c"]
impl crate::Resettable for SPI_MEM_REGISTERRND_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0x037c;
}
