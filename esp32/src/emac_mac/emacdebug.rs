#[doc = "Register `EMACDEBUG` reader"]
pub type R = crate::R<EMACDEBUG_SPEC>;
#[doc = "Field `MACRPES` reader - When high this bit indicates that the MAC MII receive protocol engine is actively receiving data and not in IDLE state."]
pub type MACRPES_R = crate::BitReader;
#[doc = "Field `MACRFFCS` reader - When high this field indicates the active state of the FIFO Read and Write controllers of the MAC Receive Frame Controller Module. MACRFFCS\\[1\\] represents the status of FIFO Read controller. MACRFFCS\\[0\\] represents the status of small FIFO Write controller."]
pub type MACRFFCS_R = crate::FieldReader;
#[doc = "Field `MTLRFWCAS` reader - When high this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO."]
pub type MTLRFWCAS_R = crate::BitReader;
#[doc = "Field `MTLRFRCS` reader - This field gives the state of the Rx FIFO read Controller: 2'b00: IDLE state.2'b01: Reading frame data.2'b10: Reading frame status (or timestamp).2'b11: Flushing the frame data and status."]
pub type MTLRFRCS_R = crate::FieldReader;
#[doc = "Field `MTLRFFLS` reader - This field gives the status of the fill-level of the Rx FIFO: 2'b00: Rx FIFO Empty. 2'b01: Rx FIFO fill-level below flow-control deactivate threshold. 2'b10: Rx FIFO fill-level above flow-control activate threshold. 2'b11: Rx FIFO Full."]
pub type MTLRFFLS_R = crate::FieldReader;
#[doc = "Field `MACTPES` reader - When high this bit indicates that the MAC MII transmit protocol engine is actively transmitting data and is not in the IDLE state."]
pub type MACTPES_R = crate::BitReader;
#[doc = "Field `MACTFCS` reader - This field indicates the state of the MAC Transmit Frame Controller module: 2'b00: IDLE state. 2'b01: Waiting for status of previous frame or IFG or backoff period to be over. 2'b10: Generating and transmitting a Pause frame (in the full-duplex mode). 2'b11: Transferring input frame for transmission."]
pub type MACTFCS_R = crate::FieldReader;
#[doc = "Field `MACTP` reader - When high this bit indicates that the MAC transmitter is in the Pause condition (in the full-duplex-mode) and hence does not schedule any frame for transmission."]
pub type MACTP_R = crate::BitReader;
#[doc = "Field `MTLTFRCS` reader - This field indicates the state of the Tx FIFO Read Controller: 2'b00: IDLE state. 2'b01: READ state (transferring data to the MAC transmitter). 2'b10: Waiting for TxStatus from the MAC transmitter. 2'b11: Writing the received TxStatus or flushing the Tx FIFO."]
pub type MTLTFRCS_R = crate::FieldReader;
#[doc = "Field `MTLTFWCS` reader - When high this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO."]
pub type MTLTFWCS_R = crate::BitReader;
#[doc = "Field `MTLTFNES` reader - When high this bit indicates that the MTL Tx FIFO is not empty and some data is left for Transmission."]
pub type MTLTFNES_R = crate::BitReader;
#[doc = "Field `MTLTSFFS` reader - When high this bit indicates that the MTL TxStatus FIFO is full. Therefore the MTL cannot accept any more frames for transmission."]
pub type MTLTSFFS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When high this bit indicates that the MAC MII receive protocol engine is actively receiving data and not in IDLE state."]
    #[inline(always)]
    pub fn macrpes(&self) -> MACRPES_R {
        MACRPES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - When high this field indicates the active state of the FIFO Read and Write controllers of the MAC Receive Frame Controller Module. MACRFFCS\\[1\\] represents the status of FIFO Read controller. MACRFFCS\\[0\\] represents the status of small FIFO Write controller."]
    #[inline(always)]
    pub fn macrffcs(&self) -> MACRFFCS_R {
        MACRFFCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - When high this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO."]
    #[inline(always)]
    pub fn mtlrfwcas(&self) -> MTLRFWCAS_R {
        MTLRFWCAS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - This field gives the state of the Rx FIFO read Controller: 2'b00: IDLE state.2'b01: Reading frame data.2'b10: Reading frame status (or timestamp).2'b11: Flushing the frame data and status."]
    #[inline(always)]
    pub fn mtlrfrcs(&self) -> MTLRFRCS_R {
        MTLRFRCS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - This field gives the status of the fill-level of the Rx FIFO: 2'b00: Rx FIFO Empty. 2'b01: Rx FIFO fill-level below flow-control deactivate threshold. 2'b10: Rx FIFO fill-level above flow-control activate threshold. 2'b11: Rx FIFO Full."]
    #[inline(always)]
    pub fn mtlrffls(&self) -> MTLRFFLS_R {
        MTLRFFLS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When high this bit indicates that the MAC MII transmit protocol engine is actively transmitting data and is not in the IDLE state."]
    #[inline(always)]
    pub fn mactpes(&self) -> MACTPES_R {
        MACTPES_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - This field indicates the state of the MAC Transmit Frame Controller module: 2'b00: IDLE state. 2'b01: Waiting for status of previous frame or IFG or backoff period to be over. 2'b10: Generating and transmitting a Pause frame (in the full-duplex mode). 2'b11: Transferring input frame for transmission."]
    #[inline(always)]
    pub fn mactfcs(&self) -> MACTFCS_R {
        MACTFCS_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - When high this bit indicates that the MAC transmitter is in the Pause condition (in the full-duplex-mode) and hence does not schedule any frame for transmission."]
    #[inline(always)]
    pub fn mactp(&self) -> MACTP_R {
        MACTP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - This field indicates the state of the Tx FIFO Read Controller: 2'b00: IDLE state. 2'b01: READ state (transferring data to the MAC transmitter). 2'b10: Waiting for TxStatus from the MAC transmitter. 2'b11: Writing the received TxStatus or flushing the Tx FIFO."]
    #[inline(always)]
    pub fn mtltfrcs(&self) -> MTLTFRCS_R {
        MTLTFRCS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - When high this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO."]
    #[inline(always)]
    pub fn mtltfwcs(&self) -> MTLTFWCS_R {
        MTLTFWCS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - When high this bit indicates that the MTL Tx FIFO is not empty and some data is left for Transmission."]
    #[inline(always)]
    pub fn mtltfnes(&self) -> MTLTFNES_R {
        MTLTFNES_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When high this bit indicates that the MTL TxStatus FIFO is full. Therefore the MTL cannot accept any more frames for transmission."]
    #[inline(always)]
    pub fn mtltsffs(&self) -> MTLTSFFS_R {
        MTLTSFFS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACDEBUG")
            .field("macrpes", &format_args!("{}", self.macrpes().bit()))
            .field("macrffcs", &format_args!("{}", self.macrffcs().bits()))
            .field("mtlrfwcas", &format_args!("{}", self.mtlrfwcas().bit()))
            .field("mtlrfrcs", &format_args!("{}", self.mtlrfrcs().bits()))
            .field("mtlrffls", &format_args!("{}", self.mtlrffls().bits()))
            .field("mactpes", &format_args!("{}", self.mactpes().bit()))
            .field("mactfcs", &format_args!("{}", self.mactfcs().bits()))
            .field("mactp", &format_args!("{}", self.mactp().bit()))
            .field("mtltfrcs", &format_args!("{}", self.mtltfrcs().bits()))
            .field("mtltfwcs", &format_args!("{}", self.mtltfwcs().bit()))
            .field("mtltfnes", &format_args!("{}", self.mtltfnes().bit()))
            .field("mtltsffs", &format_args!("{}", self.mtltsffs().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACDEBUG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status debugging bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacdebug::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACDEBUG_SPEC;
impl crate::RegisterSpec for EMACDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacdebug::R`](R) reader structure"]
impl crate::Readable for EMACDEBUG_SPEC {}
#[doc = "`reset()` method sets EMACDEBUG to value 0"]
impl crate::Resettable for EMACDEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
