#[doc = "Register `REGISTER0_BUSMODEREGISTER` reader"]
pub type R = crate::R<REGISTER0_BUSMODEREGISTER_SPEC>;
#[doc = "Register `REGISTER0_BUSMODEREGISTER` writer"]
pub type W = crate::W<REGISTER0_BUSMODEREGISTER_SPEC>;
#[doc = "Field `SWR` reader - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
pub type ATDS_R = crate::BitReader;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
pub type ATDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
pub type RPBL_R = crate::FieldReader;
#[doc = "Field `RPBL` writer - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
pub type RPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLX8` reader - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLX8` writer - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPR` reader - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type TXPR_R = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRWG` reader - Channel Priority Weights This field sets the priority weights for Channel 0 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
pub type PRWG_R = crate::FieldReader;
#[doc = "Field `PRWG` writer - Channel Priority Weights This field sets the priority weights for Channel 0 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
pub type PRWG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RIB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
pub type RIB_R = crate::BitReader;
#[doc = "Field `RIB` writer - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
pub type RIB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
    #[inline(always)]
    pub fn atds(&self) -> ATDS_R {
        ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
    #[inline(always)]
    pub fn rpbl(&self) -> RPBL_R {
        RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 0 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn prwg(&self) -> PRWG_R {
        PRWG_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
    #[inline(always)]
    pub fn rib(&self) -> RIB_R {
        RIB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER0_BUSMODEREGISTER")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("atds", &self.atds())
            .field("pbl", &self.pbl())
            .field("pr", &self.pr())
            .field("fb", &self.fb())
            .field("rpbl", &self.rpbl())
            .field("usp", &self.usp())
            .field("pblx8", &self.pblx8())
            .field("aal", &self.aal())
            .field("mb", &self.mb())
            .field("txpr", &self.txpr())
            .field("prwg", &self.prwg())
            .field("rib", &self.rib())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains Before reprogramming any register of the DWC_gmac, you should read a zero _0_ value in this bit Note: The Software reset function is driven only by this bit Bit 0 of Register 64 _Channel 1 Bus Mode Register_ or Register 128 _Channel 2 Bus Mode Register_ has no impact on the Software reset function The reset operation is completed only when all resets in all active clock domains are deasserted Therefore, it is essential that all PHY inputs clocks _applicable for the selected PHY interface_ are present for the software reset completion The time to complete the software reset operation depends on the frequency of the slowest active clock"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0 0: Weighted roundrobin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\] _PR_ and priority weights specified in Bit 27 _TXPR_ 1: Fixed priority The transmit path has priority over receive path when Bit 27 _TXPR_ is set Otherwise, receive path has priority over the transmit path In the GMACAXI configuration, these bits are reserved and are readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword _depending on the 32bit, 64bit, or 128bit bus_ to skip between two unchained descriptors The address skipping starts from the end of current descriptor to the start of next descriptor When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor _described in “Alternate or Enhanced Descriptors” on page 545_ increases to 32 bytes _8 DWORDS_ This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine _Type 2_ is enabled in the receiver The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features are not enabled In such case, you can use the 16 bytes descriptor to save 4 bytes of memory This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: Advanced Timestamp feature IPC Full Checksum Offload Engine _Type 2_ feature Otherwise, this bit is reserved and is readonly When reset, the descriptor size reverts back to 4 DWORDs _16 bytes_ This bit preserves the backward compatibility for the descriptor size In versions prior to 350a, the descriptor size is 16 bytes for both normal and enhanced descriptors In version 350a, descriptor size is increased to 32 bytes because of the Advanced Timestamp and IPC Full Checksum Offload Engine _Type 2_ features"]
    #[inline(always)]
    pub fn atds(&mut self) -> ATDS_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        ATDS_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction This is the maximum value that is used in a single block Read or Write The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior When USP is set high, this PBL value is applicable only for Tx DMA transactions If the number of beats to be transferred is more than 32, then perform the following steps: 1 Set the PBLx8 mode 2 Set the PBL For example, if the maximum number of beats to be transferred is 64, then first set PBLx8 to 1 and then set PBL to 8 The PBL values have the following limitation: The maximum number of possible beats _PBL_ is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified For different data bus widths and FIFO sizes, the valid PBL range _including x8 mode_ is provided in Table 66 on page 382"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted roundrobin arbitration between the Rx DMA and Tx DMA These bits are valid only when Bit 1 _DA_ is reset The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 _TXPR_ is reset or set 00: The Priority Ratio is 1:1 01: The Priority Ratio is 2:1 10: The Priority Ratio is 3:1 11: The Priority Ratio is 4:1 In the GMACAXI configuration, these bits are reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations For more information, see Bit 0 _UNDEF_ of the AXI Bus Mode register in the GMACAXI configuration"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction This is the maximum value that is used in a single block Read or Write The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus You can program RPBL with values of 1, 2, 4, 8, 16, and 32 Any other value results in undefined behavior This field is valid and applicable only when USP is set high"]
    #[inline(always)]
    pub fn rpbl(&mut self) -> RPBL_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        RPBL_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\] as PBL The PBL value in Bits \\[13:8\\] is applicable only to the Tx DMA operations When reset to low, the PBL value in Bits \\[13:8\\] is applicable for both DMA engines"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value _Bits \\[22:17\\] and Bits\\[13:8\\]_ eight times Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value Note: This bit function is not backward compatible Before release 350a, this bit was 4xPBL"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        PBLX8_W::new(self, 24)
    }
    #[doc = "Bit 25 - AddressAligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits If the FB bit is equal to 0, the first burst _accessing the start address of data buffer_ is not aligned, but subsequent bursts are aligned to the address This bit is valid only in the GMACAHB and GMACAXI configurations and is reserved _RO with default value 0_ in all other configurations"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        AAL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR _undefined burst_, whereas it reverts to fixed burst transfers _INCRx and SINGLE_ for burst length of 16 and less This bit is valid only in the GMACAHB configuration and reserved in all other configuration"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        MB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the systemside bus In the GMACAXI configuration, this bit is reserved and readonly _RO_ For more information about the priority scheme between the transmit and receive paths, see Table 412 in “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        TXPR_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 0 during the roundrobin arbitration between the DMA channels for the system bus 00: The priority weight is 1 01: The priority weight is 2 10: The priority weight is 3 11: The priority weight is 4 This field is present in all DWC_gmac configurations except GMACAXI when you select the AV feature Otherwise, this field is reserved and readonly _RO_ For more information about the priority weights of DMA channels, see “DMA Arbiter Functions” on page 167"]
    #[inline(always)]
    pub fn prwg(&mut self) -> PRWG_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        PRWG_W::new(self, 28)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT _Retry, Split, or Losing bus grant_, the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified _INCR_ burst This bit is valid only in the GMACAHB configuration It is reserved in all other configuration"]
    #[inline(always)]
    pub fn rib(&mut self) -> RIB_W<'_, REGISTER0_BUSMODEREGISTER_SPEC> {
        RIB_W::new(self, 31)
    }
}
#[doc = "Controls the Host Interface Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`register0_busmoderegister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0_busmoderegister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER0_BUSMODEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER0_BUSMODEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register0_busmoderegister::R`](R) reader structure"]
impl crate::Readable for REGISTER0_BUSMODEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register0_busmoderegister::W`](W) writer structure"]
impl crate::Writable for REGISTER0_BUSMODEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER0_BUSMODEREGISTER to value 0x0002_0101"]
impl crate::Resettable for REGISTER0_BUSMODEREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
