#[doc = "Register `REGISTER69_CHANNEL1STATUSREGISTER` reader"]
pub type R = crate::R<REGISTER69_CHANNEL1STATUSREGISTER_SPEC>;
#[doc = "Register `REGISTER69_CHANNEL1STATUSREGISTER` writer"]
pub type W = crate::W<REGISTER69_CHANNEL1STATUSREGISTER_SPEC>;
#[doc = "Field `CH1_TI` reader - Transmit Interrupt This bit indicates that the frame transmission is complete When transmission is complete, Bit 31 _OWN_ of TDES0 is reset, and the specific frame status information is updated in the descriptor"]
pub type CH1_TI_R = crate::BitReader;
#[doc = "Field `CH1_TI` writer - Transmit Interrupt This bit indicates that the frame transmission is complete When transmission is complete, Bit 31 _OWN_ of TDES0 is reset, and the specific frame status information is updated in the descriptor"]
pub type CH1_TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TPS` reader - Transmit Process Stopped This bit is set when the transmission is stopped"]
pub type CH1_TPS_R = crate::BitReader;
#[doc = "Field `CH1_TPS` writer - Transmit Process Stopped This bit is set when the transmission is stopped"]
pub type CH1_TPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TU` reader - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it Transmission is suspended Bits\\[22:20\\] explain the Transmit Process state transitions To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand command"]
pub type CH1_TU_R = crate::BitReader;
#[doc = "Field `CH1_TU` writer - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it Transmission is suspended Bits\\[22:20\\] explain the Transmit Process state transitions To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand command"]
pub type CH1_TU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TJT` reader - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 _10,240 bytes when the Jumbo frame is enabled_ When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert"]
pub type CH1_TJT_R = crate::BitReader;
#[doc = "Field `CH1_TJT` writer - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 _10,240 bytes when the Jumbo frame is enabled_ When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert"]
pub type CH1_TJT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVF` reader - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]"]
pub type CH1_OVF_R = crate::BitReader;
#[doc = "Field `CH1_OVF` writer - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]"]
pub type CH1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNF` reader - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission Transmission is suspended and an Underflow Error TDES0\\[1\\] is set"]
pub type CH1_UNF_R = crate::BitReader;
#[doc = "Field `CH1_UNF` writer - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission Transmission is suspended and an Underflow Error TDES0\\[1\\] is set"]
pub type CH1_UNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RI` reader - Receive Interrupt This bit indicates that the frame reception is complete When reception is complete, the Bit 31 of RDES1 _Disable Interrupt on Completion_ is reset in the last Descriptor, and the specific frame status information is updated in the descriptor The reception remains in the Running state"]
pub type CH1_RI_R = crate::BitReader;
#[doc = "Field `CH1_RI` writer - Receive Interrupt This bit indicates that the frame reception is complete When reception is complete, the Bit 31 of RDES1 _Disable Interrupt on Completion_ is reset in the last Descriptor, and the specific frame status information is updated in the descriptor The reception remains in the Running state"]
pub type CH1_RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RU` reader - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it The Receive Process is suspended To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received This bit is set only when the previous Receive Descriptor is owned by the DMA"]
pub type CH1_RU_R = crate::BitReader;
#[doc = "Field `CH1_RU` writer - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it The Receive Process is suspended To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received This bit is set only when the previous Receive Descriptor is owned by the DMA"]
pub type CH1_RU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RPS` reader - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state"]
pub type CH1_RPS_R = crate::BitReader;
#[doc = "Field `CH1_RPS` writer - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state"]
pub type CH1_RPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RWT` reader - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout"]
pub type CH1_RWT_R = crate::BitReader;
#[doc = "Field `CH1_RWT` writer - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout"]
pub type CH1_RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ETI` reader - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO"]
pub type CH1_ETI_R = crate::BitReader;
#[doc = "Field `CH1_ETI` writer - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO"]
pub type CH1_ETI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_FBI` reader - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\] When this bit is set, the corresponding DMA engine disables all of its bus accesses 12:11 Reserved 00 RO"]
pub type CH1_FBI_R = crate::BitReader;
#[doc = "Field `CH1_FBI` writer - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\] When this bit is set, the corresponding DMA engine disables all of its bus accesses 12:11 Reserved 00 RO"]
pub type CH1_FBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ERI` reader - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet This bit is cleared when the software writes 1 to this bit or Bit 6 _RI_ of this register is set _whichever occurs earlier_"]
pub type CH1_ERI_R = crate::BitReader;
#[doc = "Field `CH1_ERI` writer - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet This bit is cleared when the software writes 1 to this bit or Bit 6 _RI_ of this register is set _whichever occurs earlier_"]
pub type CH1_ERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_AIS` reader - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive FIFO Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes AIS to be set, is cleared"]
pub type CH1_AIS_R = crate::BitReader;
#[doc = "Field `CH1_AIS` writer - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive FIFO Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes AIS to be set, is cleared"]
pub type CH1_AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_NIS` reader - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits _interrupts for which interrupt enable is set in Register 7_ affect the Normal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes NIS to be set, is cleared"]
pub type CH1_NIS_R = crate::BitReader;
#[doc = "Field `CH1_NIS` writer - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits _interrupts for which interrupt enable is set in Register 7_ affect the Normal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes NIS to be set, is cleared"]
pub type CH1_NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_RS` reader - Receive Process State This field indicates the Receive DMA FSM state This field does not generate an interrupt 3’b000: Stopped: Reset or Stop Receive Command issued 3’b001: Running: Fetching Receive Transfer Descriptor 3’b010: Reserved for future use 3’b011: Running: Waiting for receive packet 3’b100: Suspended: Receive Descriptor Unavailable 3’b101: Running: Closing Receive Descriptor 3’b110: TIME_STAMP write state 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
pub type CH1_RS_R = crate::FieldReader;
#[doc = "Field `CH1_TS` reader - Transmit Process State This field indicates the Transmit DMA FSM state This field does not generate an interrupt 3’b000: Stopped: Reset or Stop Transmit Command issued 3’b001: Running: Fetching Transmit Transfer Descriptor 3’b010: Running: Waiting for status 3’b011: Running: Reading Data from host memory buffer and queuing it to transmit buffer _Tx FIFO_ 3’b100: TIME_STAMP write state 3’b101: Reserved for future use 3’b110: Suspended: Transmit Descriptor Unavailable or Transmit Buffer Underflow 3’b111: Running: Closing Transmit Descriptor"]
pub type CH1_TS_R = crate::FieldReader;
#[doc = "Field `CH1_EB` reader - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface This field is valid only when Bit 13 _FBI_ is set This field does not generate an interrupt 0 0 0: Error during Rx DMA Write Data Transfer 0 1 1: Error during Tx DMA Read Data Transfer 1 0 0: Error during Rx DMA Descriptor Write Access 1 0 1: Error during Tx DMA Descriptor Write Access 1 1 0: Error during Rx DMA Descriptor Read Access 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved"]
pub type CH1_EB_R = crate::FieldReader;
#[doc = "Field `CH1_GLI` reader - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces _if present and enabled in your configuration_: PCS _TBI, RTBI, or SGMII_: Link change or autonegotiation complete event SMII or RGMII: Link change event General Purpose Input Status _GPIS_: Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\] of Register 14 _Interrupt Status Register_ and then to clear the source of interrupt _which also clears the GLI interrupt_, read any of the following corresponding registers: PCS _TBI, RTBI, or SGMII_: Register 49 _AN Status Register_ SMII or RGMII: Register 54 _SGMII/RGMII/SMII Control and Status Register_ General Purpose Input _GPI_: Register 56 _General Purpose IO Register_ The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high"]
pub type CH1_GLI_R = crate::BitReader;
#[doc = "Field `CH1_GMI` reader - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0 The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high This bit is applicable only when the MAC Management Counters _MMC_ are enabled Otherwise, this bit is reserved"]
pub type CH1_GMI_R = crate::BitReader;
#[doc = "Field `CH1_GPI` reader - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0 The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high This bit is applicable only when the Power Management feature is enabled Otherwise, this bit is reserved Note: The GPI and pmt_intr_o interrupts are generated in different clock domains"]
pub type CH1_GPI_R = crate::BitReader;
#[doc = "Field `CH1_TTI` reader - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0 When this bit is high, the interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high This bit is applicable only when the IEEE 1588 Timestamp feature is enabled Otherwise, this bit is reserved"]
pub type CH1_TTI_R = crate::BitReader;
#[doc = "Field `CH1_GLPII_GTMSI` reader - GTMSI: GMAC TMS Interrupt _for Channel 1 and Channel 2_ This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac To reset this bit, the software must read the corresponding registers _Channel Status Register_ to get the exact cause of the interrupt and clear its source Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present Otherwise, this bit is reserved When this bit is high, the interrupt signal from the MAC _sbd_intr_o_ is high"]
pub type CH1_GLPII_GTMSI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the frame transmission is complete When transmission is complete, Bit 31 _OWN_ of TDES0 is reset, and the specific frame status information is updated in the descriptor"]
    #[inline(always)]
    pub fn ch1_ti(&self) -> CH1_TI_R {
        CH1_TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped"]
    #[inline(always)]
    pub fn ch1_tps(&self) -> CH1_TPS_R {
        CH1_TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it Transmission is suspended Bits\\[22:20\\] explain the Transmit Process state transitions To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand command"]
    #[inline(always)]
    pub fn ch1_tu(&self) -> CH1_TU_R {
        CH1_TU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 _10,240 bytes when the Jumbo frame is enabled_ When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert"]
    #[inline(always)]
    pub fn ch1_tjt(&self) -> CH1_TJT_R {
        CH1_TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]"]
    #[inline(always)]
    pub fn ch1_ovf(&self) -> CH1_OVF_R {
        CH1_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission Transmission is suspended and an Underflow Error TDES0\\[1\\] is set"]
    #[inline(always)]
    pub fn ch1_unf(&self) -> CH1_UNF_R {
        CH1_UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the frame reception is complete When reception is complete, the Bit 31 of RDES1 _Disable Interrupt on Completion_ is reset in the last Descriptor, and the specific frame status information is updated in the descriptor The reception remains in the Running state"]
    #[inline(always)]
    pub fn ch1_ri(&self) -> CH1_RI_R {
        CH1_RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it The Receive Process is suspended To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received This bit is set only when the previous Receive Descriptor is owned by the DMA"]
    #[inline(always)]
    pub fn ch1_ru(&self) -> CH1_RU_R {
        CH1_RU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state"]
    #[inline(always)]
    pub fn ch1_rps(&self) -> CH1_RPS_R {
        CH1_RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout"]
    #[inline(always)]
    pub fn ch1_rwt(&self) -> CH1_RWT_R {
        CH1_RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO"]
    #[inline(always)]
    pub fn ch1_eti(&self) -> CH1_ETI_R {
        CH1_ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\] When this bit is set, the corresponding DMA engine disables all of its bus accesses 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn ch1_fbi(&self) -> CH1_FBI_R {
        CH1_FBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet This bit is cleared when the software writes 1 to this bit or Bit 6 _RI_ of this register is set _whichever occurs earlier_"]
    #[inline(always)]
    pub fn ch1_eri(&self) -> CH1_ERI_R {
        CH1_ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive FIFO Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes AIS to be set, is cleared"]
    #[inline(always)]
    pub fn ch1_ais(&self) -> CH1_AIS_R {
        CH1_AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits _interrupts for which interrupt enable is set in Register 7_ affect the Normal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes NIS to be set, is cleared"]
    #[inline(always)]
    pub fn ch1_nis(&self) -> CH1_NIS_R {
        CH1_NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State This field indicates the Receive DMA FSM state This field does not generate an interrupt 3’b000: Stopped: Reset or Stop Receive Command issued 3’b001: Running: Fetching Receive Transfer Descriptor 3’b010: Reserved for future use 3’b011: Running: Waiting for receive packet 3’b100: Suspended: Receive Descriptor Unavailable 3’b101: Running: Closing Receive Descriptor 3’b110: TIME_STAMP write state 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn ch1_rs(&self) -> CH1_RS_R {
        CH1_RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State This field indicates the Transmit DMA FSM state This field does not generate an interrupt 3’b000: Stopped: Reset or Stop Transmit Command issued 3’b001: Running: Fetching Transmit Transfer Descriptor 3’b010: Running: Waiting for status 3’b011: Running: Reading Data from host memory buffer and queuing it to transmit buffer _Tx FIFO_ 3’b100: TIME_STAMP write state 3’b101: Reserved for future use 3’b110: Suspended: Transmit Descriptor Unavailable or Transmit Buffer Underflow 3’b111: Running: Closing Transmit Descriptor"]
    #[inline(always)]
    pub fn ch1_ts(&self) -> CH1_TS_R {
        CH1_TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface This field is valid only when Bit 13 _FBI_ is set This field does not generate an interrupt 0 0 0: Error during Rx DMA Write Data Transfer 0 1 1: Error during Tx DMA Read Data Transfer 1 0 0: Error during Rx DMA Descriptor Write Access 1 0 1: Error during Tx DMA Descriptor Write Access 1 1 0: Error during Rx DMA Descriptor Read Access 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved"]
    #[inline(always)]
    pub fn ch1_eb(&self) -> CH1_EB_R {
        CH1_EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces _if present and enabled in your configuration_: PCS _TBI, RTBI, or SGMII_: Link change or autonegotiation complete event SMII or RGMII: Link change event General Purpose Input Status _GPIS_: Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\] of Register 14 _Interrupt Status Register_ and then to clear the source of interrupt _which also clears the GLI interrupt_, read any of the following corresponding registers: PCS _TBI, RTBI, or SGMII_: Register 49 _AN Status Register_ SMII or RGMII: Register 54 _SGMII/RGMII/SMII Control and Status Register_ General Purpose Input _GPI_: Register 56 _General Purpose IO Register_ The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high"]
    #[inline(always)]
    pub fn ch1_gli(&self) -> CH1_GLI_R {
        CH1_GLI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0 The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high This bit is applicable only when the MAC Management Counters _MMC_ are enabled Otherwise, this bit is reserved"]
    #[inline(always)]
    pub fn ch1_gmi(&self) -> CH1_GMI_R {
        CH1_GMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0 The interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high when this bit is high This bit is applicable only when the Power Management feature is enabled Otherwise, this bit is reserved Note: The GPI and pmt_intr_o interrupts are generated in different clock domains"]
    #[inline(always)]
    pub fn ch1_gpi(&self) -> CH1_GPI_R {
        CH1_GPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0 When this bit is high, the interrupt signal from the DWC_gmac subsystem _sbd_intr_o_ is high This bit is applicable only when the IEEE 1588 Timestamp feature is enabled Otherwise, this bit is reserved"]
    #[inline(always)]
    pub fn ch1_tti(&self) -> CH1_TTI_R {
        CH1_TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GTMSI: GMAC TMS Interrupt _for Channel 1 and Channel 2_ This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac To reset this bit, the software must read the corresponding registers _Channel Status Register_ to get the exact cause of the interrupt and clear its source Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present Otherwise, this bit is reserved When this bit is high, the interrupt signal from the MAC _sbd_intr_o_ is high"]
    #[inline(always)]
    pub fn ch1_glpii_gtmsi(&self) -> CH1_GLPII_GTMSI_R {
        CH1_GLPII_GTMSI_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER69_CHANNEL1STATUSREGISTER")
            .field("ch1_ti", &self.ch1_ti())
            .field("ch1_tps", &self.ch1_tps())
            .field("ch1_tu", &self.ch1_tu())
            .field("ch1_tjt", &self.ch1_tjt())
            .field("ch1_ovf", &self.ch1_ovf())
            .field("ch1_unf", &self.ch1_unf())
            .field("ch1_ri", &self.ch1_ri())
            .field("ch1_ru", &self.ch1_ru())
            .field("ch1_rps", &self.ch1_rps())
            .field("ch1_rwt", &self.ch1_rwt())
            .field("ch1_eti", &self.ch1_eti())
            .field("ch1_fbi", &self.ch1_fbi())
            .field("ch1_eri", &self.ch1_eri())
            .field("ch1_ais", &self.ch1_ais())
            .field("ch1_nis", &self.ch1_nis())
            .field("ch1_rs", &self.ch1_rs())
            .field("ch1_ts", &self.ch1_ts())
            .field("ch1_eb", &self.ch1_eb())
            .field("ch1_gli", &self.ch1_gli())
            .field("ch1_gmi", &self.ch1_gmi())
            .field("ch1_gpi", &self.ch1_gpi())
            .field("ch1_tti", &self.ch1_tti())
            .field("ch1_glpii_gtmsi", &self.ch1_glpii_gtmsi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the frame transmission is complete When transmission is complete, Bit 31 _OWN_ of TDES0 is reset, and the specific frame status information is updated in the descriptor"]
    #[inline(always)]
    pub fn ch1_ti(&mut self) -> CH1_TI_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped"]
    #[inline(always)]
    pub fn ch1_tps(&mut self) -> CH1_TPS_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_TPS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it Transmission is suspended Bits\\[22:20\\] explain the Transmit Process state transitions To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand command"]
    #[inline(always)]
    pub fn ch1_tu(&mut self) -> CH1_TU_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_TU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 _10,240 bytes when the Jumbo frame is enabled_ When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert"]
    #[inline(always)]
    pub fn ch1_tjt(&mut self) -> CH1_TJT_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_TJT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]"]
    #[inline(always)]
    pub fn ch1_ovf(&mut self) -> CH1_OVF_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission Transmission is suspended and an Underflow Error TDES0\\[1\\] is set"]
    #[inline(always)]
    pub fn ch1_unf(&mut self) -> CH1_UNF_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_UNF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the frame reception is complete When reception is complete, the Bit 31 of RDES1 _Disable Interrupt on Completion_ is reset in the last Descriptor, and the specific frame status information is updated in the descriptor The reception remains in the Running state"]
    #[inline(always)]
    pub fn ch1_ri(&mut self) -> CH1_RI_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_RI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it The Receive Process is suspended To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received This bit is set only when the previous Receive Descriptor is owned by the DMA"]
    #[inline(always)]
    pub fn ch1_ru(&mut self) -> CH1_RU_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_RU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state"]
    #[inline(always)]
    pub fn ch1_rps(&mut self) -> CH1_RPS_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_RPS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout"]
    #[inline(always)]
    pub fn ch1_rwt(&mut self) -> CH1_RWT_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_RWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO"]
    #[inline(always)]
    pub fn ch1_eti(&mut self) -> CH1_ETI_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_ETI_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\] When this bit is set, the corresponding DMA engine disables all of its bus accesses 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn ch1_fbi(&mut self) -> CH1_FBI_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_FBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet This bit is cleared when the software writes 1 to this bit or Bit 6 _RI_ of this register is set _whichever occurs earlier_"]
    #[inline(always)]
    pub fn ch1_eri(&mut self) -> CH1_ERI_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_ERI_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive FIFO Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes AIS to be set, is cleared"]
    #[inline(always)]
    pub fn ch1_ais(&mut self) -> CH1_AIS_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_AIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 _Interrupt Enable Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits _interrupts for which interrupt enable is set in Register 7_ affect the Normal Interrupt Summary bit This is a sticky bit and must be cleared _by writing 1 to this bit_ each time a corresponding bit, which causes NIS to be set, is cleared"]
    #[inline(always)]
    pub fn ch1_nis(&mut self) -> CH1_NIS_W<'_, REGISTER69_CHANNEL1STATUSREGISTER_SPEC> {
        CH1_NIS_W::new(self, 16)
    }
}
#[doc = "The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits 29:26 are reserved for the Channel 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register69_channel1statusregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register69_channel1statusregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER69_CHANNEL1STATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER69_CHANNEL1STATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register69_channel1statusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER69_CHANNEL1STATUSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register69_channel1statusregister::W`](W) writer structure"]
impl crate::Writable for REGISTER69_CHANNEL1STATUSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER69_CHANNEL1STATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER69_CHANNEL1STATUSREGISTER_SPEC {}
