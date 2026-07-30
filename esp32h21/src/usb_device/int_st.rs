#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SOF` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush(&self) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_R {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof(&self) -> USB_SERIAL_JTAG_SOF_R {
        USB_SERIAL_JTAG_SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt(&self) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty(&self) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err(&self) -> USB_SERIAL_JTAG_PID_ERR_R {
        USB_SERIAL_JTAG_PID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err(&self) -> USB_SERIAL_JTAG_CRC5_ERR_R {
        USB_SERIAL_JTAG_CRC5_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err(&self) -> USB_SERIAL_JTAG_CRC16_ERR_R {
        USB_SERIAL_JTAG_CRC16_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err(&self) -> USB_SERIAL_JTAG_STUFF_ERR_R {
        USB_SERIAL_JTAG_STUFF_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1(&self) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset(&self) -> USB_SERIAL_JTAG_USB_BUS_RESET_R {
        USB_SERIAL_JTAG_USB_BUS_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload(&self) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload(&self) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg(&self) -> USB_SERIAL_JTAG_RTS_CHG_R {
        USB_SERIAL_JTAG_RTS_CHG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg(&self) -> USB_SERIAL_JTAG_DTR_CHG_R {
        USB_SERIAL_JTAG_DTR_CHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code(&self) -> USB_SERIAL_JTAG_GET_LINE_CODE_R {
        USB_SERIAL_JTAG_GET_LINE_CODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code(&self) -> USB_SERIAL_JTAG_SET_LINE_CODE_R {
        USB_SERIAL_JTAG_SET_LINE_CODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "usb_serial_jtag_jtag_in_flush",
                &self.usb_serial_jtag_jtag_in_flush(),
            )
            .field("usb_serial_jtag_sof", &self.usb_serial_jtag_sof())
            .field(
                "usb_serial_jtag_serial_out_recv_pkt",
                &self.usb_serial_jtag_serial_out_recv_pkt(),
            )
            .field(
                "usb_serial_jtag_serial_in_empty",
                &self.usb_serial_jtag_serial_in_empty(),
            )
            .field("usb_serial_jtag_pid_err", &self.usb_serial_jtag_pid_err())
            .field("usb_serial_jtag_crc5_err", &self.usb_serial_jtag_crc5_err())
            .field(
                "usb_serial_jtag_crc16_err",
                &self.usb_serial_jtag_crc16_err(),
            )
            .field(
                "usb_serial_jtag_stuff_err",
                &self.usb_serial_jtag_stuff_err(),
            )
            .field(
                "usb_serial_jtag_in_token_rec_in_ep1",
                &self.usb_serial_jtag_in_token_rec_in_ep1(),
            )
            .field(
                "usb_serial_jtag_usb_bus_reset",
                &self.usb_serial_jtag_usb_bus_reset(),
            )
            .field(
                "usb_serial_jtag_out_ep1_zero_payload",
                &self.usb_serial_jtag_out_ep1_zero_payload(),
            )
            .field(
                "usb_serial_jtag_out_ep2_zero_payload",
                &self.usb_serial_jtag_out_ep2_zero_payload(),
            )
            .field("usb_serial_jtag_rts_chg", &self.usb_serial_jtag_rts_chg())
            .field("usb_serial_jtag_dtr_chg", &self.usb_serial_jtag_dtr_chg())
            .field(
                "usb_serial_jtag_get_line_code",
                &self.usb_serial_jtag_get_line_code(),
            )
            .field(
                "usb_serial_jtag_set_line_code",
                &self.usb_serial_jtag_set_line_code(),
            )
            .finish()
    }
}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
