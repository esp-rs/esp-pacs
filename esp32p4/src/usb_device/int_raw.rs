///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `JTAG_IN_FLUSH` reader - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
pub type JTAG_IN_FLUSH_R = crate::BitReader;
///Field `JTAG_IN_FLUSH` writer - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
pub type JTAG_IN_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF` reader - The raw interrupt bit turns to high level when SOF frame is received.
pub type SOF_R = crate::BitReader;
///Field `SOF` writer - The raw interrupt bit turns to high level when SOF frame is received.
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERIAL_OUT_RECV_PKT` reader - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
pub type SERIAL_OUT_RECV_PKT_R = crate::BitReader;
///Field `SERIAL_OUT_RECV_PKT` writer - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
pub type SERIAL_OUT_RECV_PKT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERIAL_IN_EMPTY` reader - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
pub type SERIAL_IN_EMPTY_R = crate::BitReader;
///Field `SERIAL_IN_EMPTY` writer - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
pub type SERIAL_IN_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PID_ERR` reader - The raw interrupt bit turns to high level when pid error is detected.
pub type PID_ERR_R = crate::BitReader;
///Field `PID_ERR` writer - The raw interrupt bit turns to high level when pid error is detected.
pub type PID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC5_ERR` reader - The raw interrupt bit turns to high level when CRC5 error is detected.
pub type CRC5_ERR_R = crate::BitReader;
///Field `CRC5_ERR` writer - The raw interrupt bit turns to high level when CRC5 error is detected.
pub type CRC5_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC16_ERR` reader - The raw interrupt bit turns to high level when CRC16 error is detected.
pub type CRC16_ERR_R = crate::BitReader;
///Field `CRC16_ERR` writer - The raw interrupt bit turns to high level when CRC16 error is detected.
pub type CRC16_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUFF_ERR` reader - The raw interrupt bit turns to high level when stuff error is detected.
pub type STUFF_ERR_R = crate::BitReader;
///Field `STUFF_ERR` writer - The raw interrupt bit turns to high level when stuff error is detected.
pub type STUFF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_TOKEN_REC_IN_EP1` reader - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
pub type IN_TOKEN_REC_IN_EP1_R = crate::BitReader;
///Field `IN_TOKEN_REC_IN_EP1` writer - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
pub type IN_TOKEN_REC_IN_EP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB_BUS_RESET` reader - The raw interrupt bit turns to high level when usb bus reset is detected.
pub type USB_BUS_RESET_R = crate::BitReader;
///Field `USB_BUS_RESET` writer - The raw interrupt bit turns to high level when usb bus reset is detected.
pub type USB_BUS_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EP1_ZERO_PAYLOAD` reader - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
pub type OUT_EP1_ZERO_PAYLOAD_R = crate::BitReader;
///Field `OUT_EP1_ZERO_PAYLOAD` writer - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
pub type OUT_EP1_ZERO_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EP2_ZERO_PAYLOAD` reader - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
pub type OUT_EP2_ZERO_PAYLOAD_R = crate::BitReader;
///Field `OUT_EP2_ZERO_PAYLOAD` writer - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
pub type OUT_EP2_ZERO_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTS_CHG` reader - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
pub type RTS_CHG_R = crate::BitReader;
///Field `RTS_CHG` writer - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
pub type RTS_CHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTR_CHG` reader - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
pub type DTR_CHG_R = crate::BitReader;
///Field `DTR_CHG` writer - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
pub type DTR_CHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GET_LINE_CODE` reader - The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
pub type GET_LINE_CODE_R = crate::BitReader;
///Field `GET_LINE_CODE` writer - The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
pub type GET_LINE_CODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SET_LINE_CODE` reader - The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
pub type SET_LINE_CODE_R = crate::BitReader;
///Field `SET_LINE_CODE` writer - The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
pub type SET_LINE_CODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    #[inline(always)]
    pub fn jtag_in_flush(&self) -> JTAG_IN_FLUSH_R {
        JTAG_IN_FLUSH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The raw interrupt bit turns to high level when SOF frame is received.
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    #[inline(always)]
    pub fn serial_out_recv_pkt(&self) -> SERIAL_OUT_RECV_PKT_R {
        SERIAL_OUT_RECV_PKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    #[inline(always)]
    pub fn serial_in_empty(&self) -> SERIAL_IN_EMPTY_R {
        SERIAL_IN_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The raw interrupt bit turns to high level when pid error is detected.
    #[inline(always)]
    pub fn pid_err(&self) -> PID_ERR_R {
        PID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected.
    #[inline(always)]
    pub fn crc5_err(&self) -> CRC5_ERR_R {
        CRC5_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected.
    #[inline(always)]
    pub fn crc16_err(&self) -> CRC16_ERR_R {
        CRC16_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The raw interrupt bit turns to high level when stuff error is detected.
    #[inline(always)]
    pub fn stuff_err(&self) -> STUFF_ERR_R {
        STUFF_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    #[inline(always)]
    pub fn in_token_rec_in_ep1(&self) -> IN_TOKEN_REC_IN_EP1_R {
        IN_TOKEN_REC_IN_EP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected.
    #[inline(always)]
    pub fn usb_bus_reset(&self) -> USB_BUS_RESET_R {
        USB_BUS_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    #[inline(always)]
    pub fn out_ep1_zero_payload(&self) -> OUT_EP1_ZERO_PAYLOAD_R {
        OUT_EP1_ZERO_PAYLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    #[inline(always)]
    pub fn out_ep2_zero_payload(&self) -> OUT_EP2_ZERO_PAYLOAD_R {
        OUT_EP2_ZERO_PAYLOAD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    #[inline(always)]
    pub fn rts_chg(&self) -> RTS_CHG_R {
        RTS_CHG_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    #[inline(always)]
    pub fn dtr_chg(&self) -> DTR_CHG_R {
        DTR_CHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    #[inline(always)]
    pub fn get_line_code(&self) -> GET_LINE_CODE_R {
        GET_LINE_CODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    #[inline(always)]
    pub fn set_line_code(&self) -> SET_LINE_CODE_R {
        SET_LINE_CODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("jtag_in_flush", &self.jtag_in_flush())
            .field("sof", &self.sof())
            .field("serial_out_recv_pkt", &self.serial_out_recv_pkt())
            .field("serial_in_empty", &self.serial_in_empty())
            .field("pid_err", &self.pid_err())
            .field("crc5_err", &self.crc5_err())
            .field("crc16_err", &self.crc16_err())
            .field("stuff_err", &self.stuff_err())
            .field("in_token_rec_in_ep1", &self.in_token_rec_in_ep1())
            .field("usb_bus_reset", &self.usb_bus_reset())
            .field("out_ep1_zero_payload", &self.out_ep1_zero_payload())
            .field("out_ep2_zero_payload", &self.out_ep2_zero_payload())
            .field("rts_chg", &self.rts_chg())
            .field("dtr_chg", &self.dtr_chg())
            .field("get_line_code", &self.get_line_code())
            .field("set_line_code", &self.set_line_code())
            .finish()
    }
}
impl W {
    ///Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG.
    #[inline(always)]
    #[must_use]
    pub fn jtag_in_flush(&mut self) -> JTAG_IN_FLUSH_W<INT_RAW_SPEC> {
        JTAG_IN_FLUSH_W::new(self, 0)
    }
    ///Bit 1 - The raw interrupt bit turns to high level when SOF frame is received.
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<INT_RAW_SPEC> {
        SOF_W::new(self, 1)
    }
    ///Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet.
    #[inline(always)]
    #[must_use]
    pub fn serial_out_recv_pkt(&mut self) -> SERIAL_OUT_RECV_PKT_W<INT_RAW_SPEC> {
        SERIAL_OUT_RECV_PKT_W::new(self, 2)
    }
    ///Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty.
    #[inline(always)]
    #[must_use]
    pub fn serial_in_empty(&mut self) -> SERIAL_IN_EMPTY_W<INT_RAW_SPEC> {
        SERIAL_IN_EMPTY_W::new(self, 3)
    }
    ///Bit 4 - The raw interrupt bit turns to high level when pid error is detected.
    #[inline(always)]
    #[must_use]
    pub fn pid_err(&mut self) -> PID_ERR_W<INT_RAW_SPEC> {
        PID_ERR_W::new(self, 4)
    }
    ///Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected.
    #[inline(always)]
    #[must_use]
    pub fn crc5_err(&mut self) -> CRC5_ERR_W<INT_RAW_SPEC> {
        CRC5_ERR_W::new(self, 5)
    }
    ///Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected.
    #[inline(always)]
    #[must_use]
    pub fn crc16_err(&mut self) -> CRC16_ERR_W<INT_RAW_SPEC> {
        CRC16_ERR_W::new(self, 6)
    }
    ///Bit 7 - The raw interrupt bit turns to high level when stuff error is detected.
    #[inline(always)]
    #[must_use]
    pub fn stuff_err(&mut self) -> STUFF_ERR_W<INT_RAW_SPEC> {
        STUFF_ERR_W::new(self, 7)
    }
    ///Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received.
    #[inline(always)]
    #[must_use]
    pub fn in_token_rec_in_ep1(&mut self) -> IN_TOKEN_REC_IN_EP1_W<INT_RAW_SPEC> {
        IN_TOKEN_REC_IN_EP1_W::new(self, 8)
    }
    ///Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected.
    #[inline(always)]
    #[must_use]
    pub fn usb_bus_reset(&mut self) -> USB_BUS_RESET_W<INT_RAW_SPEC> {
        USB_BUS_RESET_W::new(self, 9)
    }
    ///Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload.
    #[inline(always)]
    #[must_use]
    pub fn out_ep1_zero_payload(&mut self) -> OUT_EP1_ZERO_PAYLOAD_W<INT_RAW_SPEC> {
        OUT_EP1_ZERO_PAYLOAD_W::new(self, 10)
    }
    ///Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload.
    #[inline(always)]
    #[must_use]
    pub fn out_ep2_zero_payload(&mut self) -> OUT_EP2_ZERO_PAYLOAD_W<INT_RAW_SPEC> {
        OUT_EP2_ZERO_PAYLOAD_W::new(self, 11)
    }
    ///Bit 12 - The raw interrupt bit turns to high level when level of RTS from usb serial channel is changed.
    #[inline(always)]
    #[must_use]
    pub fn rts_chg(&mut self) -> RTS_CHG_W<INT_RAW_SPEC> {
        RTS_CHG_W::new(self, 12)
    }
    ///Bit 13 - The raw interrupt bit turns to high level when level of DTR from usb serial channel is changed.
    #[inline(always)]
    #[must_use]
    pub fn dtr_chg(&mut self) -> DTR_CHG_W<INT_RAW_SPEC> {
        DTR_CHG_W::new(self, 13)
    }
    ///Bit 14 - The raw interrupt bit turns to high level when level of GET LINE CODING request is received.
    #[inline(always)]
    #[must_use]
    pub fn get_line_code(&mut self) -> GET_LINE_CODE_W<INT_RAW_SPEC> {
        GET_LINE_CODE_W::new(self, 14)
    }
    ///Bit 15 - The raw interrupt bit turns to high level when level of SET LINE CODING request is received.
    #[inline(always)]
    #[must_use]
    pub fn set_line_code(&mut self) -> SET_LINE_CODE_W<INT_RAW_SPEC> {
        SET_LINE_CODE_W::new(self, 15)
    }
}
/**Interrupt raw status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0x08
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
