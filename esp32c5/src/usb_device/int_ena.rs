#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_ENA_R = crate::BitReader;
#[doc = "Field `JTAG_IN_FLUSH_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type SOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SOF_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type SOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_IN_EMPTY_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID_ERR_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `PID_ERR_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5_ERR_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CRC5_ERR_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16_ERR_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CRC16_ERR_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUFF_ERR_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `STUFF_ERR_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_BUS_RESET_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_CHG_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type RTS_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RTS_CHG_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type RTS_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_CHG_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type DTR_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DTR_CHG_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type DTR_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type GET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `GET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type GET_LINE_CODE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type SET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type SET_LINE_CODE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush_int_ena(&self) -> JTAG_IN_FLUSH_INT_ENA_R {
        JTAG_IN_FLUSH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof_int_ena(&self) -> SOF_INT_ENA_R {
        SOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_ena(&self) -> SERIAL_OUT_RECV_PKT_INT_ENA_R {
        SERIAL_OUT_RECV_PKT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty_int_ena(&self) -> SERIAL_IN_EMPTY_INT_ENA_R {
        SERIAL_IN_EMPTY_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err_int_ena(&self) -> PID_ERR_INT_ENA_R {
        PID_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err_int_ena(&self) -> CRC5_ERR_INT_ENA_R {
        CRC5_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err_int_ena(&self) -> CRC16_ERR_INT_ENA_R {
        CRC16_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err_int_ena(&self) -> STUFF_ERR_INT_ENA_R {
        STUFF_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_ena(&self) -> IN_TOKEN_REC_IN_EP1_INT_ENA_R {
        IN_TOKEN_REC_IN_EP1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset_int_ena(&self) -> USB_BUS_RESET_INT_ENA_R {
        USB_BUS_RESET_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_ena(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_ENA_R {
        OUT_EP1_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_ena(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_ENA_R {
        OUT_EP2_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg_int_ena(&self) -> RTS_CHG_INT_ENA_R {
        RTS_CHG_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg_int_ena(&self) -> DTR_CHG_INT_ENA_R {
        DTR_CHG_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code_int_ena(&self) -> GET_LINE_CODE_INT_ENA_R {
        GET_LINE_CODE_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code_int_ena(&self) -> SET_LINE_CODE_INT_ENA_R {
        SET_LINE_CODE_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("jtag_in_flush_int_ena", &self.jtag_in_flush_int_ena())
            .field("sof_int_ena", &self.sof_int_ena())
            .field(
                "serial_out_recv_pkt_int_ena",
                &self.serial_out_recv_pkt_int_ena(),
            )
            .field("serial_in_empty_int_ena", &self.serial_in_empty_int_ena())
            .field("pid_err_int_ena", &self.pid_err_int_ena())
            .field("crc5_err_int_ena", &self.crc5_err_int_ena())
            .field("crc16_err_int_ena", &self.crc16_err_int_ena())
            .field("stuff_err_int_ena", &self.stuff_err_int_ena())
            .field(
                "in_token_rec_in_ep1_int_ena",
                &self.in_token_rec_in_ep1_int_ena(),
            )
            .field("usb_bus_reset_int_ena", &self.usb_bus_reset_int_ena())
            .field(
                "out_ep1_zero_payload_int_ena",
                &self.out_ep1_zero_payload_int_ena(),
            )
            .field(
                "out_ep2_zero_payload_int_ena",
                &self.out_ep2_zero_payload_int_ena(),
            )
            .field("rts_chg_int_ena", &self.rts_chg_int_ena())
            .field("dtr_chg_int_ena", &self.dtr_chg_int_ena())
            .field("get_line_code_int_ena", &self.get_line_code_int_ena())
            .field("set_line_code_int_ena", &self.set_line_code_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush_int_ena(&mut self) -> JTAG_IN_FLUSH_INT_ENA_W<'_, INT_ENA_SPEC> {
        JTAG_IN_FLUSH_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof_int_ena(&mut self) -> SOF_INT_ENA_W<'_, INT_ENA_SPEC> {
        SOF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_ena(
        &mut self,
    ) -> SERIAL_OUT_RECV_PKT_INT_ENA_W<'_, INT_ENA_SPEC> {
        SERIAL_OUT_RECV_PKT_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty_int_ena(&mut self) -> SERIAL_IN_EMPTY_INT_ENA_W<'_, INT_ENA_SPEC> {
        SERIAL_IN_EMPTY_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err_int_ena(&mut self) -> PID_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        PID_ERR_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err_int_ena(&mut self) -> CRC5_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        CRC5_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err_int_ena(&mut self) -> CRC16_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        CRC16_ERR_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err_int_ena(&mut self) -> STUFF_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        STUFF_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_ena(
        &mut self,
    ) -> IN_TOKEN_REC_IN_EP1_INT_ENA_W<'_, INT_ENA_SPEC> {
        IN_TOKEN_REC_IN_EP1_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset_int_ena(&mut self) -> USB_BUS_RESET_INT_ENA_W<'_, INT_ENA_SPEC> {
        USB_BUS_RESET_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_ena(
        &mut self,
    ) -> OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<'_, INT_ENA_SPEC> {
        OUT_EP1_ZERO_PAYLOAD_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_ena(
        &mut self,
    ) -> OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<'_, INT_ENA_SPEC> {
        OUT_EP2_ZERO_PAYLOAD_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg_int_ena(&mut self) -> RTS_CHG_INT_ENA_W<'_, INT_ENA_SPEC> {
        RTS_CHG_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg_int_ena(&mut self) -> DTR_CHG_INT_ENA_W<'_, INT_ENA_SPEC> {
        DTR_CHG_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code_int_ena(&mut self) -> GET_LINE_CODE_INT_ENA_W<'_, INT_ENA_SPEC> {
        GET_LINE_CODE_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code_int_ena(&mut self) -> SET_LINE_CODE_INT_ENA_W<'_, INT_ENA_SPEC> {
        SET_LINE_CODE_INT_ENA_W::new(self, 15)
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
