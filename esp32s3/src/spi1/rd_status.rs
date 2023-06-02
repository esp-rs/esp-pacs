#[doc = "Register `RD_STATUS` reader"]
pub struct R(crate::R<RD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_STATUS` writer"]
pub struct W(crate::W<RD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_STATUS_SPEC>;
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
impl From<crate::W<RD_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - The value is stored when set SPI_MEM_FLASH_RDSR bit and SPI_MEM_FLASH_RES bit."]
pub type STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STATUS` writer - The value is stored when set SPI_MEM_FLASH_RDSR bit and SPI_MEM_FLASH_RES bit."]
pub type STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, RD_STATUS_SPEC, 16, O, u16, u16>;
#[doc = "Field `WB_MODE` reader - Mode bits in the flash fast read mode it is combined with SPI_MEM_FASTRD_MODE bit."]
pub type WB_MODE_R = crate::FieldReader;
#[doc = "Field `WB_MODE` writer - Mode bits in the flash fast read mode it is combined with SPI_MEM_FASTRD_MODE bit."]
pub type WB_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, RD_STATUS_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:15 - The value is stored when set SPI_MEM_FLASH_RDSR bit and SPI_MEM_FLASH_RES bit."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with SPI_MEM_FASTRD_MODE bit."]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_STATUS")
            .field("status", &format_args!("{}", self.status().bits()))
            .field("wb_mode", &format_args!("{}", self.wb_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value is stored when set SPI_MEM_FLASH_RDSR bit and SPI_MEM_FLASH_RES bit."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with SPI_MEM_FASTRD_MODE bit."]
    #[inline(always)]
    #[must_use]
    pub fn wb_mode(&mut self) -> WB_MODE_W<16> {
        WB_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 read control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_status](index.html) module"]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_status::R](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_status::W](W) writer structure"]
impl crate::Writable for RD_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
