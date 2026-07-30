#[doc = "Register `OUT_EP2_ST` reader"]
pub type R = crate::R<OUT_EP2_ST_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_STATE` reader - State of OUT Endpoint 2."]
pub type USB_SERIAL_JTAG_OUT_EP2_STATE_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_WR_ADDR` reader - Write data address of OUT endpoint 2. When USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_SERIAL_JTAG_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
pub type USB_SERIAL_JTAG_OUT_EP2_WR_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_RD_ADDR` reader - Read data address of OUT endpoint 2."]
pub type USB_SERIAL_JTAG_OUT_EP2_RD_ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 2."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_state(&self) -> USB_SERIAL_JTAG_OUT_EP2_STATE_R {
        USB_SERIAL_JTAG_OUT_EP2_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 2. When USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_SERIAL_JTAG_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EP2_ST")
            .field(
                "usb_serial_jtag_out_ep2_state",
                &self.usb_serial_jtag_out_ep2_state(),
            )
            .field(
                "usb_serial_jtag_out_ep2_wr_addr",
                &self.usb_serial_jtag_out_ep2_wr_addr(),
            )
            .field(
                "usb_serial_jtag_out_ep2_rd_addr",
                &self.usb_serial_jtag_out_ep2_rd_addr(),
            )
            .finish()
    }
}
#[doc = "JTAG OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ep2_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EP2_ST_SPEC;
impl crate::RegisterSpec for OUT_EP2_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ep2_st::R`](R) reader structure"]
impl crate::Readable for OUT_EP2_ST_SPEC {}
#[doc = "`reset()` method sets OUT_EP2_ST to value 0"]
impl crate::Resettable for OUT_EP2_ST_SPEC {}
