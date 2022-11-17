#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SOF_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST` reader - The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_st(&self) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_R {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_st(&self) -> USB_SERIAL_JTAG_SOF_INT_ST_R {
        USB_SERIAL_JTAG_SOF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_st(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_R {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_st(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_R {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_st(&self) -> USB_SERIAL_JTAG_PID_ERR_INT_ST_R {
        USB_SERIAL_JTAG_PID_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_st(&self) -> USB_SERIAL_JTAG_CRC5_ERR_INT_ST_R {
        USB_SERIAL_JTAG_CRC5_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_st(&self) -> USB_SERIAL_JTAG_CRC16_ERR_INT_ST_R {
        USB_SERIAL_JTAG_CRC16_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_st(&self) -> USB_SERIAL_JTAG_STUFF_ERR_INT_ST_R {
        USB_SERIAL_JTAG_STUFF_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_st(
        &self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_R {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_st(&self) -> USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_R {
        USB_SERIAL_JTAG_USB_BUS_RESET_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_st(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_R {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_st(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_R {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_st(&self) -> USB_SERIAL_JTAG_RTS_CHG_INT_ST_R {
        USB_SERIAL_JTAG_RTS_CHG_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_st(&self) -> USB_SERIAL_JTAG_DTR_CHG_INT_ST_R {
        USB_SERIAL_JTAG_DTR_CHG_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_st(&self) -> USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_R {
        USB_SERIAL_JTAG_GET_LINE_CODE_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_st(&self) -> USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_R {
        USB_SERIAL_JTAG_SET_LINE_CODE_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
