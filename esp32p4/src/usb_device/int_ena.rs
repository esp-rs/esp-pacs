#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SOF_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SOF_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush_int_ena(&self) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_R {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof_int_ena(&self) -> USB_SERIAL_JTAG_SOF_INT_ENA_R {
        USB_SERIAL_JTAG_SOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_ena(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_R {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty_int_ena(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_R {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err_int_ena(&self) -> USB_SERIAL_JTAG_PID_ERR_INT_ENA_R {
        USB_SERIAL_JTAG_PID_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err_int_ena(&self) -> USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_R {
        USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err_int_ena(&self) -> USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_R {
        USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err_int_ena(&self) -> USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_R {
        USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_ena(
        &self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_R {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset_int_ena(&self) -> USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_R {
        USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_ena(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_R {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_ena(
        &self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_R {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg_int_ena(&self) -> USB_SERIAL_JTAG_RTS_CHG_INT_ENA_R {
        USB_SERIAL_JTAG_RTS_CHG_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg_int_ena(&self) -> USB_SERIAL_JTAG_DTR_CHG_INT_ENA_R {
        USB_SERIAL_JTAG_DTR_CHG_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code_int_ena(&self) -> USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_R {
        USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code_int_ena(&self) -> USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_R {
        USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "usb_serial_jtag_jtag_in_flush_int_ena",
                &format_args!("{}", self.usb_serial_jtag_jtag_in_flush_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_sof_int_ena",
                &format_args!("{}", self.usb_serial_jtag_sof_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_serial_out_recv_pkt_int_ena",
                &format_args!(
                    "{}",
                    self.usb_serial_jtag_serial_out_recv_pkt_int_ena().bit()
                ),
            )
            .field(
                "usb_serial_jtag_serial_in_empty_int_ena",
                &format_args!("{}", self.usb_serial_jtag_serial_in_empty_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_pid_err_int_ena",
                &format_args!("{}", self.usb_serial_jtag_pid_err_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_crc5_err_int_ena",
                &format_args!("{}", self.usb_serial_jtag_crc5_err_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_crc16_err_int_ena",
                &format_args!("{}", self.usb_serial_jtag_crc16_err_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_stuff_err_int_ena",
                &format_args!("{}", self.usb_serial_jtag_stuff_err_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_in_token_rec_in_ep1_int_ena",
                &format_args!(
                    "{}",
                    self.usb_serial_jtag_in_token_rec_in_ep1_int_ena().bit()
                ),
            )
            .field(
                "usb_serial_jtag_usb_bus_reset_int_ena",
                &format_args!("{}", self.usb_serial_jtag_usb_bus_reset_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_out_ep1_zero_payload_int_ena",
                &format_args!(
                    "{}",
                    self.usb_serial_jtag_out_ep1_zero_payload_int_ena().bit()
                ),
            )
            .field(
                "usb_serial_jtag_out_ep2_zero_payload_int_ena",
                &format_args!(
                    "{}",
                    self.usb_serial_jtag_out_ep2_zero_payload_int_ena().bit()
                ),
            )
            .field(
                "usb_serial_jtag_rts_chg_int_ena",
                &format_args!("{}", self.usb_serial_jtag_rts_chg_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_dtr_chg_int_ena",
                &format_args!("{}", self.usb_serial_jtag_dtr_chg_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_get_line_code_int_ena",
                &format_args!("{}", self.usb_serial_jtag_get_line_code_int_ena().bit()),
            )
            .field(
                "usb_serial_jtag_set_line_code_int_ena",
                &format_args!("{}", self.usb_serial_jtag_set_line_code_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_jtag_in_flush_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_sof_int_ena(&mut self) -> USB_SERIAL_JTAG_SOF_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SOF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_serial_in_empty_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pid_err_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_PID_ERR_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_PID_ERR_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_crc5_err_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_CRC5_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_crc16_err_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_CRC16_ERR_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_stuff_err_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_STUFF_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_bus_reset_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_USB_BUS_RESET_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_rts_chg_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_RTS_CHG_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_RTS_CHG_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dtr_chg_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_DTR_CHG_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_DTR_CHG_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_get_line_code_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_GET_LINE_CODE_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_set_line_code_int_ena(
        &mut self,
    ) -> USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_W<INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SET_LINE_CODE_INT_ENA_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
