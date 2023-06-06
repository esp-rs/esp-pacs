#[doc = "Register `SPI_W9` reader"]
pub struct R(crate::R<SPI_W9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W9` writer"]
pub struct W(crate::W<SPI_W9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W9_SPEC>;
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
impl From<crate::W<SPI_W9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_w9` reader - the data inside the buffer of the SPI module, word 9"]
pub type SPI_W9_R = crate::FieldReader<u32>;
#[doc = "Field `spi_w9` writer - the data inside the buffer of the SPI module, word 9"]
pub type SPI_W9_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_W9_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 9"]
    #[inline(always)]
    pub fn spi_w9(&self) -> SPI_W9_R {
        SPI_W9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W9")
            .field("spi_w9", &format_args!("{}", self.spi_w9().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_W9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 9"]
    #[inline(always)]
    #[must_use]
    pub fn spi_w9(&mut self) -> SPI_W9_W<0> {
        SPI_W9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the data inside the buffer of the SPI module, word 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w9](index.html) module"]
pub struct SPI_W9_SPEC;
impl crate::RegisterSpec for SPI_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w9::R](R) reader structure"]
impl crate::Readable for SPI_W9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w9::W](W) writer structure"]
impl crate::Writable for SPI_W9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_W9 to value 0"]
impl crate::Resettable for SPI_W9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
