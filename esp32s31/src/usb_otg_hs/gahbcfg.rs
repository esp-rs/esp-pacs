#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GLBLINTRMSK` reader - Mode: Host and device Global Interrupt Mask (GlblIntrMsk) The application uses this bit to mask or unmask the interrupt line assertion to itself. Irrespective of this bit's setting, the interrupt status registers are updated by the controller."]
pub type GLBLINTRMSK_R = crate::BitReader;
#[doc = "Field `GLBLINTRMSK` writer - Mode: Host and device Global Interrupt Mask (GlblIntrMsk) The application uses this bit to mask or unmask the interrupt line assertion to itself. Irrespective of this bit's setting, the interrupt status registers are updated by the controller."]
pub type GLBLINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - Mode: Host and device Burst Length/Type (HBstLen) This field is used in both External and Internal DMA modes. In External DMA mode, these bits appear on dma_burst\\[3:0\\] ports, which can be used by an external wrapper to interface the External DMA Controller interface to Synopsys DW_ahb_dmac or ARM PrimeCell. External DMA Mode defines the DMA burst length in terms of 32-bit words: - 4'b0000: 1 word - 4'b0001: 4 words - 4'b0010: 8 words - 4'b0011: 16 words - 4'b0100: 32 words - 4'b0101: 64 words - 4'b0110: 128 words - 4'b0111: 256 words - Others: Reserved Internal DMA Mode AHB Master burst type: - 4'b0000 Single - 4'b0001 INCR - 4'b0011 INCR4 - 4'b0101 INCR8 - 4'b0111 INCR16 - Others: Reserved"]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - Mode: Host and device Burst Length/Type (HBstLen) This field is used in both External and Internal DMA modes. In External DMA mode, these bits appear on dma_burst\\[3:0\\] ports, which can be used by an external wrapper to interface the External DMA Controller interface to Synopsys DW_ahb_dmac or ARM PrimeCell. External DMA Mode defines the DMA burst length in terms of 32-bit words: - 4'b0000: 1 word - 4'b0001: 4 words - 4'b0010: 8 words - 4'b0011: 16 words - 4'b0100: 32 words - 4'b0101: 64 words - 4'b0110: 128 words - 4'b0111: 256 words - Others: Reserved Internal DMA Mode AHB Master burst type: - 4'b0000 Single - 4'b0001 INCR - 4'b0011 INCR4 - 4'b0101 INCR8 - 4'b0111 INCR16 - Others: Reserved"]
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - Mode: Host and device DMA Enable (DMAEn) This bit is always 0 when Slave-Only mode has been selected. Reset: 1'b0"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - Mode: Host and device DMA Enable (DMAEn) This bit is always 0 when Slave-Only mode has been selected. Reset: 1'b0"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - Mode: Host and device Non-Periodic TxFIFO Empty Level (NPTxFEmpLvl) This bit is used only in Slave mode. In host mode and with Shared FIFO with device mode, this bit indicates when the Non-Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.NPTxFEmp) is triggered. With dedicated FIFO in device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (DIEPINTn.TxFEmp) is triggered. Host mode and with Shared FIFO with device mode: - 1'b0: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is half empty - 1'b1: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is completely empty Dedicated FIFO in device mode: - 1'b0: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty - 1'b1: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
pub type NPTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Mode: Host and device Non-Periodic TxFIFO Empty Level (NPTxFEmpLvl) This bit is used only in Slave mode. In host mode and with Shared FIFO with device mode, this bit indicates when the Non-Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.NPTxFEmp) is triggered. With dedicated FIFO in device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (DIEPINTn.TxFEmp) is triggered. Host mode and with Shared FIFO with device mode: - 1'b0: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is half empty - 1'b1: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is completely empty Dedicated FIFO in device mode: - 1'b0: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty - 1'b1: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
pub type NPTXFEMPLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPLVL` reader - Mode: Host only Periodic TxFIFO Empty Level (PTxFEmpLvl) Indicates when the Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.PTxFEmp) is triggered. This bit is used only in Slave mode. - 1'b0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty - 1'b1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
pub type PTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `PTXFEMPLVL` writer - Mode: Host only Periodic TxFIFO Empty Level (PTxFEmpLvl) Indicates when the Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.PTxFEmp) is triggered. This bit is used only in Slave mode. - 1'b0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty - 1'b1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
pub type PTXFEMPLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMMEMSUPP` reader - Mode: Host and Device Remote Memory Support (RemMemSupp) This bit is programmed to enable the functionality to wait for the system DMA Done Signal for the DMA Write Transfers. - GAHBCFG.RemMemSupp=1 The int_dma_req output signal is asserted when the DMA starts write transfer to the external memory. When the core is done with the Transfers it asserts int_dma_done signal to flag the completion of DMA writes from the controller. The core then waits for sys_dma_done signal from the system to proceed further and complete the Data Transfer corresponding to a particular Channel/Endpoint. - GAHBCFG.RemMemSupp=0 The int_dma_req and int_dma_done signals are not asserted and the core proceeds with the assertion of the XferComp interrupt as soon as the DMA write transfer is done at the Core Boundary and it does not wait for the sys_dma_done signal to complete the DATA transfers."]
pub type REMMEMSUPP_R = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - Mode: Host and Device Remote Memory Support (RemMemSupp) This bit is programmed to enable the functionality to wait for the system DMA Done Signal for the DMA Write Transfers. - GAHBCFG.RemMemSupp=1 The int_dma_req output signal is asserted when the DMA starts write transfer to the external memory. When the core is done with the Transfers it asserts int_dma_done signal to flag the completion of DMA writes from the controller. The core then waits for sys_dma_done signal from the system to proceed further and complete the Data Transfer corresponding to a particular Channel/Endpoint. - GAHBCFG.RemMemSupp=0 The int_dma_req and int_dma_done signals are not asserted and the core proceeds with the assertion of the XferComp interrupt as soon as the DMA write transfer is done at the Core Boundary and it does not wait for the sys_dma_done signal to complete the DATA transfers."]
pub type REMMEMSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTIALLDMAWRIT` reader - Mode: Host and Device Notify All DMA Write Transactions (NotiAllDmaWrit) This bit is programmed to enable the System DMA Done functionality for all the DMA write Transactions corresponding to the Channel/Endpoint. This bit is valid only when GAHBCFG.RemMemSupp is set to 1. - GAHBCFG.NotiAllDmaWrit = 1 The core asserts int_dma_req for all the DMA write transactions on the AHB interface along with int_dma_done, chep_last_transact and chep_number signal informations. The core waits for sys_dma_done signal for all the DMA write transactions in order to complete the transfer of a particular Channel/Endpoint. - GAHBCFG.NotiAllDmaWrit = 0 The core asserts int_dma_req signal only for the last transaction of DMA write transfer corresponding to a particular Channel/Endpoint. Similarly, the core waits for sys_dma_done signal only for that transaction of DMA write to complete the transfer of a particular Channel/Endpoint."]
pub type NOTIALLDMAWRIT_R = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - Mode: Host and Device Notify All DMA Write Transactions (NotiAllDmaWrit) This bit is programmed to enable the System DMA Done functionality for all the DMA write Transactions corresponding to the Channel/Endpoint. This bit is valid only when GAHBCFG.RemMemSupp is set to 1. - GAHBCFG.NotiAllDmaWrit = 1 The core asserts int_dma_req for all the DMA write transactions on the AHB interface along with int_dma_done, chep_last_transact and chep_number signal informations. The core waits for sys_dma_done signal for all the DMA write transactions in order to complete the transfer of a particular Channel/Endpoint. - GAHBCFG.NotiAllDmaWrit = 0 The core asserts int_dma_req signal only for the last transaction of DMA write transfer corresponding to a particular Channel/Endpoint. Similarly, the core waits for sys_dma_done signal only for that transaction of DMA write to complete the transfer of a particular Channel/Endpoint."]
pub type NOTIALLDMAWRIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSINGLE` reader - Mode: Host and Device AHB Single Support (AHBSingle) This bit when programmed supports Single transfers for the remaining data in a transfer when the core is operating in DMA mode. - 1'b0: The remaining data in the transfer is sent using INCR burst size. - 1'b1: The remaining data in the transfer is sent using Single burst size. Note: If this feature is enabled, the AHB RETRY and SPLIT transfers still have INCR burst type. Enable this feature when the AHB Slave connected to the core does not support INCR burst (and when Split, and Retry transactions are not being used in the bus)."]
pub type AHBSINGLE_R = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - Mode: Host and Device AHB Single Support (AHBSingle) This bit when programmed supports Single transfers for the remaining data in a transfer when the core is operating in DMA mode. - 1'b0: The remaining data in the transfer is sent using INCR burst size. - 1'b1: The remaining data in the transfer is sent using Single burst size. Note: If this feature is enabled, the AHB RETRY and SPLIT transfers still have INCR burst type. Enable this feature when the AHB Slave connected to the core does not support INCR burst (and when Split, and Retry transactions are not being used in the bus)."]
pub type AHBSINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDESCENDIANESS` reader - Mode: Host and Device Invert Descriptor Endianess (InvDescEndianess) - 1'b0: Descriptor Endianness is same as AHB Master Endianness. - 1'b1: -- If the AHB Master endianness is Big Endian, the Descriptor Endianness is Little Endian. -- If the AHB Master endianness is Little Endian, the Descriptor Endianness is Big Endian."]
pub type INVDESCENDIANESS_R = crate::BitReader;
#[doc = "Field `INVDESCENDIANESS` writer - Mode: Host and Device Invert Descriptor Endianess (InvDescEndianess) - 1'b0: Descriptor Endianness is same as AHB Master Endianness. - 1'b1: -- If the AHB Master endianness is Big Endian, the Descriptor Endianness is Little Endian. -- If the AHB Master endianness is Little Endian, the Descriptor Endianness is Big Endian."]
pub type INVDESCENDIANESS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode: Host and device Global Interrupt Mask (GlblIntrMsk) The application uses this bit to mask or unmask the interrupt line assertion to itself. Irrespective of this bit's setting, the interrupt status registers are updated by the controller."]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GLBLINTRMSK_R {
        GLBLINTRMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Mode: Host and device Burst Length/Type (HBstLen) This field is used in both External and Internal DMA modes. In External DMA mode, these bits appear on dma_burst\\[3:0\\] ports, which can be used by an external wrapper to interface the External DMA Controller interface to Synopsys DW_ahb_dmac or ARM PrimeCell. External DMA Mode defines the DMA burst length in terms of 32-bit words: - 4'b0000: 1 word - 4'b0001: 4 words - 4'b0010: 8 words - 4'b0011: 16 words - 4'b0100: 32 words - 4'b0101: 64 words - 4'b0110: 128 words - 4'b0111: 256 words - Others: Reserved Internal DMA Mode AHB Master burst type: - 4'b0000 Single - 4'b0001 INCR - 4'b0011 INCR4 - 4'b0101 INCR8 - 4'b0111 INCR16 - Others: Reserved"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Mode: Host and device DMA Enable (DMAEn) This bit is always 0 when Slave-Only mode has been selected. Reset: 1'b0"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Mode: Host and device Non-Periodic TxFIFO Empty Level (NPTxFEmpLvl) This bit is used only in Slave mode. In host mode and with Shared FIFO with device mode, this bit indicates when the Non-Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.NPTxFEmp) is triggered. With dedicated FIFO in device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (DIEPINTn.TxFEmp) is triggered. Host mode and with Shared FIFO with device mode: - 1'b0: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is half empty - 1'b1: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is completely empty Dedicated FIFO in device mode: - 1'b0: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty - 1'b1: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mode: Host only Periodic TxFIFO Empty Level (PTxFEmpLvl) Indicates when the Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.PTxFEmp) is triggered. This bit is used only in Slave mode. - 1'b0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty - 1'b1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode: Host and Device Remote Memory Support (RemMemSupp) This bit is programmed to enable the functionality to wait for the system DMA Done Signal for the DMA Write Transfers. - GAHBCFG.RemMemSupp=1 The int_dma_req output signal is asserted when the DMA starts write transfer to the external memory. When the core is done with the Transfers it asserts int_dma_done signal to flag the completion of DMA writes from the controller. The core then waits for sys_dma_done signal from the system to proceed further and complete the Data Transfer corresponding to a particular Channel/Endpoint. - GAHBCFG.RemMemSupp=0 The int_dma_req and int_dma_done signals are not asserted and the core proceeds with the assertion of the XferComp interrupt as soon as the DMA write transfer is done at the Core Boundary and it does not wait for the sys_dma_done signal to complete the DATA transfers."]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mode: Host and Device Notify All DMA Write Transactions (NotiAllDmaWrit) This bit is programmed to enable the System DMA Done functionality for all the DMA write Transactions corresponding to the Channel/Endpoint. This bit is valid only when GAHBCFG.RemMemSupp is set to 1. - GAHBCFG.NotiAllDmaWrit = 1 The core asserts int_dma_req for all the DMA write transactions on the AHB interface along with int_dma_done, chep_last_transact and chep_number signal informations. The core waits for sys_dma_done signal for all the DMA write transactions in order to complete the transfer of a particular Channel/Endpoint. - GAHBCFG.NotiAllDmaWrit = 0 The core asserts int_dma_req signal only for the last transaction of DMA write transfer corresponding to a particular Channel/Endpoint. Similarly, the core waits for sys_dma_done signal only for that transaction of DMA write to complete the transfer of a particular Channel/Endpoint."]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mode: Host and Device AHB Single Support (AHBSingle) This bit when programmed supports Single transfers for the remaining data in a transfer when the core is operating in DMA mode. - 1'b0: The remaining data in the transfer is sent using INCR burst size. - 1'b1: The remaining data in the transfer is sent using Single burst size. Note: If this feature is enabled, the AHB RETRY and SPLIT transfers still have INCR burst type. Enable this feature when the AHB Slave connected to the core does not support INCR burst (and when Split, and Retry transactions are not being used in the bus)."]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mode: Host and Device Invert Descriptor Endianess (InvDescEndianess) - 1'b0: Descriptor Endianness is same as AHB Master Endianness. - 1'b1: -- If the AHB Master endianness is Big Endian, the Descriptor Endianness is Little Endian. -- If the AHB Master endianness is Little Endian, the Descriptor Endianness is Big Endian."]
    #[inline(always)]
    pub fn invdescendianess(&self) -> INVDESCENDIANESS_R {
        INVDESCENDIANESS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("glblintrmsk", &self.glblintrmsk())
            .field("hbstlen", &self.hbstlen())
            .field("dmaen", &self.dmaen())
            .field("nptxfemplvl", &self.nptxfemplvl())
            .field("ptxfemplvl", &self.ptxfemplvl())
            .field("remmemsupp", &self.remmemsupp())
            .field("notialldmawrit", &self.notialldmawrit())
            .field("ahbsingle", &self.ahbsingle())
            .field("invdescendianess", &self.invdescendianess())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Mode: Host and device Global Interrupt Mask (GlblIntrMsk) The application uses this bit to mask or unmask the interrupt line assertion to itself. Irrespective of this bit's setting, the interrupt status registers are updated by the controller."]
    #[inline(always)]
    pub fn glblintrmsk(&mut self) -> GLBLINTRMSK_W<'_, GAHBCFG_SPEC> {
        GLBLINTRMSK_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Mode: Host and device Burst Length/Type (HBstLen) This field is used in both External and Internal DMA modes. In External DMA mode, these bits appear on dma_burst\\[3:0\\] ports, which can be used by an external wrapper to interface the External DMA Controller interface to Synopsys DW_ahb_dmac or ARM PrimeCell. External DMA Mode defines the DMA burst length in terms of 32-bit words: - 4'b0000: 1 word - 4'b0001: 4 words - 4'b0010: 8 words - 4'b0011: 16 words - 4'b0100: 32 words - 4'b0101: 64 words - 4'b0110: 128 words - 4'b0111: 256 words - Others: Reserved Internal DMA Mode AHB Master burst type: - 4'b0000 Single - 4'b0001 INCR - 4'b0011 INCR4 - 4'b0101 INCR8 - 4'b0111 INCR16 - Others: Reserved"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<'_, GAHBCFG_SPEC> {
        HBSTLEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - Mode: Host and device DMA Enable (DMAEn) This bit is always 0 when Slave-Only mode has been selected. Reset: 1'b0"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, GAHBCFG_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Mode: Host and device Non-Periodic TxFIFO Empty Level (NPTxFEmpLvl) This bit is used only in Slave mode. In host mode and with Shared FIFO with device mode, this bit indicates when the Non-Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.NPTxFEmp) is triggered. With dedicated FIFO in device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (DIEPINTn.TxFEmp) is triggered. Host mode and with Shared FIFO with device mode: - 1'b0: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is half empty - 1'b1: GINTSTS.NPTxFEmp interrupt indicates that the Non-Periodic TxFIFO is completely empty Dedicated FIFO in device mode: - 1'b0: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty - 1'b1: DIEPINTn.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<'_, GAHBCFG_SPEC> {
        NPTXFEMPLVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Mode: Host only Periodic TxFIFO Empty Level (PTxFEmpLvl) Indicates when the Periodic TxFIFO Empty Interrupt bit in the Core Interrupt register (GINTSTS.PTxFEmp) is triggered. This bit is used only in Slave mode. - 1'b0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty - 1'b1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W<'_, GAHBCFG_SPEC> {
        PTXFEMPLVL_W::new(self, 8)
    }
    #[doc = "Bit 21 - Mode: Host and Device Remote Memory Support (RemMemSupp) This bit is programmed to enable the functionality to wait for the system DMA Done Signal for the DMA Write Transfers. - GAHBCFG.RemMemSupp=1 The int_dma_req output signal is asserted when the DMA starts write transfer to the external memory. When the core is done with the Transfers it asserts int_dma_done signal to flag the completion of DMA writes from the controller. The core then waits for sys_dma_done signal from the system to proceed further and complete the Data Transfer corresponding to a particular Channel/Endpoint. - GAHBCFG.RemMemSupp=0 The int_dma_req and int_dma_done signals are not asserted and the core proceeds with the assertion of the XferComp interrupt as soon as the DMA write transfer is done at the Core Boundary and it does not wait for the sys_dma_done signal to complete the DATA transfers."]
    #[inline(always)]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W<'_, GAHBCFG_SPEC> {
        REMMEMSUPP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Mode: Host and Device Notify All DMA Write Transactions (NotiAllDmaWrit) This bit is programmed to enable the System DMA Done functionality for all the DMA write Transactions corresponding to the Channel/Endpoint. This bit is valid only when GAHBCFG.RemMemSupp is set to 1. - GAHBCFG.NotiAllDmaWrit = 1 The core asserts int_dma_req for all the DMA write transactions on the AHB interface along with int_dma_done, chep_last_transact and chep_number signal informations. The core waits for sys_dma_done signal for all the DMA write transactions in order to complete the transfer of a particular Channel/Endpoint. - GAHBCFG.NotiAllDmaWrit = 0 The core asserts int_dma_req signal only for the last transaction of DMA write transfer corresponding to a particular Channel/Endpoint. Similarly, the core waits for sys_dma_done signal only for that transaction of DMA write to complete the transfer of a particular Channel/Endpoint."]
    #[inline(always)]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W<'_, GAHBCFG_SPEC> {
        NOTIALLDMAWRIT_W::new(self, 22)
    }
    #[doc = "Bit 23 - Mode: Host and Device AHB Single Support (AHBSingle) This bit when programmed supports Single transfers for the remaining data in a transfer when the core is operating in DMA mode. - 1'b0: The remaining data in the transfer is sent using INCR burst size. - 1'b1: The remaining data in the transfer is sent using Single burst size. Note: If this feature is enabled, the AHB RETRY and SPLIT transfers still have INCR burst type. Enable this feature when the AHB Slave connected to the core does not support INCR burst (and when Split, and Retry transactions are not being used in the bus)."]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W<'_, GAHBCFG_SPEC> {
        AHBSINGLE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Mode: Host and Device Invert Descriptor Endianess (InvDescEndianess) - 1'b0: Descriptor Endianness is same as AHB Master Endianness. - 1'b1: -- If the AHB Master endianness is Big Endian, the Descriptor Endianness is Little Endian. -- If the AHB Master endianness is Little Endian, the Descriptor Endianness is Big Endian."]
    #[inline(always)]
    pub fn invdescendianess(&mut self) -> INVDESCENDIANESS_W<'_, GAHBCFG_SPEC> {
        INVDESCENDIANESS_W::new(self, 24)
    }
}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {}
