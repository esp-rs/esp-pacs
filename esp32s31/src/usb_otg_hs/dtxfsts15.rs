#[doc = "Register `DTXFSTS15` reader"]
pub type R = crate::R<DTXFSTS15_SPEC>;
#[doc = "Field `INEPTXFSPCAVAIL` reader - IN Endpoint TxFIFO Space Avail (INEPTxFSpcAvail) Indicates the amount of free space available in the Endpoint TxFIFO. Values are in terms of 32-bit words. - 16'h0: Endpoint TxFIFO is full - 16'h1: 1 word available - 16'h2: 2 words available - 16'hn: n words available (where 0 n 32,768) - 16'h8000: 32,768 words available - Others: Reserved In DRD configurations (OTG_MODE = 0, 1, or 2) with dynamic fifo sizing feature enabled (OTG_DFIFO_DYNAMIC=1), the value of this field is, - the maximum value of (OTG_TX_HNPERIO_DFIFO_DEPTH, OTG_TX_DINEP_DFIFO_DEPTH_0) during reset, and - OTG_TX_DINEP_DFIFO_DEPTH_0, immediately after reset deassertion"]
pub type INEPTXFSPCAVAIL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail (INEPTxFSpcAvail) Indicates the amount of free space available in the Endpoint TxFIFO. Values are in terms of 32-bit words. - 16'h0: Endpoint TxFIFO is full - 16'h1: 1 word available - 16'h2: 2 words available - 16'hn: n words available (where 0 n 32,768) - 16'h8000: 32,768 words available - Others: Reserved In DRD configurations (OTG_MODE = 0, 1, or 2) with dynamic fifo sizing feature enabled (OTG_DFIFO_DYNAMIC=1), the value of this field is, - the maximum value of (OTG_TX_HNPERIO_DFIFO_DEPTH, OTG_TX_DINEP_DFIFO_DEPTH_0) during reset, and - OTG_TX_DINEP_DFIFO_DEPTH_0, immediately after reset deassertion"]
    #[inline(always)]
    pub fn ineptxfspcavail(&self) -> INEPTXFSPCAVAIL_R {
        INEPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS15")
            .field("ineptxfspcavail", &self.ineptxfspcavail())
            .finish()
    }
}
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS15_SPEC;
impl crate::RegisterSpec for DTXFSTS15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts15::R`](R) reader structure"]
impl crate::Readable for DTXFSTS15_SPEC {}
#[doc = "`reset()` method sets DTXFSTS15 to value 0x0400"]
impl crate::Resettable for DTXFSTS15_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
