#[doc = "Register `RX_STATUS_RX_SETTINGS` reader"]
pub type R = crate::R<RX_STATUS_RX_SETTINGS_SPEC>;
#[doc = "Register `RX_STATUS_RX_SETTINGS` writer"]
pub type W = crate::W<RX_STATUS_RX_SETTINGS_SPEC>;
#[doc = "Field `RXE` reader - Represents whether or not the RX buffer is empty. RX buffer is empty. There is no CAN Frame stored in it. 0: not empty 1: empty"]
pub type RXE_R = crate::BitReader;
#[doc = "Field `RXF` reader - Represents whether or not the RX buffer is full. RX buffer is full, all memory words of RX buffer are occupied. 0: not full 1: full"]
pub type RXF_R = crate::BitReader;
#[doc = "Field `RXMOF` reader - Represents the number of received frame in RX buffer. RX Buffer middle of frame. When RXMOF = 1, next read from RX_DATA register will return other than first word (FRAME_FORMAT_W) of CAN frame."]
pub type RXMOF_R = crate::BitReader;
#[doc = "Field `RXFRC` reader - RX buffer frame count. Number of CAN frames stored in RX buffer."]
pub type RXFRC_R = crate::FieldReader<u16>;
#[doc = "Field `RTSOP` reader - Receive buffer timestamp option. This register should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - RTS_END - Timestamp of received frame in RX FIFO is captured in last bit of EOF field. 0b1 - RTS_BEG - Timestamp of received frame in RX FIFO is captured in SOF field."]
pub type RTSOP_R = crate::BitReader;
#[doc = "Field `RTSOP` writer - Receive buffer timestamp option. This register should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - RTS_END - Timestamp of received frame in RX FIFO is captured in last bit of EOF field. 0b1 - RTS_BEG - Timestamp of received frame in RX FIFO is captured in SOF field."]
pub type RTSOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents whether or not the RX buffer is empty. RX buffer is empty. There is no CAN Frame stored in it. 0: not empty 1: empty"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not the RX buffer is full. RX buffer is full, all memory words of RX buffer are occupied. 0: not full 1: full"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents the number of received frame in RX buffer. RX Buffer middle of frame. When RXMOF = 1, next read from RX_DATA register will return other than first word (FRAME_FORMAT_W) of CAN frame."]
    #[inline(always)]
    pub fn rxmof(&self) -> RXMOF_R {
        RXMOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:14 - RX buffer frame count. Number of CAN frames stored in RX buffer."]
    #[inline(always)]
    pub fn rxfrc(&self) -> RXFRC_R {
        RXFRC_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Receive buffer timestamp option. This register should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - RTS_END - Timestamp of received frame in RX FIFO is captured in last bit of EOF field. 0b1 - RTS_BEG - Timestamp of received frame in RX FIFO is captured in SOF field."]
    #[inline(always)]
    pub fn rtsop(&self) -> RTSOP_R {
        RTSOP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_STATUS_RX_SETTINGS")
            .field("rxe", &self.rxe())
            .field("rxf", &self.rxf())
            .field("rxmof", &self.rxmof())
            .field("rxfrc", &self.rxfrc())
            .field("rtsop", &self.rtsop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - Receive buffer timestamp option. This register should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - RTS_END - Timestamp of received frame in RX FIFO is captured in last bit of EOF field. 0b1 - RTS_BEG - Timestamp of received frame in RX FIFO is captured in SOF field."]
    #[inline(always)]
    pub fn rtsop(&mut self) -> RTSOP_W<RX_STATUS_RX_SETTINGS_SPEC> {
        RTSOP_W::new(self, 16)
    }
}
#[doc = "TWAI FD rx status & setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_status_rx_settings::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_status_rx_settings::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_STATUS_RX_SETTINGS_SPEC;
impl crate::RegisterSpec for RX_STATUS_RX_SETTINGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_status_rx_settings::R`](R) reader structure"]
impl crate::Readable for RX_STATUS_RX_SETTINGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_status_rx_settings::W`](W) writer structure"]
impl crate::Writable for RX_STATUS_RX_SETTINGS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_STATUS_RX_SETTINGS to value 0x01"]
impl crate::Resettable for RX_STATUS_RX_SETTINGS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
