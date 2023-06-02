#[doc = "Register `SPI_EXT2` reader"]
pub struct R(crate::R<SPI_EXT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_EXT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_EXT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_EXT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_EXT2` writer"]
pub struct W(crate::W<SPI_EXT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_EXT2_SPEC>;
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
impl From<crate::W<SPI_EXT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_EXT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `st` reader - "]
pub type ST_R = crate::FieldReader;
#[doc = "Field `st` writer - "]
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_EXT2_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_EXT2")
            .field("st", &format_args!("{}", self.st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_EXT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ext2](index.html) module"]
pub struct SPI_EXT2_SPEC;
impl crate::RegisterSpec for SPI_EXT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ext2::R](R) reader structure"]
impl crate::Readable for SPI_EXT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ext2::W](W) writer structure"]
impl crate::Writable for SPI_EXT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_EXT2 to value 0"]
impl crate::Resettable for SPI_EXT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
