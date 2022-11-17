#[doc = "Register `IN_EP0_ST` reader"]
pub struct R(crate::R<IN_EP0_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_EP0_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_EP0_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_EP0_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_IN_EP0_STATE` reader - State of IN Endpoint 0."]
pub type USB_SERIAL_JTAG_IN_EP0_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_IN_EP0_WR_ADDR` reader - Write data address of IN endpoint 0."]
pub type USB_SERIAL_JTAG_IN_EP0_WR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_IN_EP0_RD_ADDR` reader - Read data address of IN endpoint 0."]
pub type USB_SERIAL_JTAG_IN_EP0_RD_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - State of IN Endpoint 0."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_state(&self) -> USB_SERIAL_JTAG_IN_EP0_STATE_R {
        USB_SERIAL_JTAG_IN_EP0_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of IN endpoint 0."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_wr_addr(&self) -> USB_SERIAL_JTAG_IN_EP0_WR_ADDR_R {
        USB_SERIAL_JTAG_IN_EP0_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of IN endpoint 0."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_ep0_rd_addr(&self) -> USB_SERIAL_JTAG_IN_EP0_RD_ADDR_R {
        USB_SERIAL_JTAG_IN_EP0_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "Control IN endpoint status information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_ep0_st](index.html) module"]
pub struct IN_EP0_ST_SPEC;
impl crate::RegisterSpec for IN_EP0_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_ep0_st::R](R) reader structure"]
impl crate::Readable for IN_EP0_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_EP0_ST to value 0x01"]
impl crate::Resettable for IN_EP0_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
