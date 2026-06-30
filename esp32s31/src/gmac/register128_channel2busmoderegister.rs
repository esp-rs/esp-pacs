#[doc = "Register `REGISTER128_CHANNEL2BUSMODEREGISTER` reader"]
pub type R = crate::R<REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC>;
#[doc = "Register `REGISTER128_CHANNEL2BUSMODEREGISTER` writer"]
pub type W = crate::W<REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC>;
#[doc = "Field `CH2_SWR` reader - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
pub type CH2_SWR_R = crate::BitReader;
#[doc = "Field `CH2_SWR` writer - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
pub type CH2_SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_DA` reader - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 2 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_DA_R = crate::BitReader;
#[doc = "Field `CH2_DA` writer - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 2 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_DSL` reader - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
pub type CH2_DSL_R = crate::FieldReader;
#[doc = "Field `CH2_DSL` writer - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
pub type CH2_DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH2_ATDS` reader - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
pub type CH2_ATDS_R = crate::BitReader;
#[doc = "Field `CH2_ATDS` writer - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
pub type CH2_ATDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_PBL` reader - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
pub type CH2_PBL_R = crate::FieldReader;
#[doc = "Field `CH2_PBL` writer - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
pub type CH2_PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CH2_PR` reader - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_PR_R = crate::FieldReader;
#[doc = "Field `CH2_PR` writer - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_FB` reader - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
pub type CH2_FB_R = crate::BitReader;
#[doc = "Field `CH2_FB` writer - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
pub type CH2_FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RPBL` reader - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
pub type CH2_RPBL_R = crate::FieldReader;
#[doc = "Field `CH2_RPBL` writer - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
pub type CH2_RPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CH2_USP` reader - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
pub type CH2_USP_R = crate::BitReader;
#[doc = "Field `CH2_USP` writer - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
pub type CH2_USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_PBLX8` reader - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
pub type CH2_PBLX8_R = crate::BitReader;
#[doc = "Field `CH2_PBLX8` writer - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
pub type CH2_PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_AAL` reader - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
pub type CH2_AAL_R = crate::BitReader;
#[doc = "Field `CH2_AAL` writer - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
pub type CH2_AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
pub type CH2_MB_R = crate::BitReader;
#[doc = "Field `CH2_MB` writer - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
pub type CH2_MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TXPR` reader - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_TXPR_R = crate::BitReader;
#[doc = "Field `CH2_TXPR` writer - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type CH2_TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_PRWG` reader - Channel Priority Weights This field sets the priority weights for Channel 2 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
pub type CH2_PRWG_R = crate::FieldReader;
#[doc = "Field `CH2_PRWG` writer - Channel Priority Weights This field sets the priority weights for Channel 2 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
pub type CH2_PRWG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_RIB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
pub type CH2_RIB_R = crate::BitReader;
#[doc = "Field `CH2_RIB` writer - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
pub type CH2_RIB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
    #[inline(always)]
    pub fn ch2_swr(&self) -> CH2_SWR_R {
        CH2_SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 2 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_da(&self) -> CH2_DA_R {
        CH2_DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
    #[inline(always)]
    pub fn ch2_dsl(&self) -> CH2_DSL_R {
        CH2_DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
    #[inline(always)]
    pub fn ch2_atds(&self) -> CH2_ATDS_R {
        CH2_ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
    #[inline(always)]
    pub fn ch2_pbl(&self) -> CH2_PBL_R {
        CH2_PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_pr(&self) -> CH2_PR_R {
        CH2_PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
    #[inline(always)]
    pub fn ch2_fb(&self) -> CH2_FB_R {
        CH2_FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
    #[inline(always)]
    pub fn ch2_rpbl(&self) -> CH2_RPBL_R {
        CH2_RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
    #[inline(always)]
    pub fn ch2_usp(&self) -> CH2_USP_R {
        CH2_USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
    #[inline(always)]
    pub fn ch2_pblx8(&self) -> CH2_PBLX8_R {
        CH2_PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
    #[inline(always)]
    pub fn ch2_aal(&self) -> CH2_AAL_R {
        CH2_AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
    #[inline(always)]
    pub fn ch2_mb(&self) -> CH2_MB_R {
        CH2_MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_txpr(&self) -> CH2_TXPR_R {
        CH2_TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 2 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_prwg(&self) -> CH2_PRWG_R {
        CH2_PRWG_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
    #[inline(always)]
    pub fn ch2_rib(&self) -> CH2_RIB_R {
        CH2_RIB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER128_CHANNEL2BUSMODEREGISTER")
            .field("ch2_swr", &self.ch2_swr())
            .field("ch2_da", &self.ch2_da())
            .field("ch2_dsl", &self.ch2_dsl())
            .field("ch2_atds", &self.ch2_atds())
            .field("ch2_pbl", &self.ch2_pbl())
            .field("ch2_pr", &self.ch2_pr())
            .field("ch2_fb", &self.ch2_fb())
            .field("ch2_rpbl", &self.ch2_rpbl())
            .field("ch2_usp", &self.ch2_usp())
            .field("ch2_pblx8", &self.ch2_pblx8())
            .field("ch2_aal", &self.ch2_aal())
            .field("ch2_mb", &self.ch2_mb())
            .field("ch2_txpr", &self.ch2_txpr())
            .field("ch2_prwg", &self.ch2_prwg())
            .field("ch2_rib", &self.ch2_rib())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
    #[inline(always)]
    pub fn ch2_swr(&mut self) -> CH2_SWR_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 2 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_da(&mut self) -> CH2_DA_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
    #[inline(always)]
    pub fn ch2_dsl(&mut self) -> CH2_DSL_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_DSL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
    #[inline(always)]
    pub fn ch2_atds(&mut self) -> CH2_ATDS_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_ATDS_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
    #[inline(always)]
    pub fn ch2_pbl(&mut self) -> CH2_PBL_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_pr(&mut self) -> CH2_PR_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
    #[inline(always)]
    pub fn ch2_fb(&mut self) -> CH2_FB_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
    #[inline(always)]
    pub fn ch2_rpbl(&mut self) -> CH2_RPBL_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_RPBL_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
    #[inline(always)]
    pub fn ch2_usp(&mut self) -> CH2_USP_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
    #[inline(always)]
    pub fn ch2_pblx8(&mut self) -> CH2_PBLX8_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_PBLX8_W::new(self, 24)
    }
    #[doc = "Bit 25 - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
    #[inline(always)]
    pub fn ch2_aal(&mut self) -> CH2_AAL_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_AAL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
    #[inline(always)]
    pub fn ch2_mb(&mut self) -> CH2_MB_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_MB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_txpr(&mut self) -> CH2_TXPR_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_TXPR_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 2 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn ch2_prwg(&mut self) -> CH2_PRWG_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_PRWG_W::new(self, 28)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
    #[inline(always)]
    pub fn ch2_rib(&mut self) -> CH2_RIB_W<'_, REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC> {
        CH2_RIB_W::new(self, 31)
    }
}
#[doc = "Controls the Host Interface mode for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register128_channel2busmoderegister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register128_channel2busmoderegister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register128_channel2busmoderegister::R`](R) reader structure"]
impl crate::Readable for REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register128_channel2busmoderegister::W`](W) writer structure"]
impl crate::Writable for REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER128_CHANNEL2BUSMODEREGISTER to value 0x0002_0101"]
impl crate::Resettable for REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
