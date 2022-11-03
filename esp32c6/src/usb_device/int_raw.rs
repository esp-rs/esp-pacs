#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW` reader - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SOF_INT_RAW` reader - The raw interrupt bit turns to high level when SOF frame is received."]
pub type USB_SERIAL_JTAG_SOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when pid error is detected."]
pub type USB_SERIAL_JTAG_PID_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub type USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub type USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when stuff error is detected."]
pub type USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW` reader - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW` reader - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG_INT_RAW` reader - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
pub type USB_SERIAL_JTAG_RTS_CHG_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG_INT_RAW` reader - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
pub type USB_SERIAL_JTAG_DTR_CHG_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW` reader - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW` reader - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_raw(&self) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_R {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_raw(&self) -> USB_SERIAL_JTAG_SOF_INT_RAW_R {
        USB_SERIAL_JTAG_SOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_raw(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_R {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_raw(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_R {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_raw(&self) -> USB_SERIAL_JTAG_PID_ERR_INT_RAW_R {
        USB_SERIAL_JTAG_PID_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_raw(&self) -> USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_R {
        USB_SERIAL_JTAG_CRC5_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_raw(&self) -> USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_R {
        USB_SERIAL_JTAG_CRC16_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_raw(&self) -> USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_R {
        USB_SERIAL_JTAG_STUFF_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_raw(
        &self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_R {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_raw(&self) -> USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_R {
        USB_SERIAL_JTAG_USB_BUS_RESET_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_raw(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_raw(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_raw(&self) -> USB_SERIAL_JTAG_RTS_CHG_INT_RAW_R {
        USB_SERIAL_JTAG_RTS_CHG_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_raw(&self) -> USB_SERIAL_JTAG_DTR_CHG_INT_RAW_R {
        USB_SERIAL_JTAG_DTR_CHG_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt bit turns to high level when level of GET LINE CODING request is received."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_raw(&self) -> USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_R {
        USB_SERIAL_JTAG_GET_LINE_CODE_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt bit turns to high level when level of SET LINE CODING request is received."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_raw(&self) -> USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_R {
        USB_SERIAL_JTAG_SET_LINE_CODE_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt raw status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
