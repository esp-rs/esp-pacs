#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HPTXSTS_SPEC>;
#[doc = "Field `PTXFSPCAVAIL` reader - Periodic Transmit Data FIFO Space Available (PTxFSpcAvail) Indicates the number of free locations available to be written to in the Periodic TxFIFO. Values are in terms of 32-bit words - 16'h0 : Periodic TxFIFO is full - 16'h1 : 1 word available - 16'h2 : 2 words available - 16'hn : n words available (where 0 n 32,768) - 16'h8000 : 32,768 words - Others : Reserved"]
pub type PTXFSPCAVAIL_R = crate::FieldReader<u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - Periodic Transmit Request Queue Space Available (PTxQSpcAvail) Indicates the number of free locations available to be written in the Periodic Transmit Request Queue. This queue holds both IN and OUT requests. - 7'h0: Periodic Transmit Request Queue is full - 7'h1: 1 location available - 7'h2: 2 locations available - n: n locations available (0 <= n <= 16) - Others: Reserved"]
pub type PTXQSPCAVAIL_R = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - Top of the Periodic Transmit Request Queue (PTxQTop) This indicates the Entry in the Periodic Tx Request Queue that is currently being processed by the MAC. This register is used for debugging. - Bit \\[31\\]: Odd/Even (micro)Frame -- 1'b0: send in even (micro)Frame -- 1'b1: send in odd (micro)Frame - Bits \\[30:27\\]: Channel/endpoint number - Bits \\[26:25\\]: Type -- 2'b00: IN/OUT -- 2'b01: Zero-length packet -- 2'b10: CSPLIT -- 2'b11: Disable channel command - Bit \\[24\\]: Last Periodic Entry for the selected periodic channel/endpoint - Bit \\[23\\]: Terminate (last Entry for the selected channel/endpoint)"]
pub type PTXQTOP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available (PTxFSpcAvail) Indicates the number of free locations available to be written to in the Periodic TxFIFO. Values are in terms of 32-bit words - 16'h0 : Periodic TxFIFO is full - 16'h1 : 1 word available - 16'h2 : 2 words available - 16'hn : n words available (where 0 n 32,768) - 16'h8000 : 32,768 words - Others : Reserved"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Periodic Transmit Request Queue Space Available (PTxQSpcAvail) Indicates the number of free locations available to be written in the Periodic Transmit Request Queue. This queue holds both IN and OUT requests. - 7'h0: Periodic Transmit Request Queue is full - 7'h1: 1 location available - 7'h2: 2 locations available - n: n locations available (0 <= n <= 16) - Others: Reserved"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - Top of the Periodic Transmit Request Queue (PTxQTop) This indicates the Entry in the Periodic Tx Request Queue that is currently being processed by the MAC. This register is used for debugging. - Bit \\[31\\]: Odd/Even (micro)Frame -- 1'b0: send in even (micro)Frame -- 1'b1: send in odd (micro)Frame - Bits \\[30:27\\]: Channel/endpoint number - Bits \\[26:25\\]: Type -- 2'b00: IN/OUT -- 2'b01: Zero-length packet -- 2'b10: CSPLIT -- 2'b11: Disable channel command - Bit \\[24\\]: Last Periodic Entry for the selected periodic channel/endpoint - Bit \\[23\\]: Terminate (last Entry for the selected channel/endpoint)"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfspcavail", &self.ptxfspcavail())
            .field("ptxqspcavail", &self.ptxqspcavail())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
#[doc = "This register contains information about the Periodic Transmit Queue in the Host controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {}
#[doc = "`reset()` method sets HPTXSTS to value 0x0010_0400"]
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0010_0400;
}
