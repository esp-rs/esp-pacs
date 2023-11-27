#[doc = "Register `OUT_EP1_ST` reader"]
pub type R = crate::R<OUT_EP1_ST_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_STATE` reader - State of OUT Endpoint 1."]
pub type USB_SERIAL_JTAG_OUT_EP1_STATE_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_WR_ADDR` reader - Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1."]
pub type USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_RD_ADDR` reader - Read data address of OUT endpoint 1."]
pub type USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT` reader - Data count in OUT endpoint 1 when one packet is received."]
pub type USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 1."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_state(&self) -> USB_SERIAL_JTAG_OUT_EP1_STATE_R {
        USB_SERIAL_JTAG_OUT_EP1_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_wr_addr(&self) -> USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_R {
        USB_SERIAL_JTAG_OUT_EP1_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 1."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_rd_addr(&self) -> USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_R {
        USB_SERIAL_JTAG_OUT_EP1_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Data count in OUT endpoint 1 when one packet is received."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_rec_data_cnt(&self) -> USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_R {
        USB_SERIAL_JTAG_OUT_EP1_REC_DATA_CNT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EP1_ST")
            .field(
                "usb_serial_jtag_out_ep1_state",
                &format_args!("{}", self.usb_serial_jtag_out_ep1_state().bits()),
            )
            .field(
                "usb_serial_jtag_out_ep1_wr_addr",
                &format_args!("{}", self.usb_serial_jtag_out_ep1_wr_addr().bits()),
            )
            .field(
                "usb_serial_jtag_out_ep1_rd_addr",
                &format_args!("{}", self.usb_serial_jtag_out_ep1_rd_addr().bits()),
            )
            .field(
                "usb_serial_jtag_out_ep1_rec_data_cnt",
                &format_args!("{}", self.usb_serial_jtag_out_ep1_rec_data_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_EP1_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "CDC-ACM OUT endpoint status information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ep1_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EP1_ST_SPEC;
impl crate::RegisterSpec for OUT_EP1_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ep1_st::R`](R) reader structure"]
impl crate::Readable for OUT_EP1_ST_SPEC {}
#[doc = "`reset()` method sets OUT_EP1_ST to value 0"]
impl crate::Resettable for OUT_EP1_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
