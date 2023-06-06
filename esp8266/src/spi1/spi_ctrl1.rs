#[doc = "Register `SPI_CTRL1` reader"]
pub struct R(crate::R<SPI_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL1` writer"]
pub struct W(crate::W<SPI_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL1_SPEC>;
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
impl From<crate::W<SPI_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `status` reader - In the slave mode, it is the status for master to read out."]
pub type STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `status` writer - In the slave mode, it is the status for master to read out."]
pub type STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL1_SPEC, 16, O, u16>;
#[doc = "Field `wb_mode` reader - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
pub type WB_MODE_R = crate::FieldReader;
#[doc = "Field `wb_mode` writer - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
pub type WB_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL1_SPEC, 8, O>;
#[doc = "Field `status_ext` reader - In the slave mode,it is the status for master to read out."]
pub type STATUS_EXT_R = crate::FieldReader;
#[doc = "Field `status_ext` writer - In the slave mode,it is the status for master to read out."]
pub type STATUS_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_CTRL1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:15 - In the slave mode, it is the status for master to read out."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode,it is the status for master to read out."]
    #[inline(always)]
    pub fn status_ext(&self) -> STATUS_EXT_R {
        STATUS_EXT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL1")
            .field("status", &format_args!("{}", self.status().bits()))
            .field("wb_mode", &format_args!("{}", self.wb_mode().bits()))
            .field("status_ext", &format_args!("{}", self.status_ext().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - In the slave mode, it is the status for master to read out."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
    #[inline(always)]
    #[must_use]
    pub fn wb_mode(&mut self) -> WB_MODE_W<16> {
        WB_MODE_W::new(self)
    }
    #[doc = "Bits 24:31 - In the slave mode,it is the status for master to read out."]
    #[inline(always)]
    #[must_use]
    pub fn status_ext(&mut self) -> STATUS_EXT_W<24> {
        STATUS_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl1](index.html) module"]
pub struct SPI_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl1::R](R) reader structure"]
impl crate::Readable for SPI_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl1::W](W) writer structure"]
impl crate::Writable for SPI_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CTRL1 to value 0"]
impl crate::Resettable for SPI_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
