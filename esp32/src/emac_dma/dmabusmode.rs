#[doc = "Register `DMABUSMODE` reader"]
pub type R = crate::R<DMABUSMODE_SPEC>;
#[doc = "Register `DMABUSMODE` writer"]
pub type W = crate::W<DMABUSMODE_SPEC>;
#[doc = "Field `SW_RST` reader - When this bit is set the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the ETH_MAC clock domains. Before reprogramming any register of the ETH_MAC you should read a zero (0) value in this bit."]
pub type SW_RST_R = crate::BitReader;
#[doc = "Field `SW_RST` writer - When this bit is set the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the ETH_MAC clock domains. Before reprogramming any register of the ETH_MAC you should read a zero (0) value in this bit."]
pub type SW_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_ARB_SCH` reader - This bit specifies the arbitration scheme between the transmit and receive paths.1'b0: weighted round-robin with RX:TX or TX:RX priority specified in PR (bit\\[15:14\\]). 1'b1 Fixed priority (Rx priority to Tx)."]
pub type DMA_ARB_SCH_R = crate::BitReader;
#[doc = "Field `DMA_ARB_SCH` writer - This bit specifies the arbitration scheme between the transmit and receive paths.1'b0: weighted round-robin with RX:TX or TX:RX priority specified in PR (bit\\[15:14\\]). 1'b1 Fixed priority (Rx priority to Tx)."]
pub type DMA_ARB_SCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DESC_SKIP_LEN` reader - This bit specifies the number of Word to skip between two unchained descriptors.The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL(DESC_SKIP_LEN) value is equal to zero the descriptor table is taken as contiguous by the DMA in Ring mode."]
pub type DESC_SKIP_LEN_R = crate::FieldReader;
#[doc = "Field `DESC_SKIP_LEN` writer - This bit specifies the number of Word to skip between two unchained descriptors.The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL(DESC_SKIP_LEN) value is equal to zero the descriptor table is taken as contiguous by the DMA in Ring mode."]
pub type DESC_SKIP_LEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ALT_DESC_SIZE` reader - When set the size of the alternate descriptor increases to 32 bytes."]
pub type ALT_DESC_SIZE_R = crate::BitReader;
#[doc = "Field `ALT_DESC_SIZE` writer - When set the size of the alternate descriptor increases to 32 bytes."]
pub type ALT_DESC_SIZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROG_BURST_LEN` reader - These bits indicate the maximum number of beats to be transferred in one DMA transaction. If the number of beats to be transferred is more than 32 then perform the following steps: 1. Set the PBLx8 mode 2. Set the PBL(PROG_BURST_LEN)."]
pub type PROG_BURST_LEN_R = crate::FieldReader;
#[doc = "Field `PROG_BURST_LEN` writer - These bits indicate the maximum number of beats to be transferred in one DMA transaction. If the number of beats to be transferred is more than 32 then perform the following steps: 1. Set the PBLx8 mode 2. Set the PBL(PROG_BURST_LEN)."]
pub type PROG_BURST_LEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `PRI_RATIO` reader - These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio Rx:Tx represented by each bit: 2'b00 -- 1: 1 2'b01 -- 2: 0 2'b10 -- 3: 1 2'b11 -- 4: 1"]
pub type PRI_RATIO_R = crate::FieldReader;
#[doc = "Field `PRI_RATIO` writer - These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio Rx:Tx represented by each bit: 2'b00 -- 1: 1 2'b01 -- 2: 0 2'b10 -- 3: 1 2'b11 -- 4: 1"]
pub type PRI_RATIO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FIXED_BURST` reader - This bit controls whether the AHB master interface performs fixed burst transfers or not. When set the AHB interface uses only SINGLE INCR4 INCR8 or INCR16 during start of the normal burst transfers. When reset the AHB interface uses SINGLE and INCR burst transfer Operations."]
pub type FIXED_BURST_R = crate::BitReader;
#[doc = "Field `FIXED_BURST` writer - This bit controls whether the AHB master interface performs fixed burst transfers or not. When set the AHB interface uses only SINGLE INCR4 INCR8 or INCR16 during start of the normal burst transfers. When reset the AHB interface uses SINGLE and INCR burst transfer Operations."]
pub type FIXED_BURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DMA_PBL` reader - This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write.The Rx DMA always attempts to burst as specified in the RPBL(RX_DMA_PBL) bit each time it starts a burst transfer on the host bus. You can program RPBL with values of 1 2 4 8 16 and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP(USE_SEP_PBL) is set high."]
pub type RX_DMA_PBL_R = crate::FieldReader;
#[doc = "Field `RX_DMA_PBL` writer - This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write.The Rx DMA always attempts to burst as specified in the RPBL(RX_DMA_PBL) bit each time it starts a burst transfer on the host bus. You can program RPBL with values of 1 2 4 8 16 and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP(USE_SEP_PBL) is set high."]
pub type RX_DMA_PBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `USE_SEP_PBL` reader - When set high this bit configures the Rx DMA to use the value configured in Bits\\[22:17\\] as PBL. The PBL value in Bits\\[13:8\\] is applicable only to the Tx DMA operations. When reset to low the PBL value in Bits\\[13:8\\] is applicable for both DMA engines."]
pub type USE_SEP_PBL_R = crate::BitReader;
#[doc = "Field `USE_SEP_PBL` writer - When set high this bit configures the Rx DMA to use the value configured in Bits\\[22:17\\] as PBL. The PBL value in Bits\\[13:8\\] is applicable only to the Tx DMA operations. When reset to low the PBL value in Bits\\[13:8\\] is applicable for both DMA engines."]
pub type USE_SEP_PBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBLX8_MODE` reader - When set high this bit multiplies the programmed PBL value (Bits\\[22:17\\] and Bits\\[13:8\\]) eight times. Therefore the DMA transfers the data in 8 16 32 64 128 and 256 beats depending on the PBL value."]
pub type PBLX8_MODE_R = crate::BitReader;
#[doc = "Field `PBLX8_MODE` writer - When set high this bit multiplies the programmed PBL value (Bits\\[22:17\\] and Bits\\[13:8\\]) eight times. Therefore the DMA transfers the data in 8 16 32 64 128 and 256 beats depending on the PBL value."]
pub type PBLX8_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAADDRALIBEA` reader - When this bit is set high and the FIXED_BURST bit is 1 the AHB interface generates all bursts aligned to the start address LS bits. If the FIXED_BURST bit is 0 the first burst (accessing the start address of data buffer) is not aligned but subsequent bursts are aligned to the address."]
pub type DMAADDRALIBEA_R = crate::BitReader;
#[doc = "Field `DMAADDRALIBEA` writer - When this bit is set high and the FIXED_BURST bit is 1 the AHB interface generates all bursts aligned to the start address LS bits. If the FIXED_BURST bit is 0 the first burst (accessing the start address of data buffer) is not aligned but subsequent bursts are aligned to the address."]
pub type DMAADDRALIBEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAMIXEDBURST` reader - When this bit is set high and the FIXED_BURST bit is low the AHB master interface starts all bursts of a length more than 16 with INCR (undefined burst) whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
pub type DMAMIXEDBURST_R = crate::BitReader;
#[doc = "Field `DMAMIXEDBURST` writer - When this bit is set high and the FIXED_BURST bit is low the AHB master interface starts all bursts of a length more than 16 with INCR (undefined burst) whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
pub type DMAMIXEDBURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When this bit is set the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the ETH_MAC clock domains. Before reprogramming any register of the ETH_MAC you should read a zero (0) value in this bit."]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit specifies the arbitration scheme between the transmit and receive paths.1'b0: weighted round-robin with RX:TX or TX:RX priority specified in PR (bit\\[15:14\\]). 1'b1 Fixed priority (Rx priority to Tx)."]
    #[inline(always)]
    pub fn dma_arb_sch(&self) -> DMA_ARB_SCH_R {
        DMA_ARB_SCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - This bit specifies the number of Word to skip between two unchained descriptors.The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL(DESC_SKIP_LEN) value is equal to zero the descriptor table is taken as contiguous by the DMA in Ring mode."]
    #[inline(always)]
    pub fn desc_skip_len(&self) -> DESC_SKIP_LEN_R {
        DESC_SKIP_LEN_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - When set the size of the alternate descriptor increases to 32 bytes."]
    #[inline(always)]
    pub fn alt_desc_size(&self) -> ALT_DESC_SIZE_R {
        ALT_DESC_SIZE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - These bits indicate the maximum number of beats to be transferred in one DMA transaction. If the number of beats to be transferred is more than 32 then perform the following steps: 1. Set the PBLx8 mode 2. Set the PBL(PROG_BURST_LEN)."]
    #[inline(always)]
    pub fn prog_burst_len(&self) -> PROG_BURST_LEN_R {
        PROG_BURST_LEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio Rx:Tx represented by each bit: 2'b00 -- 1: 1 2'b01 -- 2: 0 2'b10 -- 3: 1 2'b11 -- 4: 1"]
    #[inline(always)]
    pub fn pri_ratio(&self) -> PRI_RATIO_R {
        PRI_RATIO_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - This bit controls whether the AHB master interface performs fixed burst transfers or not. When set the AHB interface uses only SINGLE INCR4 INCR8 or INCR16 during start of the normal burst transfers. When reset the AHB interface uses SINGLE and INCR burst transfer Operations."]
    #[inline(always)]
    pub fn fixed_burst(&self) -> FIXED_BURST_R {
        FIXED_BURST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write.The Rx DMA always attempts to burst as specified in the RPBL(RX_DMA_PBL) bit each time it starts a burst transfer on the host bus. You can program RPBL with values of 1 2 4 8 16 and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP(USE_SEP_PBL) is set high."]
    #[inline(always)]
    pub fn rx_dma_pbl(&self) -> RX_DMA_PBL_R {
        RX_DMA_PBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - When set high this bit configures the Rx DMA to use the value configured in Bits\\[22:17\\] as PBL. The PBL value in Bits\\[13:8\\] is applicable only to the Tx DMA operations. When reset to low the PBL value in Bits\\[13:8\\] is applicable for both DMA engines."]
    #[inline(always)]
    pub fn use_sep_pbl(&self) -> USE_SEP_PBL_R {
        USE_SEP_PBL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When set high this bit multiplies the programmed PBL value (Bits\\[22:17\\] and Bits\\[13:8\\]) eight times. Therefore the DMA transfers the data in 8 16 32 64 128 and 256 beats depending on the PBL value."]
    #[inline(always)]
    pub fn pblx8_mode(&self) -> PBLX8_MODE_R {
        PBLX8_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is set high and the FIXED_BURST bit is 1 the AHB interface generates all bursts aligned to the start address LS bits. If the FIXED_BURST bit is 0 the first burst (accessing the start address of data buffer) is not aligned but subsequent bursts are aligned to the address."]
    #[inline(always)]
    pub fn dmaaddralibea(&self) -> DMAADDRALIBEA_R {
        DMAADDRALIBEA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is set high and the FIXED_BURST bit is low the AHB master interface starts all bursts of a length more than 16 with INCR (undefined burst) whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
    #[inline(always)]
    pub fn dmamixedburst(&self) -> DMAMIXEDBURST_R {
        DMAMIXEDBURST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABUSMODE")
            .field("sw_rst", &format_args!("{}", self.sw_rst().bit()))
            .field("dma_arb_sch", &format_args!("{}", self.dma_arb_sch().bit()))
            .field(
                "desc_skip_len",
                &format_args!("{}", self.desc_skip_len().bits()),
            )
            .field(
                "alt_desc_size",
                &format_args!("{}", self.alt_desc_size().bit()),
            )
            .field(
                "prog_burst_len",
                &format_args!("{}", self.prog_burst_len().bits()),
            )
            .field("pri_ratio", &format_args!("{}", self.pri_ratio().bits()))
            .field("fixed_burst", &format_args!("{}", self.fixed_burst().bit()))
            .field("rx_dma_pbl", &format_args!("{}", self.rx_dma_pbl().bits()))
            .field("use_sep_pbl", &format_args!("{}", self.use_sep_pbl().bit()))
            .field("pblx8_mode", &format_args!("{}", self.pblx8_mode().bit()))
            .field(
                "dmaaddralibea",
                &format_args!("{}", self.dmaaddralibea().bit()),
            )
            .field(
                "dmamixedburst",
                &format_args!("{}", self.dmamixedburst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMABUSMODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the ETH_MAC clock domains. Before reprogramming any register of the ETH_MAC you should read a zero (0) value in this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst(&mut self) -> SW_RST_W<DMABUSMODE_SPEC, 0> {
        SW_RST_W::new(self)
    }
    #[doc = "Bit 1 - This bit specifies the arbitration scheme between the transmit and receive paths.1'b0: weighted round-robin with RX:TX or TX:RX priority specified in PR (bit\\[15:14\\]). 1'b1 Fixed priority (Rx priority to Tx)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_arb_sch(&mut self) -> DMA_ARB_SCH_W<DMABUSMODE_SPEC, 1> {
        DMA_ARB_SCH_W::new(self)
    }
    #[doc = "Bits 2:6 - This bit specifies the number of Word to skip between two unchained descriptors.The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL(DESC_SKIP_LEN) value is equal to zero the descriptor table is taken as contiguous by the DMA in Ring mode."]
    #[inline(always)]
    #[must_use]
    pub fn desc_skip_len(&mut self) -> DESC_SKIP_LEN_W<DMABUSMODE_SPEC, 2> {
        DESC_SKIP_LEN_W::new(self)
    }
    #[doc = "Bit 7 - When set the size of the alternate descriptor increases to 32 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn alt_desc_size(&mut self) -> ALT_DESC_SIZE_W<DMABUSMODE_SPEC, 7> {
        ALT_DESC_SIZE_W::new(self)
    }
    #[doc = "Bits 8:13 - These bits indicate the maximum number of beats to be transferred in one DMA transaction. If the number of beats to be transferred is more than 32 then perform the following steps: 1. Set the PBLx8 mode 2. Set the PBL(PROG_BURST_LEN)."]
    #[inline(always)]
    #[must_use]
    pub fn prog_burst_len(&mut self) -> PROG_BURST_LEN_W<DMABUSMODE_SPEC, 8> {
        PROG_BURST_LEN_W::new(self)
    }
    #[doc = "Bits 14:15 - These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio Rx:Tx represented by each bit: 2'b00 -- 1: 1 2'b01 -- 2: 0 2'b10 -- 3: 1 2'b11 -- 4: 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri_ratio(&mut self) -> PRI_RATIO_W<DMABUSMODE_SPEC, 14> {
        PRI_RATIO_W::new(self)
    }
    #[doc = "Bit 16 - This bit controls whether the AHB master interface performs fixed burst transfers or not. When set the AHB interface uses only SINGLE INCR4 INCR8 or INCR16 during start of the normal burst transfers. When reset the AHB interface uses SINGLE and INCR burst transfer Operations."]
    #[inline(always)]
    #[must_use]
    pub fn fixed_burst(&mut self) -> FIXED_BURST_W<DMABUSMODE_SPEC, 16> {
        FIXED_BURST_W::new(self)
    }
    #[doc = "Bits 17:22 - This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write.The Rx DMA always attempts to burst as specified in the RPBL(RX_DMA_PBL) bit each time it starts a burst transfer on the host bus. You can program RPBL with values of 1 2 4 8 16 and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP(USE_SEP_PBL) is set high."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_pbl(&mut self) -> RX_DMA_PBL_W<DMABUSMODE_SPEC, 17> {
        RX_DMA_PBL_W::new(self)
    }
    #[doc = "Bit 23 - When set high this bit configures the Rx DMA to use the value configured in Bits\\[22:17\\] as PBL. The PBL value in Bits\\[13:8\\] is applicable only to the Tx DMA operations. When reset to low the PBL value in Bits\\[13:8\\] is applicable for both DMA engines."]
    #[inline(always)]
    #[must_use]
    pub fn use_sep_pbl(&mut self) -> USE_SEP_PBL_W<DMABUSMODE_SPEC, 23> {
        USE_SEP_PBL_W::new(self)
    }
    #[doc = "Bit 24 - When set high this bit multiplies the programmed PBL value (Bits\\[22:17\\] and Bits\\[13:8\\]) eight times. Therefore the DMA transfers the data in 8 16 32 64 128 and 256 beats depending on the PBL value."]
    #[inline(always)]
    #[must_use]
    pub fn pblx8_mode(&mut self) -> PBLX8_MODE_W<DMABUSMODE_SPEC, 24> {
        PBLX8_MODE_W::new(self)
    }
    #[doc = "Bit 25 - When this bit is set high and the FIXED_BURST bit is 1 the AHB interface generates all bursts aligned to the start address LS bits. If the FIXED_BURST bit is 0 the first burst (accessing the start address of data buffer) is not aligned but subsequent bursts are aligned to the address."]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddralibea(&mut self) -> DMAADDRALIBEA_W<DMABUSMODE_SPEC, 25> {
        DMAADDRALIBEA_W::new(self)
    }
    #[doc = "Bit 26 - When this bit is set high and the FIXED_BURST bit is low the AHB master interface starts all bursts of a length more than 16 with INCR (undefined burst) whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
    #[inline(always)]
    #[must_use]
    pub fn dmamixedburst(&mut self) -> DMAMIXEDBURST_W<DMABUSMODE_SPEC, 26> {
        DMAMIXEDBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bus mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabusmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabusmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABUSMODE_SPEC;
impl crate::RegisterSpec for DMABUSMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabusmode::R`](R) reader structure"]
impl crate::Readable for DMABUSMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmabusmode::W`](W) writer structure"]
impl crate::Writable for DMABUSMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMABUSMODE to value 0"]
impl crate::Resettable for DMABUSMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
