#[doc = "Register `GNPTXSTS` reader"]
pub type R = crate::R<GNPTXSTS_SPEC>;
#[doc = "Field `NPTXFSPCAVAIL` reader - Non-periodic TxFIFO Space Avail (NPTxFSpcAvail) Indicates the amount of free space available in the Non-periodic TxFIFO. Values are in terms of 32-bit words. - 16'h0: Non-periodic TxFIFO is full - 16'h1: 1 word available - 16'h2: 2 words available - 16'hn: n words available (where 0 <= n <= 32,768) - 16'h8000: 32,768 words available - Others: Reserved Reset: Configurable"]
pub type NPTXFSPCAVAIL_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXQSPCAVAIL` reader - Non-periodic Transmit Request Queue Space Available (NPTxQSpcAvail) Indicates the amount of free space available in the Non-periodic Transmit Request Queue. This queue holds both IN and OUT requests in Host mode. Device mode has only IN requests. - 8'h0: Non-periodic Transmit Request Queue is full - 8'h1: 1 location available - 8'h2: 2 locations available - n: n locations available (0 <= n <= 8) - Others: Reserved Reset: Configurable"]
pub type NPTXQSPCAVAIL_R = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the Non-periodic Transmit Request Queue (NPTxQTop) Entry in the Non-periodic Tx Request Queue that is currently being processed by the MAC. - Bits \\[30:27\\]: Channel/endpoint number - Bits \\[26:25\\]: - 2'b00: IN/OUT token -- 2'b01: Zero-length transmit packet (device IN/host OUT) -- 2'b10: PING/CSPLIT token -- 2'b11: Channel halt command - Bit \\[24\\]: Terminate (last Entry for selected channel/endpoint) Reset: 7'h0"]
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail (NPTxFSpcAvail) Indicates the amount of free space available in the Non-periodic TxFIFO. Values are in terms of 32-bit words. - 16'h0: Non-periodic TxFIFO is full - 16'h1: 1 word available - 16'h2: 2 words available - 16'hn: n words available (where 0 <= n <= 32,768) - 16'h8000: 32,768 words available - Others: Reserved Reset: Configurable"]
    #[inline(always)]
    pub fn nptxfspcavail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available (NPTxQSpcAvail) Indicates the amount of free space available in the Non-periodic Transmit Request Queue. This queue holds both IN and OUT requests in Host mode. Device mode has only IN requests. - 8'h0: Non-periodic Transmit Request Queue is full - 8'h1: 1 location available - 8'h2: 2 locations available - n: n locations available (0 <= n <= 8) - Others: Reserved Reset: Configurable"]
    #[inline(always)]
    pub fn nptxqspcavail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue (NPTxQTop) Entry in the Non-periodic Tx Request Queue that is currently being processed by the MAC. - Bits \\[30:27\\]: Channel/endpoint number - Bits \\[26:25\\]: - 2'b00: IN/OUT token -- 2'b01: Zero-length transmit packet (device IN/host OUT) -- 2'b10: PING/CSPLIT token -- 2'b11: Channel halt command - Bit \\[24\\]: Terminate (last Entry for selected channel/endpoint) Reset: 7'h0"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXSTS")
            .field("nptxfspcavail", &self.nptxfspcavail())
            .field("nptxqspcavail", &self.nptxqspcavail())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
#[doc = "In Device mode, this register is valid only in Shared FIFO operation. This read-only register contains the free space information for the Non-periodic TxFIFO and the Non-periodic Transmit Request Queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXSTS_SPEC;
impl crate::RegisterSpec for GNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxsts::R`](R) reader structure"]
impl crate::Readable for GNPTXSTS_SPEC {}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0400"]
impl crate::Resettable for GNPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0008_0400;
}
