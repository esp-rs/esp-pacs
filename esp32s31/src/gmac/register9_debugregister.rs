#[doc = "Register `REGISTER9_DEBUGREGISTER` reader"]
pub type R = crate::R<REGISTER9_DEBUGREGISTER_SPEC>;
#[doc = "Field `RPESTS` reader - MAC GMII or MII Receive Protocol Engine Status When high, this bit indicates that the MAC GMII or MII receive protocol engine is actively receiving data and not in IDLE state"]
pub type RPESTS_R = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame FIFO Controller Status When high, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Frame Controller Module RFCFCSTS\\[1\\] represents the status of small FIFO Read controller RFCFCSTS\\[0\\] represents the status of small FIFO Write controller"]
pub type RFCFCSTS_R = crate::FieldReader;
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status When high, this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO"]
pub type RWCSTS_R = crate::BitReader;
#[doc = "Field `RRCSTS` reader - MTL RxFIFO Read Controller State This field gives the state of the Rx FIFO read Controller: 00: IDLE state 01: Reading frame data 10: Reading frame status _or timestamp_ 11: Flushing the frame data and status"]
pub type RRCSTS_R = crate::FieldReader;
#[doc = "Field `RXFSTS` reader - MTL RxFIFO FillLevel Status This field gives the status of the filllevel of the Rx FIFO: 00: Rx FIFO Empty 01: Rx FIFO filllevel below flowcontrol deactivate threshold 10: Rx FIFO filllevel above flowcontrol activate threshold 11: Rx FIFO Full"]
pub type RXFSTS_R = crate::FieldReader;
#[doc = "Field `TPESTS` reader - MAC GMII or MII Transmit Protocol Engine Status When high, this bit indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data and is not in the IDLE state"]
pub type TPESTS_R = crate::BitReader;
#[doc = "Field `TFCSTS` reader - MAC Transmit Frame Controller Status This field indicates the state of the MAC Transmit Frame Controller module: 00: IDLE state 01: Waiting for status of previous frame or IFG or backoff period to be over 10: Generating and transmitting a Pause frame _in the fullduplex mode_ 11: Transferring input frame for transmission"]
pub type TFCSTS_R = crate::FieldReader;
#[doc = "Field `TXPAUSED` reader - MAC Transmitter in Pause When high, this bit indicates that the MAC transmitter is in the Pause condition _in the fullduplexonly mode_ and hence does not schedule any frame for transmission"]
pub type TXPAUSED_R = crate::BitReader;
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status This field indicates the state of the Tx FIFO Read Controller: 00: IDLE state 01: READ state _transferring data to the MAC transmitter_ 10: Waiting for TxStatus from the MAC transmitter 11: Writing the received TxStatus or flushing the Tx FIFO"]
pub type TRCSTS_R = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Status When high, this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO"]
pub type TWCSTS_R = crate::BitReader;
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status When high, this bit indicates that the MTL Tx FIFO is not empty and some data is left for transmission"]
pub type TXFSTS_R = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status When high, this bit indicates that the MTL TxStatus FIFO is full Therefore, the MTL cannot accept any more frames for transmission This bit is reserved in the GMACAHB and GMACDMA configurations"]
pub type TXSTSFSTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MAC GMII or MII Receive Protocol Engine Status When high, this bit indicates that the MAC GMII or MII receive protocol engine is actively receiving data and not in IDLE state"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame FIFO Controller Status When high, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Frame Controller Module RFCFCSTS\\[1\\] represents the status of small FIFO Read controller RFCFCSTS\\[0\\] represents the status of small FIFO Write controller"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status When high, this bit indicates that the MTL Rx FIFO Write Controller is active and is transferring a received frame to the FIFO"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL RxFIFO Read Controller State This field gives the state of the Rx FIFO read Controller: 00: IDLE state 01: Reading frame data 10: Reading frame status _or timestamp_ 11: Flushing the frame data and status"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MTL RxFIFO FillLevel Status This field gives the status of the filllevel of the Rx FIFO: 00: Rx FIFO Empty 01: Rx FIFO filllevel below flowcontrol deactivate threshold 10: Rx FIFO filllevel above flowcontrol activate threshold 11: Rx FIFO Full"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RXFSTS_R {
        RXFSTS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC GMII or MII Transmit Protocol Engine Status When high, this bit indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data and is not in the IDLE state"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status This field indicates the state of the MAC Transmit Frame Controller module: 00: IDLE state 01: Waiting for status of previous frame or IFG or backoff period to be over 10: Generating and transmitting a Pause frame _in the fullduplex mode_ 11: Transferring input frame for transmission"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter in Pause When high, this bit indicates that the MAC transmitter is in the Pause condition _in the fullduplexonly mode_ and hence does not schedule any frame for transmission"]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status This field indicates the state of the Tx FIFO Read Controller: 00: IDLE state 01: READ state _transferring data to the MAC transmitter_ 10: Waiting for TxStatus from the MAC transmitter 11: Writing the received TxStatus or flushing the Tx FIFO"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Status When high, this bit indicates that the MTL Tx FIFO Write Controller is active and is transferring data to the Tx FIFO"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status When high, this bit indicates that the MTL Tx FIFO is not empty and some data is left for transmission"]
    #[inline(always)]
    pub fn txfsts(&self) -> TXFSTS_R {
        TXFSTS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status When high, this bit indicates that the MTL TxStatus FIFO is full Therefore, the MTL cannot accept any more frames for transmission This bit is reserved in the GMACAHB and GMACDMA configurations"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER9_DEBUGREGISTER")
            .field("rpests", &self.rpests())
            .field("rfcfcsts", &self.rfcfcsts())
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxfsts", &self.rxfsts())
            .field("tpests", &self.tpests())
            .field("tfcsts", &self.tfcsts())
            .field("txpaused", &self.txpaused())
            .field("trcsts", &self.trcsts())
            .field("twcsts", &self.twcsts())
            .field("txfsts", &self.txfsts())
            .field("txstsfsts", &self.txstsfsts())
            .finish()
    }
}
#[doc = "Gives the status of various internal blocks for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`register9_debugregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER9_DEBUGREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER9_DEBUGREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register9_debugregister::R`](R) reader structure"]
impl crate::Readable for REGISTER9_DEBUGREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER9_DEBUGREGISTER to value 0"]
impl crate::Resettable for REGISTER9_DEBUGREGISTER_SPEC {}
