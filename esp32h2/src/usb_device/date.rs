#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_DATE` reader - register version."]
pub type USB_SERIAL_JTAG_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_SERIAL_JTAG_DATE` writer - register version."]
pub type USB_SERIAL_JTAG_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - register version."]
    #[inline(always)]
    pub fn usb_serial_jtag_date(&self) -> USB_SERIAL_JTAG_DATE_R {
        USB_SERIAL_JTAG_DATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - register version."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_date(&mut self) -> USB_SERIAL_JTAG_DATE_W<0> {
        USB_SERIAL_JTAG_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0210_9220"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_9220;
}
