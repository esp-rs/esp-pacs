#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_HOLD_DELAY_RES` reader - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
pub type CS_HOLD_DELAY_RES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS_HOLD_DELAY_RES` writer - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
pub type CS_HOLD_DELAY_RES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL1_SPEC, u16, u16, 12, O>;
#[doc = "Field `CS_HOLD_DELAY` reader - SPI cs signal is delayed by spi clock cycles"]
pub type CS_HOLD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS_HOLD_DELAY` writer - SPI cs signal is delayed by spi clock cycles"]
pub type CS_HOLD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    pub fn cs_hold_delay_res(&self) -> CS_HOLD_DELAY_RES_R {
        CS_HOLD_DELAY_RES_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_delay_res(&mut self) -> CS_HOLD_DELAY_RES_W<16> {
        CS_HOLD_DELAY_RES_W::new(self)
    }
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<28> {
        CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x5fff_0000"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x5fff_0000;
}
