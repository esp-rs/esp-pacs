#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH` reader - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH` writer - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SOF` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SOF` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR` reader - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR` writer - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR` reader - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR` writer - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR` reader - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR` writer - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR` reader - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR` writer - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1` reader - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1` writer - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET` reader - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET` writer - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD` reader - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD` writer - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD` reader - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD` writer - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG` reader - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG` writer - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG` reader - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG` writer - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE` reader - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE` writer - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush(&self) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_R {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof(&self) -> USB_SERIAL_JTAG_SOF_R {
        USB_SERIAL_JTAG_SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt(&self) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty(&self) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err(&self) -> USB_SERIAL_JTAG_PID_ERR_R {
        USB_SERIAL_JTAG_PID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err(&self) -> USB_SERIAL_JTAG_CRC5_ERR_R {
        USB_SERIAL_JTAG_CRC5_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err(&self) -> USB_SERIAL_JTAG_CRC16_ERR_R {
        USB_SERIAL_JTAG_CRC16_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err(&self) -> USB_SERIAL_JTAG_STUFF_ERR_R {
        USB_SERIAL_JTAG_STUFF_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1(&self) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset(&self) -> USB_SERIAL_JTAG_USB_BUS_RESET_R {
        USB_SERIAL_JTAG_USB_BUS_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload(&self) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload(&self) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg(&self) -> USB_SERIAL_JTAG_RTS_CHG_R {
        USB_SERIAL_JTAG_RTS_CHG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg(&self) -> USB_SERIAL_JTAG_DTR_CHG_R {
        USB_SERIAL_JTAG_DTR_CHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code(&self) -> USB_SERIAL_JTAG_GET_LINE_CODE_R {
        USB_SERIAL_JTAG_GET_LINE_CODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code(&self) -> USB_SERIAL_JTAG_SET_LINE_CODE_R {
        USB_SERIAL_JTAG_SET_LINE_CODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
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
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_jtag_in_flush(
        &mut self,
    ) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof(&mut self) -> USB_SERIAL_JTAG_SOF_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_recv_pkt(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_empty(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_pid_err(&mut self) -> USB_SERIAL_JTAG_PID_ERR_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_PID_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc5_err(&mut self) -> USB_SERIAL_JTAG_CRC5_ERR_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_CRC5_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_crc16_err(&mut self) -> USB_SERIAL_JTAG_CRC16_ERR_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_CRC16_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_stuff_err(&mut self) -> USB_SERIAL_JTAG_STUFF_ERR_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_STUFF_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_token_rec_in_ep1(
        &mut self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_bus_reset(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_BUS_RESET_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_USB_BUS_RESET_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep1_zero_payload(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_ep2_zero_payload(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_rts_chg(&mut self) -> USB_SERIAL_JTAG_RTS_CHG_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_RTS_CHG_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_dtr_chg(&mut self) -> USB_SERIAL_JTAG_DTR_CHG_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_DTR_CHG_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_line_code(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_LINE_CODE_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_GET_LINE_CODE_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn usb_serial_jtag_set_line_code(
        &mut self,
    ) -> USB_SERIAL_JTAG_SET_LINE_CODE_W<'_, INT_ENA_SPEC> {
        USB_SERIAL_JTAG_SET_LINE_CODE_W::new(self, 15)
    }
}
#[doc = "Interrupt enable status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
