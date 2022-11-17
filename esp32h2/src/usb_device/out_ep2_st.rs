#[doc = "Register `OUT_EP2_ST` reader"]
pub struct R(crate::R<OUT_EP2_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EP2_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EP2_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EP2_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_STATE` reader - State of OUT Endpoint 2."]
pub type USB_SERIAL_JTAG_OUT_EP2_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_WR_ADDR` reader - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
pub type USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_RD_ADDR` reader - Read data address of OUT endpoint 2."]
pub type USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 2."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_state(&self) -> USB_SERIAL_JTAG_OUT_EP2_STATE_R {
        USB_SERIAL_JTAG_OUT_EP2_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_wr_addr(&self) -> USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_R {
        USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 2."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_rd_addr(&self) -> USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_R {
        USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "JTAG OUT endpoint status information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ep2_st](index.html) module"]
pub struct OUT_EP2_ST_SPEC;
impl crate::RegisterSpec for OUT_EP2_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_ep2_st::R](R) reader structure"]
impl crate::Readable for OUT_EP2_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_EP2_ST to value 0"]
impl crate::Resettable for OUT_EP2_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
