#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSFTRST` reader - Mode: Host and Device Core Soft Reset (CSftRst) Resets the hclk and phy_clock domains as follows: - Clears the interrupts and all the CSR registers except the following register bits: -- PCGCCTL.RstPdwnModule -- PCGCCTL.GateHclk -- PCGCCTL.PwrClmp -- PCGCCTL.StopPPhyLPwrClkSelclk -- GUSBCFG.ForceDevMode -- GUSBCFG.ForceHstMode -- GUSBCFG.PhyLPwrClkSel -- GUSBCFG.DDRSel -- GUSBCFG.PHYSel -- GUSBCFG.FSIntf -- GUSBCFG.ULPI_UTMI_Sel -- GUSBCFG.PHYIf -- GUSBCFG.TxEndDelay -- GUSBCFG.TermSelDLPulse -- GUSBCFG.ULPIClkSusM -- GUSBCFG.ULPIAutoRes -- GUSBCFG.ULPIFsLs -- GGPIO -- GPWRDN -- GADPCTL -- HCFG.FSLSPclkSel -- DCFG.DevSpd -- DCTL.SftDiscon - All module state machines - All module state machines (except the AHB Slave Unit) are reset to the IDLE state, and all the transmit FIFOs and the receive FIFO are flushed. - Any transactions on the AHB Master are terminated as soon as possible, after gracefully completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. - When Hibernation or ADP feature is enabled, the PMU module is not reset by the Core Soft Reset. The application can write to this bit any time it wants to reset the core. The application must clear this bit after checking the bit 29 of this register (Core Soft Reset Done). Software must also must check that bit 31 of this register is 1 (AHB Master is IDLE) before starting any operation. Typically software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation."]
pub type CSFTRST_R = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Mode: Host and Device Core Soft Reset (CSftRst) Resets the hclk and phy_clock domains as follows: - Clears the interrupts and all the CSR registers except the following register bits: -- PCGCCTL.RstPdwnModule -- PCGCCTL.GateHclk -- PCGCCTL.PwrClmp -- PCGCCTL.StopPPhyLPwrClkSelclk -- GUSBCFG.ForceDevMode -- GUSBCFG.ForceHstMode -- GUSBCFG.PhyLPwrClkSel -- GUSBCFG.DDRSel -- GUSBCFG.PHYSel -- GUSBCFG.FSIntf -- GUSBCFG.ULPI_UTMI_Sel -- GUSBCFG.PHYIf -- GUSBCFG.TxEndDelay -- GUSBCFG.TermSelDLPulse -- GUSBCFG.ULPIClkSusM -- GUSBCFG.ULPIAutoRes -- GUSBCFG.ULPIFsLs -- GGPIO -- GPWRDN -- GADPCTL -- HCFG.FSLSPclkSel -- DCFG.DevSpd -- DCTL.SftDiscon - All module state machines - All module state machines (except the AHB Slave Unit) are reset to the IDLE state, and all the transmit FIFOs and the receive FIFO are flushed. - Any transactions on the AHB Master are terminated as soon as possible, after gracefully completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. - When Hibernation or ADP feature is enabled, the PMU module is not reset by the Core Soft Reset. The application can write to this bit any time it wants to reset the core. The application must clear this bit after checking the bit 29 of this register (Core Soft Reset Done). Software must also must check that bit 31 of this register is 1 (AHB Master is IDLE) before starting any operation. Typically software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation."]
pub type CSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUFSSFTRST` reader - Mode: Host and Device PIU FS Dedicated Controller Soft Reset (PIUFSSftRst) Resets the PIU FS Dedicated Controller All module state machines in FS Dedicated Controller of PIU are reset to the IDLE state. Used to reset the FS Dedicated controller in PIU in case of any PHY Errors like Loss of activity or Babble Error resulting in the PHY remaining in RX state for more than one frame boundary. This is a self clearing bit and core clears this bit after all the necessary logic is reset in the core."]
pub type PIUFSSFTRST_R = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - Mode: Host and Device PIU FS Dedicated Controller Soft Reset (PIUFSSftRst) Resets the PIU FS Dedicated Controller All module state machines in FS Dedicated Controller of PIU are reset to the IDLE state. Used to reset the FS Dedicated controller in PIU in case of any PHY Errors like Loss of activity or Babble Error resulting in the PHY remaining in RX state for more than one frame boundary. This is a self clearing bit and core clears this bit after all the necessary logic is reset in the core."]
pub type PIUFSSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMCNTRRST` reader - Mode: Host only Host Frame Counter Reset (FrmCntrRst) The application writes this bit to reset the (micro)Frame number counter inside the core. When the (micro)Frame counter is reset, the subsequent SOF sent out by the core has a (micro)Frame number of 0. When application writes 1 to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles."]
pub type FRMCNTRRST_R = crate::BitReader;
#[doc = "Field `FRMCNTRRST` writer - Mode: Host only Host Frame Counter Reset (FrmCntrRst) The application writes this bit to reset the (micro)Frame number counter inside the core. When the (micro)Frame counter is reset, the subsequent SOF sent out by the core has a (micro)Frame number of 0. When application writes 1 to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles."]
pub type FRMCNTRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - Mode: Host and Device RxFIFO Flush (RxFFlsh) The application can flush the entire RxFIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the controller is neither reading from the RxFIFO nor writing to the RxFIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires eight clocks (slowest of PHY or AHB clock) to clear."]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - Mode: Host and Device RxFIFO Flush (RxFFlsh) The application can flush the entire RxFIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the controller is neither reading from the RxFIFO nor writing to the RxFIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires eight clocks (slowest of PHY or AHB clock) to clear."]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - Mode: Host and Device TxFIFO Flush (TxFFlsh) This bit selectively flushes a single or all transmit FIFOs, but cannot do so If the core is in the midst of a transaction. The application must write this bit only after checking that the core is neither writing to the TxFIFO nor reading from the TxFIFO. Verify using these registers: - ReadNAK Effective Interrupt ensures the core is not reading from the FIFO - WriteGRSTCTL.AHBIdle ensures the core is not writing anything to the FIFO. Flushing is normally recommended when FIFOs are reconfigured or when switching between Shared FIFO and Dedicated Transmit FIFO operation. FIFO flushing is also recommended during device endpoint disable. The application must wait until the core clears this bit before performing any operations. This bit takes eight clocks to clear, using the slower clock of phy_clk or hclk."]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - Mode: Host and Device TxFIFO Flush (TxFFlsh) This bit selectively flushes a single or all transmit FIFOs, but cannot do so If the core is in the midst of a transaction. The application must write this bit only after checking that the core is neither writing to the TxFIFO nor reading from the TxFIFO. Verify using these registers: - ReadNAK Effective Interrupt ensures the core is not reading from the FIFO - WriteGRSTCTL.AHBIdle ensures the core is not writing anything to the FIFO. Flushing is normally recommended when FIFOs are reconfigured or when switching between Shared FIFO and Dedicated Transmit FIFO operation. FIFO flushing is also recommended during device endpoint disable. The application must wait until the core clears this bit before performing any operations. This bit takes eight clocks to clear, using the slower clock of phy_clk or hclk."]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - Mode: Host and Device TxFIFO Number (TxFNum) This is the FIFO number that must be flushed using the TxFIFO Flush bit. This field must not be changed until the core clears the TxFIFO Flush bit. - 5'h0: -- Non-periodic TxFIFO flush in Host mode -- Non-periodic TxFIFO flush in device mode when in shared FIFO operation -- Tx FIFO 0 flush in device mode when in dedicated FIFO mode - 5'h1: -- Periodic TxFIFO flush in Host mode -- Periodic TxFIFO 1 flush in Device mode when in shared FIFO operation -- TXFIFO 1 flush in device mode when in dedicated FIFO mode - 5'h2: -- Periodic TxFIFO 2 flush in Device mode when in shared FIFO operation -- TXFIFO 2 flush in device mode when in dedicated FIFO mode ... - 5'hF -- Periodic TxFIFO 15 flush in Device mode when in shared FIFO operation -- TXFIFO 15 flush in device mode when in dedicated FIFO mode - 5'h10: Flush all the transmit FIFOs in device or host mode"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - Mode: Host and Device TxFIFO Number (TxFNum) This is the FIFO number that must be flushed using the TxFIFO Flush bit. This field must not be changed until the core clears the TxFIFO Flush bit. - 5'h0: -- Non-periodic TxFIFO flush in Host mode -- Non-periodic TxFIFO flush in device mode when in shared FIFO operation -- Tx FIFO 0 flush in device mode when in dedicated FIFO mode - 5'h1: -- Periodic TxFIFO flush in Host mode -- Periodic TxFIFO 1 flush in Device mode when in shared FIFO operation -- TXFIFO 1 flush in device mode when in dedicated FIFO mode - 5'h2: -- Periodic TxFIFO 2 flush in Device mode when in shared FIFO operation -- TXFIFO 2 flush in device mode when in dedicated FIFO mode ... - 5'hF -- Periodic TxFIFO 15 flush in Device mode when in shared FIFO operation -- TXFIFO 15 flush in device mode when in dedicated FIFO mode - 5'h10: Flush all the transmit FIFOs in device or host mode"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSFTRSTDONE` reader - Mode: Host and Device Core Soft Reset Done (CSftRstDone) The core sets this bit when all the necessary logic is reset in the core.This bit is cleared by the application along with GRSTCTL.CSftRst (bit 0)"]
pub type CSFTRSTDONE_R = crate::BitReader;
#[doc = "Field `CSFTRSTDONE` writer - Mode: Host and Device Core Soft Reset Done (CSftRstDone) The core sets this bit when all the necessary logic is reset in the core.This bit is cleared by the application along with GRSTCTL.CSftRst (bit 0)"]
pub type CSFTRSTDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQ` reader - Mode: Host and Device DMA Request Signal (DMAReq) Indicates that the DMA request is in progress. Used for debug."]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - Mode: Host and Device AHB Master Idle (AHBIdle) Indicates that the AHB Master State Machine is in the IDLE condition."]
pub type AHBIDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mode: Host and Device Core Soft Reset (CSftRst) Resets the hclk and phy_clock domains as follows: - Clears the interrupts and all the CSR registers except the following register bits: -- PCGCCTL.RstPdwnModule -- PCGCCTL.GateHclk -- PCGCCTL.PwrClmp -- PCGCCTL.StopPPhyLPwrClkSelclk -- GUSBCFG.ForceDevMode -- GUSBCFG.ForceHstMode -- GUSBCFG.PhyLPwrClkSel -- GUSBCFG.DDRSel -- GUSBCFG.PHYSel -- GUSBCFG.FSIntf -- GUSBCFG.ULPI_UTMI_Sel -- GUSBCFG.PHYIf -- GUSBCFG.TxEndDelay -- GUSBCFG.TermSelDLPulse -- GUSBCFG.ULPIClkSusM -- GUSBCFG.ULPIAutoRes -- GUSBCFG.ULPIFsLs -- GGPIO -- GPWRDN -- GADPCTL -- HCFG.FSLSPclkSel -- DCFG.DevSpd -- DCTL.SftDiscon - All module state machines - All module state machines (except the AHB Slave Unit) are reset to the IDLE state, and all the transmit FIFOs and the receive FIFO are flushed. - Any transactions on the AHB Master are terminated as soon as possible, after gracefully completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. - When Hibernation or ADP feature is enabled, the PMU module is not reset by the Core Soft Reset. The application can write to this bit any time it wants to reset the core. The application must clear this bit after checking the bit 29 of this register (Core Soft Reset Done). Software must also must check that bit 31 of this register is 1 (AHB Master is IDLE) before starting any operation. Typically software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation."]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode: Host and Device PIU FS Dedicated Controller Soft Reset (PIUFSSftRst) Resets the PIU FS Dedicated Controller All module state machines in FS Dedicated Controller of PIU are reset to the IDLE state. Used to reset the FS Dedicated controller in PIU in case of any PHY Errors like Loss of activity or Babble Error resulting in the PHY remaining in RX state for more than one frame boundary. This is a self clearing bit and core clears this bit after all the necessary logic is reset in the core."]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode: Host only Host Frame Counter Reset (FrmCntrRst) The application writes this bit to reset the (micro)Frame number counter inside the core. When the (micro)Frame counter is reset, the subsequent SOF sent out by the core has a (micro)Frame number of 0. When application writes 1 to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles."]
    #[inline(always)]
    pub fn frmcntrrst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode: Host and Device RxFIFO Flush (RxFFlsh) The application can flush the entire RxFIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the controller is neither reading from the RxFIFO nor writing to the RxFIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires eight clocks (slowest of PHY or AHB clock) to clear."]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode: Host and Device TxFIFO Flush (TxFFlsh) This bit selectively flushes a single or all transmit FIFOs, but cannot do so If the core is in the midst of a transaction. The application must write this bit only after checking that the core is neither writing to the TxFIFO nor reading from the TxFIFO. Verify using these registers: - ReadNAK Effective Interrupt ensures the core is not reading from the FIFO - WriteGRSTCTL.AHBIdle ensures the core is not writing anything to the FIFO. Flushing is normally recommended when FIFOs are reconfigured or when switching between Shared FIFO and Dedicated Transmit FIFO operation. FIFO flushing is also recommended during device endpoint disable. The application must wait until the core clears this bit before performing any operations. This bit takes eight clocks to clear, using the slower clock of phy_clk or hclk."]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Mode: Host and Device TxFIFO Number (TxFNum) This is the FIFO number that must be flushed using the TxFIFO Flush bit. This field must not be changed until the core clears the TxFIFO Flush bit. - 5'h0: -- Non-periodic TxFIFO flush in Host mode -- Non-periodic TxFIFO flush in device mode when in shared FIFO operation -- Tx FIFO 0 flush in device mode when in dedicated FIFO mode - 5'h1: -- Periodic TxFIFO flush in Host mode -- Periodic TxFIFO 1 flush in Device mode when in shared FIFO operation -- TXFIFO 1 flush in device mode when in dedicated FIFO mode - 5'h2: -- Periodic TxFIFO 2 flush in Device mode when in shared FIFO operation -- TXFIFO 2 flush in device mode when in dedicated FIFO mode ... - 5'hF -- Periodic TxFIFO 15 flush in Device mode when in shared FIFO operation -- TXFIFO 15 flush in device mode when in dedicated FIFO mode - 5'h10: Flush all the transmit FIFOs in device or host mode"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Mode: Host and Device Core Soft Reset Done (CSftRstDone) The core sets this bit when all the necessary logic is reset in the core.This bit is cleared by the application along with GRSTCTL.CSftRst (bit 0)"]
    #[inline(always)]
    pub fn csftrstdone(&self) -> CSFTRSTDONE_R {
        CSFTRSTDONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mode: Host and Device DMA Request Signal (DMAReq) Indicates that the DMA request is in progress. Used for debug."]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mode: Host and Device AHB Master Idle (AHBIdle) Indicates that the AHB Master State Machine is in the IDLE condition."]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csftrst", &self.csftrst())
            .field("piufssftrst", &self.piufssftrst())
            .field("frmcntrrst", &self.frmcntrrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("csftrstdone", &self.csftrstdone())
            .field("dmareq", &self.dmareq())
            .field("ahbidle", &self.ahbidle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Mode: Host and Device Core Soft Reset (CSftRst) Resets the hclk and phy_clock domains as follows: - Clears the interrupts and all the CSR registers except the following register bits: -- PCGCCTL.RstPdwnModule -- PCGCCTL.GateHclk -- PCGCCTL.PwrClmp -- PCGCCTL.StopPPhyLPwrClkSelclk -- GUSBCFG.ForceDevMode -- GUSBCFG.ForceHstMode -- GUSBCFG.PhyLPwrClkSel -- GUSBCFG.DDRSel -- GUSBCFG.PHYSel -- GUSBCFG.FSIntf -- GUSBCFG.ULPI_UTMI_Sel -- GUSBCFG.PHYIf -- GUSBCFG.TxEndDelay -- GUSBCFG.TermSelDLPulse -- GUSBCFG.ULPIClkSusM -- GUSBCFG.ULPIAutoRes -- GUSBCFG.ULPIFsLs -- GGPIO -- GPWRDN -- GADPCTL -- HCFG.FSLSPclkSel -- DCFG.DevSpd -- DCTL.SftDiscon - All module state machines - All module state machines (except the AHB Slave Unit) are reset to the IDLE state, and all the transmit FIFOs and the receive FIFO are flushed. - Any transactions on the AHB Master are terminated as soon as possible, after gracefully completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. - When Hibernation or ADP feature is enabled, the PMU module is not reset by the Core Soft Reset. The application can write to this bit any time it wants to reset the core. The application must clear this bit after checking the bit 29 of this register (Core Soft Reset Done). Software must also must check that bit 31 of this register is 1 (AHB Master is IDLE) before starting any operation. Typically software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation."]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CSFTRST_W<'_, GRSTCTL_SPEC> {
        CSFTRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mode: Host and Device PIU FS Dedicated Controller Soft Reset (PIUFSSftRst) Resets the PIU FS Dedicated Controller All module state machines in FS Dedicated Controller of PIU are reset to the IDLE state. Used to reset the FS Dedicated controller in PIU in case of any PHY Errors like Loss of activity or Babble Error resulting in the PHY remaining in RX state for more than one frame boundary. This is a self clearing bit and core clears this bit after all the necessary logic is reset in the core."]
    #[inline(always)]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W<'_, GRSTCTL_SPEC> {
        PIUFSSFTRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode: Host only Host Frame Counter Reset (FrmCntrRst) The application writes this bit to reset the (micro)Frame number counter inside the core. When the (micro)Frame counter is reset, the subsequent SOF sent out by the core has a (micro)Frame number of 0. When application writes 1 to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles."]
    #[inline(always)]
    pub fn frmcntrrst(&mut self) -> FRMCNTRRST_W<'_, GRSTCTL_SPEC> {
        FRMCNTRRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - Mode: Host and Device RxFIFO Flush (RxFFlsh) The application can flush the entire RxFIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the controller is neither reading from the RxFIFO nor writing to the RxFIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires eight clocks (slowest of PHY or AHB clock) to clear."]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<'_, GRSTCTL_SPEC> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mode: Host and Device TxFIFO Flush (TxFFlsh) This bit selectively flushes a single or all transmit FIFOs, but cannot do so If the core is in the midst of a transaction. The application must write this bit only after checking that the core is neither writing to the TxFIFO nor reading from the TxFIFO. Verify using these registers: - ReadNAK Effective Interrupt ensures the core is not reading from the FIFO - WriteGRSTCTL.AHBIdle ensures the core is not writing anything to the FIFO. Flushing is normally recommended when FIFOs are reconfigured or when switching between Shared FIFO and Dedicated Transmit FIFO operation. FIFO flushing is also recommended during device endpoint disable. The application must wait until the core clears this bit before performing any operations. This bit takes eight clocks to clear, using the slower clock of phy_clk or hclk."]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<'_, GRSTCTL_SPEC> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - Mode: Host and Device TxFIFO Number (TxFNum) This is the FIFO number that must be flushed using the TxFIFO Flush bit. This field must not be changed until the core clears the TxFIFO Flush bit. - 5'h0: -- Non-periodic TxFIFO flush in Host mode -- Non-periodic TxFIFO flush in device mode when in shared FIFO operation -- Tx FIFO 0 flush in device mode when in dedicated FIFO mode - 5'h1: -- Periodic TxFIFO flush in Host mode -- Periodic TxFIFO 1 flush in Device mode when in shared FIFO operation -- TXFIFO 1 flush in device mode when in dedicated FIFO mode - 5'h2: -- Periodic TxFIFO 2 flush in Device mode when in shared FIFO operation -- TXFIFO 2 flush in device mode when in dedicated FIFO mode ... - 5'hF -- Periodic TxFIFO 15 flush in Device mode when in shared FIFO operation -- TXFIFO 15 flush in device mode when in dedicated FIFO mode - 5'h10: Flush all the transmit FIFOs in device or host mode"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, GRSTCTL_SPEC> {
        TXFNUM_W::new(self, 6)
    }
    #[doc = "Bit 29 - Mode: Host and Device Core Soft Reset Done (CSftRstDone) The core sets this bit when all the necessary logic is reset in the core.This bit is cleared by the application along with GRSTCTL.CSftRst (bit 0)"]
    #[inline(always)]
    pub fn csftrstdone(&mut self) -> CSFTRSTDONE_W<'_, GRSTCTL_SPEC> {
        CSFTRSTDONE_W::new(self, 29)
    }
}
#[doc = "The application uses this register to reset various hardware features inside the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
