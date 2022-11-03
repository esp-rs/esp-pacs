#[doc = "Register `SET_LINE_CODE_W1` reader"]
pub struct R(crate::R<SET_LINE_CODE_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_LINE_CODE_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_LINE_CODE_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_LINE_CODE_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_BCHAR_FORMAT` reader - The value of bCharFormat set by host through SET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_BCHAR_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_BPARITY_TYPE` reader - The value of bParityTpye set by host through SET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_BPARITY_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_BDATA_BITS` reader - The value of bDataBits set by host through SET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_BDATA_BITS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value of bCharFormat set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_bchar_format(&self) -> USB_SERIAL_JTAG_BCHAR_FORMAT_R {
        USB_SERIAL_JTAG_BCHAR_FORMAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_bparity_type(&self) -> USB_SERIAL_JTAG_BPARITY_TYPE_R {
        USB_SERIAL_JTAG_BPARITY_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_bdata_bits(&self) -> USB_SERIAL_JTAG_BDATA_BITS_R {
        USB_SERIAL_JTAG_BDATA_BITS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "W1 of SET_LINE_CODING command.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_line_code_w1](index.html) module"]
pub struct SET_LINE_CODE_W1_SPEC;
impl crate::RegisterSpec for SET_LINE_CODE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set_line_code_w1::R](R) reader structure"]
impl crate::Readable for SET_LINE_CODE_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SET_LINE_CODE_W1 to value 0"]
impl crate::Resettable for SET_LINE_CODE_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
