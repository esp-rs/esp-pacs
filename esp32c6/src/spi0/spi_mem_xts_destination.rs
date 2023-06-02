#[doc = "Register `SPI_MEM_XTS_DESTINATION` reader"]
pub struct R(crate::R<SPI_MEM_XTS_DESTINATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_XTS_DESTINATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_XTS_DESTINATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_XTS_DESTINATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_XTS_DESTINATION` writer"]
pub struct W(crate::W<SPI_MEM_XTS_DESTINATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_XTS_DESTINATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_XTS_DESTINATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_XTS_DESTINATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_XTS_DESTINATION` reader - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SPI_XTS_DESTINATION_R = crate::BitReader;
#[doc = "Field `SPI_XTS_DESTINATION` writer - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
pub type SPI_XTS_DESTINATION_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_XTS_DESTINATION_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    pub fn spi_xts_destination(&self) -> SPI_XTS_DESTINATION_R {
        SPI_XTS_DESTINATION_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_XTS_DESTINATION")
            .field(
                "spi_xts_destination",
                &format_args!("{}", self.spi_xts_destination().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_DESTINATION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination parameter which will be used in manual encryption calculation. 0: flash(default), 1: psram(reserved). Only default value can be used."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_destination(&mut self) -> SPI_XTS_DESTINATION_W<0> {
        SPI_XTS_DESTINATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manual Encryption destination register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_xts_destination](index.html) module"]
pub struct SPI_MEM_XTS_DESTINATION_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_xts_destination::R](R) reader structure"]
impl crate::Readable for SPI_MEM_XTS_DESTINATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_xts_destination::W](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_DESTINATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_DESTINATION to value 0"]
impl crate::Resettable for SPI_MEM_XTS_DESTINATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
