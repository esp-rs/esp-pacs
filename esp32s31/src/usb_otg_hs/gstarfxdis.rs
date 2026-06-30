#[doc = "Register `GSTARFXDIS` reader"]
pub type R = crate::R<GSTARFXDIS_SPEC>;
#[doc = "Register `GSTARFXDIS` writer"]
pub type W = crate::W<GSTARFXDIS_SPEC>;
#[doc = "Field `HOST_IGNORES_RMTWKUP_DIS` reader - Disable the STAR fix added for Device controller to go back to low power mode when Host ignores Remote wakeup The application programs Host_Ignores_RmtWkup_dis=1 to disable the device controller from going back to low power mode (SUSPENDED state) when the host ignores the remote wakeup signalling from the device and does not respond back with resume. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_IGNORES_RMTWKUP_DIS_R = crate::BitReader;
#[doc = "Field `HOST_IGNORES_RMTWKUP_DIS` writer - Disable the STAR fix added for Device controller to go back to low power mode when Host ignores Remote wakeup The application programs Host_Ignores_RmtWkup_dis=1 to disable the device controller from going back to low power mode (SUSPENDED state) when the host ignores the remote wakeup signalling from the device and does not respond back with resume. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_IGNORES_RMTWKUP_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_FRM_CHK_BUS_DIS` reader - Disable the STAR fix added for Device controller to detect lineK and move to RESUMING state after the 50us pull-up delay ends The application programs Resume_frm_CHK_BUS_dis=1 to disable the device controller from detecting line K and transitioning to RESUMING state in the scenario where Host resume starts immediately after the 50us pull-up delay timer expires. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type RESUME_FRM_CHK_BUS_DIS_R = crate::BitReader;
#[doc = "Field `RESUME_FRM_CHK_BUS_DIS` writer - Disable the STAR fix added for Device controller to detect lineK and move to RESUMING state after the 50us pull-up delay ends The application programs Resume_frm_CHK_BUS_dis=1 to disable the device controller from detecting line K and transitioning to RESUMING state in the scenario where Host resume starts immediately after the 50us pull-up delay timer expires. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type RESUME_FRM_CHK_BUS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORE_CTLOUT_DATA0_DIS` reader - Disable the STAR fix added for Device controller to reject DATA0 for the first Control OUT Data Phase and Control Status OUT Phase The application programs Ignore_CtlOUT_DATA0_dis=1 to disable the device controller from reporting transaction error when DATA0 PID is received from host for the first Control OUT Data Phase and Control Status OUT phase. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type IGNORE_CTLOUT_DATA0_DIS_R = crate::BitReader;
#[doc = "Field `IGNORE_CTLOUT_DATA0_DIS` writer - Disable the STAR fix added for Device controller to reject DATA0 for the first Control OUT Data Phase and Control Status OUT Phase The application programs Ignore_CtlOUT_DATA0_dis=1 to disable the device controller from reporting transaction error when DATA0 PID is received from host for the first Control OUT Data Phase and Control Status OUT phase. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type IGNORE_CTLOUT_DATA0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSPLIT_STALLNYET_ERR_DIS` reader - Disable the STAR fix added for Host controller to flag error for SSPLIT STALL/NYET The application programs SSPLIT_STALLNYET_Err_dis=1 to disable the host controller from reporting transaction error for STALL/NYET received from device for SSPLIT transfer. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type SSPLIT_STALLNYET_ERR_DIS_R = crate::BitReader;
#[doc = "Field `SSPLIT_STALLNYET_ERR_DIS` writer - Disable the STAR fix added for Host controller to flag error for SSPLIT STALL/NYET The application programs SSPLIT_STALLNYET_Err_dis=1 to disable the host controller from reporting transaction error for STALL/NYET received from device for SSPLIT transfer. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type SSPLIT_STALLNYET_ERR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPT_ISOC_SPLIT_DATA1_DIS` reader - Disable the STAR fix added for Host controller to accept DATA1 PID from device for ISOC Split transfers The application programs Accept_Isoc_split_DATA1_dis=1 to disable the host controller from accepting DATA1 PID from device for ISOC Split transfers. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type ACCEPT_ISOC_SPLIT_DATA1_DIS_R = crate::BitReader;
#[doc = "Field `ACCEPT_ISOC_SPLIT_DATA1_DIS` writer - Disable the STAR fix added for Host controller to accept DATA1 PID from device for ISOC Split transfers The application programs Accept_Isoc_split_DATA1_dis=1 to disable the host controller from accepting DATA1 PID from device for ISOC Split transfers. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type ACCEPT_ISOC_SPLIT_DATA1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HANDLEFAULTYCABLE_DIS` reader - Disable the STAR fix added for Host controller to handle Faulty cable scenarios The application programs HandleFaultyCable_dis=1 to disable the host controller from reporting Port Babble error when the linestate is stuck at 1 (in FS mode) or 2 (in LS mode) due to a Faulty cable. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HANDLEFAULTYCABLE_DIS_R = crate::BitReader;
#[doc = "Field `HANDLEFAULTYCABLE_DIS` writer - Disable the STAR fix added for Host controller to handle Faulty cable scenarios The application programs HandleFaultyCable_dis=1 to disable the host controller from reporting Port Babble error when the linestate is stuck at 1 (in FS mode) or 2 (in LS mode) due to a Faulty cable. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HANDLEFAULTYCABLE_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_IPG_INCR_DIS` reader - Disable the STAR fix added for Host controller LS mode IPG increment from 2 LS bit times to 3 LS bit times The application programs LS_IPG_incr_dis=1 to disable the IPG increment from 2 to 3 LS bit times when host controller is operating in LS mode. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type LS_IPG_INCR_DIS_R = crate::BitReader;
#[doc = "Field `LS_IPG_INCR_DIS` writer - Disable the STAR fix added for Host controller LS mode IPG increment from 2 LS bit times to 3 LS bit times The application programs LS_IPG_incr_dis=1 to disable the IPG increment from 2 to 3 LS bit times when host controller is operating in LS mode. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type LS_IPG_INCR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSDISC_IDLE_DIS` reader - Disable the STAR fix added for Device controller to transition to IDLE state during FS device disconnect The application programs FSDisc_Idle_dis=1 to disable the Transmit/Receive state machine from moving to IDLE state when FS device disconnect happens. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type FSDISC_IDLE_DIS_R = crate::BitReader;
#[doc = "Field `FSDISC_IDLE_DIS` writer - Disable the STAR fix added for Device controller to transition to IDLE state during FS device disconnect The application programs FSDisc_Idle_dis=1 to disable the Transmit/Receive state machine from moving to IDLE state when FS device disconnect happens. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type FSDISC_IDLE_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONCURRENT_RMTWKUP_USBRESUME_DIS` reader - Disable the STAR fix added for Device controller to not start Remote Wakeup signalling when USB resume has already started The application programs Concurrent_Rmtwkup_USBResume_dis=1 to allow the controller to do remote wakeup signalling when USB resume has already started from the Host. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type CONCURRENT_RMTWKUP_USBRESUME_DIS_R = crate::BitReader;
#[doc = "Field `CONCURRENT_RMTWKUP_USBRESUME_DIS` writer - Disable the STAR fix added for Device controller to not start Remote Wakeup signalling when USB resume has already started The application programs Concurrent_Rmtwkup_USBResume_dis=1 to allow the controller to do remote wakeup signalling when USB resume has already started from the Host. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type CONCURRENT_RMTWKUP_USBRESUME_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS` reader - Disable the STAR fix added for Device controller to not hang when Remote Wakeup signalling clashes with Host resume The application can program Concurrent_Rmtwkup_USBResume_Hib_dis=1 to disable the fix. Note: With Concurrent_Rmtwkup_USBResume_Hib_dis=0, the STAR fix is enabled and the device controller does not hang when the remote wakeup clashes with host resume. After the remote wakeup programming is done, if there was no utmi_txready received from the PHY while the utmi_txvalid was asserted, then the controller detects the line K as the host resume and directly moves to the RESUMING state, along with de-asserting utmi_txvalid. This prevents the controller from hanging but also violates the UTMI spec requirement which states that utmi_txvalid must be de-asserted only after getting utmi_txready. Hence, if any misbehaviour is observed with this fix enabled, it is recommended to keep the fix disabled. This bit must be programmed during the initial configuration of the controller and must not be modified later. The fix is applicable only for Hibernation case."]
pub type CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_R = crate::BitReader;
#[doc = "Field `CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS` writer - Disable the STAR fix added for Device controller to not hang when Remote Wakeup signalling clashes with Host resume The application can program Concurrent_Rmtwkup_USBResume_Hib_dis=1 to disable the fix. Note: With Concurrent_Rmtwkup_USBResume_Hib_dis=0, the STAR fix is enabled and the device controller does not hang when the remote wakeup clashes with host resume. After the remote wakeup programming is done, if there was no utmi_txready received from the PHY while the utmi_txvalid was asserted, then the controller detects the line K as the host resume and directly moves to the RESUMING state, along with de-asserting utmi_txvalid. This prevents the controller from hanging but also violates the UTMI spec requirement which states that utmi_txvalid must be de-asserted only after getting utmi_txready. Hence, if any misbehaviour is observed with this fix enabled, it is recommended to keep the fix disabled. This bit must be programmed during the initial configuration of the controller and must not be modified later. The fix is applicable only for Hibernation case."]
pub type CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS` reader - Disable the STAR fix added for Host controller to wait for IPG duration to send next token after receiving NAK/STALL for previous IN token with FS/LS device The application programs LS_IPG_chk_after_NAK_STALL_for_IN_dis=1 to allow the controller to send the next token without waiting for the interpacket gap duration after receiving a NAK/STALL from device for the previous IN token. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_R = crate::BitReader;
#[doc = "Field `LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS` writer - Disable the STAR fix added for Host controller to wait for IPG duration to send next token after receiving NAK/STALL for previous IN token with FS/LS device The application programs LS_IPG_chk_after_NAK_STALL_for_IN_dis=1 to allow the controller to send the next token without waiting for the interpacket gap duration after receiving a NAK/STALL from device for the previous IN token. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_IOP_XCVRSEL_TXVLD_CORR_DIS` reader - Disable the STAR fix added for Host controller to increase the gap between utmi_xcvrselect switching and utmi_txvalid assertion in LS/FS mode The application programs PHY_IOp_xcvrsel_txvld_corr_dis=1 to allow the controller to assert utmi_txvalid 1 cycle after the utmi_xcvrselect switching. This can cause interop issues with SNPS PHY which requires at least 2 cycles of gap between the utmi_xcvrselect switch and the utmi_txvalid assertion. Hence it is recommended to program PHY_IOp_xcvrsel_txvld_corr_dis=0. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_R = crate::BitReader;
#[doc = "Field `PHY_IOP_XCVRSEL_TXVLD_CORR_DIS` writer - Disable the STAR fix added for Host controller to increase the gap between utmi_xcvrselect switching and utmi_txvalid assertion in LS/FS mode The application programs PHY_IOp_xcvrsel_txvld_corr_dis=1 to allow the controller to assert utmi_txvalid 1 cycle after the utmi_xcvrselect switching. This can cause interop issues with SNPS PHY which requires at least 2 cycles of gap between the utmi_xcvrselect switch and the utmi_txvalid assertion. Hence it is recommended to program PHY_IOp_xcvrsel_txvld_corr_dis=0. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPI_XCVRSEL_SWITCH_CORR_DIS` reader - Disable the STAR fix added for Host controller to increase the preamble transceiver select switch delay to accommodate time taken for ULPI function control write The application programs ULPI_xcvrsel_switch_corr_dis=1 to allow the controller to do back to back transceiver select switching within the duration required for a single ULPI function control write (around 5-6 ulpi_clk cycles). This can cause a corrupted packet to go on the bus since the ULPI wrapper may not be able to do the second ULPI functional update (register write) correctly. Hence it is recommended to program ULPI_xcvrsel_switch_corr_dis=0. With ULPI_xcvrsel_switch_corr_dis=0, the delay between transceiver select switch and txvalid assertion has also been increased for FS/LS mode to resolve PHY interop issues. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type ULPI_XCVRSEL_SWITCH_CORR_DIS_R = crate::BitReader;
#[doc = "Field `ULPI_XCVRSEL_SWITCH_CORR_DIS` writer - Disable the STAR fix added for Host controller to increase the preamble transceiver select switch delay to accommodate time taken for ULPI function control write The application programs ULPI_xcvrsel_switch_corr_dis=1 to allow the controller to do back to back transceiver select switching within the duration required for a single ULPI function control write (around 5-6 ulpi_clk cycles). This can cause a corrupted packet to go on the bus since the ULPI wrapper may not be able to do the second ULPI functional update (register write) correctly. Hence it is recommended to program ULPI_xcvrsel_switch_corr_dis=0. With ULPI_xcvrsel_switch_corr_dis=0, the delay between transceiver select switch and txvalid assertion has also been increased for FS/LS mode to resolve PHY interop issues. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type ULPI_XCVRSEL_SWITCH_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR_DATA0_CTRL_STS_IN_DIS` reader - Disable the STAR fix added for Host controller to report transaction error when DATA0 PID is received for CTRL STATUS IN transfer in DMA mode The application programs XactErr_DATA0_CTRL_STS_IN_dis=1 to allow the controller to respond with ACK for the incorrect DATA0 PID and then retry the control STATUS IN transfer until DATA1 PID is received. With XactErr_DATA0_CTRL_STS_IN_dis=0, the controller generates Transaction Error interrupt when the incorrect PID is received. This STAR fix is applicable only for Buffer DMA and Descriptor DMA modes. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type XACTERR_DATA0_CTRL_STS_IN_DIS_R = crate::BitReader;
#[doc = "Field `XACTERR_DATA0_CTRL_STS_IN_DIS` writer - Disable the STAR fix added for Host controller to report transaction error when DATA0 PID is received for CTRL STATUS IN transfer in DMA mode The application programs XactErr_DATA0_CTRL_STS_IN_dis=1 to allow the controller to respond with ACK for the incorrect DATA0 PID and then retry the control STATUS IN transfer until DATA1 PID is received. With XactErr_DATA0_CTRL_STS_IN_dis=0, the controller generates Transaction Error interrupt when the incorrect PID is received. This STAR fix is applicable only for Buffer DMA and Descriptor DMA modes. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type XACTERR_DATA0_CTRL_STS_IN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS` reader - Disable the STAR fix added for correcting XcvrSel on UTMI Interface in host mode. When host controller drives reset in power down suspend state, the behavior of XcvrSel has been corrected to drive the valid combination of XcvrSel (HS_XCVR) and TermSel (HS_TERM) on the UTMI interface. The application can set this register bit to 1'b1 to fall back to the original behavior of the Host controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when reset is driven to exit from power down state. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_R = crate::BitReader;
#[doc = "Field `HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS` writer - Disable the STAR fix added for correcting XcvrSel on UTMI Interface in host mode. When host controller drives reset in power down suspend state, the behavior of XcvrSel has been corrected to drive the valid combination of XcvrSel (HS_XCVR) and TermSel (HS_TERM) on the UTMI interface. The application can set this register bit to 1'b1 to fall back to the original behavior of the Host controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when reset is driven to exit from power down state. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_UTMI_TXVLD_CORR_DIS` reader - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface in Host mode. When HPRT.PrtRst is set by the application, the Host controller can change the Opmode, XcvrSel and TermSel while TxValid is still high (1'b1) during SOF transmission in 8-bit UTMI mode. This behavior of the controller has been corrected to handle port reset during SOF transmission. With the fix, the controller waits for Txvalid to go low (1'b0) and then changes the OpMode, XcvrSel and TermSel. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_UTMI_TXVLD_CORR_DIS_R = crate::BitReader;
#[doc = "Field `HOST_UTMI_TXVLD_CORR_DIS` writer - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface in Host mode. When HPRT.PrtRst is set by the application, the Host controller can change the Opmode, XcvrSel and TermSel while TxValid is still high (1'b1) during SOF transmission in 8-bit UTMI mode. This behavior of the controller has been corrected to handle port reset during SOF transmission. With the fix, the controller waits for Txvalid to go low (1'b0) and then changes the OpMode, XcvrSel and TermSel. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_UTMI_TXVLD_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE_XCVRSEL_CHIRPEN_CORR_DIS` reader - Disable the STAR fix added for correcting Opmode and XcvrSel on UTMI Interface when reset is detected in suspend state. In configurations with Battery charger support enabled (and GOTGCTL.ChirpEn programmed to 1), the RTL has been updated to wait for 1ms and then change the opmode and xcvrselect when reset is detected in the suspend state. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Opmode and XcvrSel without waiting for 1ms after reset detection. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_R = crate::BitReader;
#[doc = "Field `OPMODE_XCVRSEL_CHIRPEN_CORR_DIS` writer - Disable the STAR fix added for correcting Opmode and XcvrSel on UTMI Interface when reset is detected in suspend state. In configurations with Battery charger support enabled (and GOTGCTL.ChirpEn programmed to 1), the RTL has been updated to wait for 1ms and then change the opmode and xcvrselect when reset is detected in the suspend state. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Opmode and XcvrSel without waiting for 1ms after reset detection. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXVALID_DEASSERTION_CORR_DIS` reader - Disable the STAR fix added for correcting Txvalid deassertion on UTMI Interface when soft disconnect is done. The RTL has been updated to wait for utmi_txready and then de-assert the utmi_txvalid being driven by MAC DSSR when soft disconnect is done. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Txvalid without waiting for Txready. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type TXVALID_DEASSERTION_CORR_DIS_R = crate::BitReader;
#[doc = "Field `TXVALID_DEASSERTION_CORR_DIS` writer - Disable the STAR fix added for correcting Txvalid deassertion on UTMI Interface when soft disconnect is done. The RTL has been updated to wait for utmi_txready and then de-assert the utmi_txvalid being driven by MAC DSSR when soft disconnect is done. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Txvalid without waiting for Txready. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type TXVALID_DEASSERTION_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS` reader - Disable the STAR fix added for correcting Host behavior when port is disabled. The RTL has been updated to not assert utmi_txvalid when the port is disabled. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_R = crate::BitReader;
#[doc = "Field `HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS` writer - Disable the STAR fix added for correcting Host behavior when port is disabled. The RTL has been updated to not assert utmi_txvalid when the port is disabled. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
pub type HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable the STAR fix added for Device controller to go back to low power mode when Host ignores Remote wakeup The application programs Host_Ignores_RmtWkup_dis=1 to disable the device controller from going back to low power mode (SUSPENDED state) when the host ignores the remote wakeup signalling from the device and does not respond back with resume. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_ignores_rmtwkup_dis(&self) -> HOST_IGNORES_RMTWKUP_DIS_R {
        HOST_IGNORES_RMTWKUP_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable the STAR fix added for Device controller to detect lineK and move to RESUMING state after the 50us pull-up delay ends The application programs Resume_frm_CHK_BUS_dis=1 to disable the device controller from detecting line K and transitioning to RESUMING state in the scenario where Host resume starts immediately after the 50us pull-up delay timer expires. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn resume_frm_chk_bus_dis(&self) -> RESUME_FRM_CHK_BUS_DIS_R {
        RESUME_FRM_CHK_BUS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable the STAR fix added for Device controller to reject DATA0 for the first Control OUT Data Phase and Control Status OUT Phase The application programs Ignore_CtlOUT_DATA0_dis=1 to disable the device controller from reporting transaction error when DATA0 PID is received from host for the first Control OUT Data Phase and Control Status OUT phase. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ignore_ctlout_data0_dis(&self) -> IGNORE_CTLOUT_DATA0_DIS_R {
        IGNORE_CTLOUT_DATA0_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable the STAR fix added for Host controller to flag error for SSPLIT STALL/NYET The application programs SSPLIT_STALLNYET_Err_dis=1 to disable the host controller from reporting transaction error for STALL/NYET received from device for SSPLIT transfer. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ssplit_stallnyet_err_dis(&self) -> SSPLIT_STALLNYET_ERR_DIS_R {
        SSPLIT_STALLNYET_ERR_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable the STAR fix added for Host controller to accept DATA1 PID from device for ISOC Split transfers The application programs Accept_Isoc_split_DATA1_dis=1 to disable the host controller from accepting DATA1 PID from device for ISOC Split transfers. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn accept_isoc_split_data1_dis(&self) -> ACCEPT_ISOC_SPLIT_DATA1_DIS_R {
        ACCEPT_ISOC_SPLIT_DATA1_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable the STAR fix added for Host controller to handle Faulty cable scenarios The application programs HandleFaultyCable_dis=1 to disable the host controller from reporting Port Babble error when the linestate is stuck at 1 (in FS mode) or 2 (in LS mode) due to a Faulty cable. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn handlefaultycable_dis(&self) -> HANDLEFAULTYCABLE_DIS_R {
        HANDLEFAULTYCABLE_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable the STAR fix added for Host controller LS mode IPG increment from 2 LS bit times to 3 LS bit times The application programs LS_IPG_incr_dis=1 to disable the IPG increment from 2 to 3 LS bit times when host controller is operating in LS mode. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ls_ipg_incr_dis(&self) -> LS_IPG_INCR_DIS_R {
        LS_IPG_INCR_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable the STAR fix added for Device controller to transition to IDLE state during FS device disconnect The application programs FSDisc_Idle_dis=1 to disable the Transmit/Receive state machine from moving to IDLE state when FS device disconnect happens. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn fsdisc_idle_dis(&self) -> FSDISC_IDLE_DIS_R {
        FSDISC_IDLE_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable the STAR fix added for Device controller to not start Remote Wakeup signalling when USB resume has already started The application programs Concurrent_Rmtwkup_USBResume_dis=1 to allow the controller to do remote wakeup signalling when USB resume has already started from the Host. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn concurrent_rmtwkup_usbresume_dis(&self) -> CONCURRENT_RMTWKUP_USBRESUME_DIS_R {
        CONCURRENT_RMTWKUP_USBRESUME_DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable the STAR fix added for Device controller to not hang when Remote Wakeup signalling clashes with Host resume The application can program Concurrent_Rmtwkup_USBResume_Hib_dis=1 to disable the fix. Note: With Concurrent_Rmtwkup_USBResume_Hib_dis=0, the STAR fix is enabled and the device controller does not hang when the remote wakeup clashes with host resume. After the remote wakeup programming is done, if there was no utmi_txready received from the PHY while the utmi_txvalid was asserted, then the controller detects the line K as the host resume and directly moves to the RESUMING state, along with de-asserting utmi_txvalid. This prevents the controller from hanging but also violates the UTMI spec requirement which states that utmi_txvalid must be de-asserted only after getting utmi_txready. Hence, if any misbehaviour is observed with this fix enabled, it is recommended to keep the fix disabled. This bit must be programmed during the initial configuration of the controller and must not be modified later. The fix is applicable only for Hibernation case."]
    #[inline(always)]
    pub fn concurrent_rmtwkup_usbresume_hib_dis(&self) -> CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_R {
        CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable the STAR fix added for Host controller to wait for IPG duration to send next token after receiving NAK/STALL for previous IN token with FS/LS device The application programs LS_IPG_chk_after_NAK_STALL_for_IN_dis=1 to allow the controller to send the next token without waiting for the interpacket gap duration after receiving a NAK/STALL from device for the previous IN token. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ls_ipg_chk_after_nak_stall_for_in_dis(&self) -> LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_R {
        LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable the STAR fix added for Host controller to increase the gap between utmi_xcvrselect switching and utmi_txvalid assertion in LS/FS mode The application programs PHY_IOp_xcvrsel_txvld_corr_dis=1 to allow the controller to assert utmi_txvalid 1 cycle after the utmi_xcvrselect switching. This can cause interop issues with SNPS PHY which requires at least 2 cycles of gap between the utmi_xcvrselect switch and the utmi_txvalid assertion. Hence it is recommended to program PHY_IOp_xcvrsel_txvld_corr_dis=0. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn phy_iop_xcvrsel_txvld_corr_dis(&self) -> PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_R {
        PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable the STAR fix added for Host controller to increase the preamble transceiver select switch delay to accommodate time taken for ULPI function control write The application programs ULPI_xcvrsel_switch_corr_dis=1 to allow the controller to do back to back transceiver select switching within the duration required for a single ULPI function control write (around 5-6 ulpi_clk cycles). This can cause a corrupted packet to go on the bus since the ULPI wrapper may not be able to do the second ULPI functional update (register write) correctly. Hence it is recommended to program ULPI_xcvrsel_switch_corr_dis=0. With ULPI_xcvrsel_switch_corr_dis=0, the delay between transceiver select switch and txvalid assertion has also been increased for FS/LS mode to resolve PHY interop issues. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ulpi_xcvrsel_switch_corr_dis(&self) -> ULPI_XCVRSEL_SWITCH_CORR_DIS_R {
        ULPI_XCVRSEL_SWITCH_CORR_DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable the STAR fix added for Host controller to report transaction error when DATA0 PID is received for CTRL STATUS IN transfer in DMA mode The application programs XactErr_DATA0_CTRL_STS_IN_dis=1 to allow the controller to respond with ACK for the incorrect DATA0 PID and then retry the control STATUS IN transfer until DATA1 PID is received. With XactErr_DATA0_CTRL_STS_IN_dis=0, the controller generates Transaction Error interrupt when the incorrect PID is received. This STAR fix is applicable only for Buffer DMA and Descriptor DMA modes. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn xacterr_data0_ctrl_sts_in_dis(&self) -> XACTERR_DATA0_CTRL_STS_IN_DIS_R {
        XACTERR_DATA0_CTRL_STS_IN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable the STAR fix added for correcting XcvrSel on UTMI Interface in host mode. When host controller drives reset in power down suspend state, the behavior of XcvrSel has been corrected to drive the valid combination of XcvrSel (HS_XCVR) and TermSel (HS_TERM) on the UTMI interface. The application can set this register bit to 1'b1 to fall back to the original behavior of the Host controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when reset is driven to exit from power down state. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_xcvrsel_resetaftrsusp_corr_dis(&self) -> HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_R {
        HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface in Host mode. When HPRT.PrtRst is set by the application, the Host controller can change the Opmode, XcvrSel and TermSel while TxValid is still high (1'b1) during SOF transmission in 8-bit UTMI mode. This behavior of the controller has been corrected to handle port reset during SOF transmission. With the fix, the controller waits for Txvalid to go low (1'b0) and then changes the OpMode, XcvrSel and TermSel. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_utmi_txvld_corr_dis(&self) -> HOST_UTMI_TXVLD_CORR_DIS_R {
        HOST_UTMI_TXVLD_CORR_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable the STAR fix added for correcting Opmode and XcvrSel on UTMI Interface when reset is detected in suspend state. In configurations with Battery charger support enabled (and GOTGCTL.ChirpEn programmed to 1), the RTL has been updated to wait for 1ms and then change the opmode and xcvrselect when reset is detected in the suspend state. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Opmode and XcvrSel without waiting for 1ms after reset detection. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn opmode_xcvrsel_chirpen_corr_dis(&self) -> OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_R {
        OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable the STAR fix added for correcting Txvalid deassertion on UTMI Interface when soft disconnect is done. The RTL has been updated to wait for utmi_txready and then de-assert the utmi_txvalid being driven by MAC DSSR when soft disconnect is done. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Txvalid without waiting for Txready. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn txvalid_deassertion_corr_dis(&self) -> TXVALID_DEASSERTION_CORR_DIS_R {
        TXVALID_DEASSERTION_CORR_DIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable the STAR fix added for correcting Host behavior when port is disabled. The RTL has been updated to not assert utmi_txvalid when the port is disabled. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_no_xfer_after_prtdis_fix_dis(&self) -> HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_R {
        HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GSTARFXDIS")
            .field("host_ignores_rmtwkup_dis", &self.host_ignores_rmtwkup_dis())
            .field("resume_frm_chk_bus_dis", &self.resume_frm_chk_bus_dis())
            .field("ignore_ctlout_data0_dis", &self.ignore_ctlout_data0_dis())
            .field("ssplit_stallnyet_err_dis", &self.ssplit_stallnyet_err_dis())
            .field(
                "accept_isoc_split_data1_dis",
                &self.accept_isoc_split_data1_dis(),
            )
            .field("handlefaultycable_dis", &self.handlefaultycable_dis())
            .field("ls_ipg_incr_dis", &self.ls_ipg_incr_dis())
            .field("fsdisc_idle_dis", &self.fsdisc_idle_dis())
            .field(
                "concurrent_rmtwkup_usbresume_dis",
                &self.concurrent_rmtwkup_usbresume_dis(),
            )
            .field(
                "concurrent_rmtwkup_usbresume_hib_dis",
                &self.concurrent_rmtwkup_usbresume_hib_dis(),
            )
            .field(
                "ls_ipg_chk_after_nak_stall_for_in_dis",
                &self.ls_ipg_chk_after_nak_stall_for_in_dis(),
            )
            .field(
                "phy_iop_xcvrsel_txvld_corr_dis",
                &self.phy_iop_xcvrsel_txvld_corr_dis(),
            )
            .field(
                "ulpi_xcvrsel_switch_corr_dis",
                &self.ulpi_xcvrsel_switch_corr_dis(),
            )
            .field(
                "xacterr_data0_ctrl_sts_in_dis",
                &self.xacterr_data0_ctrl_sts_in_dis(),
            )
            .field(
                "host_xcvrsel_resetaftrsusp_corr_dis",
                &self.host_xcvrsel_resetaftrsusp_corr_dis(),
            )
            .field("host_utmi_txvld_corr_dis", &self.host_utmi_txvld_corr_dis())
            .field(
                "opmode_xcvrsel_chirpen_corr_dis",
                &self.opmode_xcvrsel_chirpen_corr_dis(),
            )
            .field(
                "txvalid_deassertion_corr_dis",
                &self.txvalid_deassertion_corr_dis(),
            )
            .field(
                "host_no_xfer_after_prtdis_fix_dis",
                &self.host_no_xfer_after_prtdis_fix_dis(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Disable the STAR fix added for Device controller to go back to low power mode when Host ignores Remote wakeup The application programs Host_Ignores_RmtWkup_dis=1 to disable the device controller from going back to low power mode (SUSPENDED state) when the host ignores the remote wakeup signalling from the device and does not respond back with resume. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_ignores_rmtwkup_dis(&mut self) -> HOST_IGNORES_RMTWKUP_DIS_W<'_, GSTARFXDIS_SPEC> {
        HOST_IGNORES_RMTWKUP_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable the STAR fix added for Device controller to detect lineK and move to RESUMING state after the 50us pull-up delay ends The application programs Resume_frm_CHK_BUS_dis=1 to disable the device controller from detecting line K and transitioning to RESUMING state in the scenario where Host resume starts immediately after the 50us pull-up delay timer expires. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn resume_frm_chk_bus_dis(&mut self) -> RESUME_FRM_CHK_BUS_DIS_W<'_, GSTARFXDIS_SPEC> {
        RESUME_FRM_CHK_BUS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable the STAR fix added for Device controller to reject DATA0 for the first Control OUT Data Phase and Control Status OUT Phase The application programs Ignore_CtlOUT_DATA0_dis=1 to disable the device controller from reporting transaction error when DATA0 PID is received from host for the first Control OUT Data Phase and Control Status OUT phase. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ignore_ctlout_data0_dis(&mut self) -> IGNORE_CTLOUT_DATA0_DIS_W<'_, GSTARFXDIS_SPEC> {
        IGNORE_CTLOUT_DATA0_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable the STAR fix added for Host controller to flag error for SSPLIT STALL/NYET The application programs SSPLIT_STALLNYET_Err_dis=1 to disable the host controller from reporting transaction error for STALL/NYET received from device for SSPLIT transfer. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ssplit_stallnyet_err_dis(&mut self) -> SSPLIT_STALLNYET_ERR_DIS_W<'_, GSTARFXDIS_SPEC> {
        SSPLIT_STALLNYET_ERR_DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Disable the STAR fix added for Host controller to accept DATA1 PID from device for ISOC Split transfers The application programs Accept_Isoc_split_DATA1_dis=1 to disable the host controller from accepting DATA1 PID from device for ISOC Split transfers. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn accept_isoc_split_data1_dis(
        &mut self,
    ) -> ACCEPT_ISOC_SPLIT_DATA1_DIS_W<'_, GSTARFXDIS_SPEC> {
        ACCEPT_ISOC_SPLIT_DATA1_DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable the STAR fix added for Host controller to handle Faulty cable scenarios The application programs HandleFaultyCable_dis=1 to disable the host controller from reporting Port Babble error when the linestate is stuck at 1 (in FS mode) or 2 (in LS mode) due to a Faulty cable. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn handlefaultycable_dis(&mut self) -> HANDLEFAULTYCABLE_DIS_W<'_, GSTARFXDIS_SPEC> {
        HANDLEFAULTYCABLE_DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable the STAR fix added for Host controller LS mode IPG increment from 2 LS bit times to 3 LS bit times The application programs LS_IPG_incr_dis=1 to disable the IPG increment from 2 to 3 LS bit times when host controller is operating in LS mode. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ls_ipg_incr_dis(&mut self) -> LS_IPG_INCR_DIS_W<'_, GSTARFXDIS_SPEC> {
        LS_IPG_INCR_DIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Disable the STAR fix added for Device controller to transition to IDLE state during FS device disconnect The application programs FSDisc_Idle_dis=1 to disable the Transmit/Receive state machine from moving to IDLE state when FS device disconnect happens. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn fsdisc_idle_dis(&mut self) -> FSDISC_IDLE_DIS_W<'_, GSTARFXDIS_SPEC> {
        FSDISC_IDLE_DIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Disable the STAR fix added for Device controller to not start Remote Wakeup signalling when USB resume has already started The application programs Concurrent_Rmtwkup_USBResume_dis=1 to allow the controller to do remote wakeup signalling when USB resume has already started from the Host. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn concurrent_rmtwkup_usbresume_dis(
        &mut self,
    ) -> CONCURRENT_RMTWKUP_USBRESUME_DIS_W<'_, GSTARFXDIS_SPEC> {
        CONCURRENT_RMTWKUP_USBRESUME_DIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable the STAR fix added for Device controller to not hang when Remote Wakeup signalling clashes with Host resume The application can program Concurrent_Rmtwkup_USBResume_Hib_dis=1 to disable the fix. Note: With Concurrent_Rmtwkup_USBResume_Hib_dis=0, the STAR fix is enabled and the device controller does not hang when the remote wakeup clashes with host resume. After the remote wakeup programming is done, if there was no utmi_txready received from the PHY while the utmi_txvalid was asserted, then the controller detects the line K as the host resume and directly moves to the RESUMING state, along with de-asserting utmi_txvalid. This prevents the controller from hanging but also violates the UTMI spec requirement which states that utmi_txvalid must be de-asserted only after getting utmi_txready. Hence, if any misbehaviour is observed with this fix enabled, it is recommended to keep the fix disabled. This bit must be programmed during the initial configuration of the controller and must not be modified later. The fix is applicable only for Hibernation case."]
    #[inline(always)]
    pub fn concurrent_rmtwkup_usbresume_hib_dis(
        &mut self,
    ) -> CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_W<'_, GSTARFXDIS_SPEC> {
        CONCURRENT_RMTWKUP_USBRESUME_HIB_DIS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Disable the STAR fix added for Host controller to wait for IPG duration to send next token after receiving NAK/STALL for previous IN token with FS/LS device The application programs LS_IPG_chk_after_NAK_STALL_for_IN_dis=1 to allow the controller to send the next token without waiting for the interpacket gap duration after receiving a NAK/STALL from device for the previous IN token. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ls_ipg_chk_after_nak_stall_for_in_dis(
        &mut self,
    ) -> LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_W<'_, GSTARFXDIS_SPEC> {
        LS_IPG_CHK_AFTER_NAK_STALL_FOR_IN_DIS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Disable the STAR fix added for Host controller to increase the gap between utmi_xcvrselect switching and utmi_txvalid assertion in LS/FS mode The application programs PHY_IOp_xcvrsel_txvld_corr_dis=1 to allow the controller to assert utmi_txvalid 1 cycle after the utmi_xcvrselect switching. This can cause interop issues with SNPS PHY which requires at least 2 cycles of gap between the utmi_xcvrselect switch and the utmi_txvalid assertion. Hence it is recommended to program PHY_IOp_xcvrsel_txvld_corr_dis=0. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn phy_iop_xcvrsel_txvld_corr_dis(
        &mut self,
    ) -> PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        PHY_IOP_XCVRSEL_TXVLD_CORR_DIS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Disable the STAR fix added for Host controller to increase the preamble transceiver select switch delay to accommodate time taken for ULPI function control write The application programs ULPI_xcvrsel_switch_corr_dis=1 to allow the controller to do back to back transceiver select switching within the duration required for a single ULPI function control write (around 5-6 ulpi_clk cycles). This can cause a corrupted packet to go on the bus since the ULPI wrapper may not be able to do the second ULPI functional update (register write) correctly. Hence it is recommended to program ULPI_xcvrsel_switch_corr_dis=0. With ULPI_xcvrsel_switch_corr_dis=0, the delay between transceiver select switch and txvalid assertion has also been increased for FS/LS mode to resolve PHY interop issues. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn ulpi_xcvrsel_switch_corr_dis(
        &mut self,
    ) -> ULPI_XCVRSEL_SWITCH_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        ULPI_XCVRSEL_SWITCH_CORR_DIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable the STAR fix added for Host controller to report transaction error when DATA0 PID is received for CTRL STATUS IN transfer in DMA mode The application programs XactErr_DATA0_CTRL_STS_IN_dis=1 to allow the controller to respond with ACK for the incorrect DATA0 PID and then retry the control STATUS IN transfer until DATA1 PID is received. With XactErr_DATA0_CTRL_STS_IN_dis=0, the controller generates Transaction Error interrupt when the incorrect PID is received. This STAR fix is applicable only for Buffer DMA and Descriptor DMA modes. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn xacterr_data0_ctrl_sts_in_dis(
        &mut self,
    ) -> XACTERR_DATA0_CTRL_STS_IN_DIS_W<'_, GSTARFXDIS_SPEC> {
        XACTERR_DATA0_CTRL_STS_IN_DIS_W::new(self, 13)
    }
    #[doc = "Bit 15 - Disable the STAR fix added for correcting XcvrSel on UTMI Interface in host mode. When host controller drives reset in power down suspend state, the behavior of XcvrSel has been corrected to drive the valid combination of XcvrSel (HS_XCVR) and TermSel (HS_TERM) on the UTMI interface. The application can set this register bit to 1'b1 to fall back to the original behavior of the Host controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when reset is driven to exit from power down state. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_xcvrsel_resetaftrsusp_corr_dis(
        &mut self,
    ) -> HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        HOST_XCVRSEL_RESETAFTRSUSP_CORR_DIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface in Host mode. When HPRT.PrtRst is set by the application, the Host controller can change the Opmode, XcvrSel and TermSel while TxValid is still high (1'b1) during SOF transmission in 8-bit UTMI mode. This behavior of the controller has been corrected to handle port reset during SOF transmission. With the fix, the controller waits for Txvalid to go low (1'b0) and then changes the OpMode, XcvrSel and TermSel. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_utmi_txvld_corr_dis(&mut self) -> HOST_UTMI_TXVLD_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        HOST_UTMI_TXVLD_CORR_DIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Disable the STAR fix added for correcting Opmode and XcvrSel on UTMI Interface when reset is detected in suspend state. In configurations with Battery charger support enabled (and GOTGCTL.ChirpEn programmed to 1), the RTL has been updated to wait for 1ms and then change the opmode and xcvrselect when reset is detected in the suspend state. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Opmode and XcvrSel without waiting for 1ms after reset detection. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn opmode_xcvrsel_chirpen_corr_dis(
        &mut self,
    ) -> OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        OPMODE_XCVRSEL_CHIRPEN_CORR_DIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Disable the STAR fix added for correcting Txvalid deassertion on UTMI Interface when soft disconnect is done. The RTL has been updated to wait for utmi_txready and then de-assert the utmi_txvalid being driven by MAC DSSR when soft disconnect is done. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller and change Txvalid without waiting for Txready. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn txvalid_deassertion_corr_dis(
        &mut self,
    ) -> TXVALID_DEASSERTION_CORR_DIS_W<'_, GSTARFXDIS_SPEC> {
        TXVALID_DEASSERTION_CORR_DIS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Disable the STAR fix added for correcting Host behavior when port is disabled. The RTL has been updated to not assert utmi_txvalid when the port is disabled. The application can set this register bit to 1'b1 to fall back to the original behavior of the controller. This bit must be programmed during the initial configuration of the controller and must not be modified later."]
    #[inline(always)]
    pub fn host_no_xfer_after_prtdis_fix_dis(
        &mut self,
    ) -> HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_W<'_, GSTARFXDIS_SPEC> {
        HOST_NO_XFER_AFTER_PRTDIS_FIX_DIS_W::new(self, 19)
    }
}
#[doc = "This register is used to disable STAR fixes added in the controller. The application can set the register fields to operate with the functionality before the fix was done.\n\nYou can [`read`](crate::Reg::read) this register and get [`gstarfxdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gstarfxdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSTARFXDIS_SPEC;
impl crate::RegisterSpec for GSTARFXDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gstarfxdis::R`](R) reader structure"]
impl crate::Readable for GSTARFXDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gstarfxdis::W`](W) writer structure"]
impl crate::Writable for GSTARFXDIS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSTARFXDIS to value 0x2200"]
impl crate::Resettable for GSTARFXDIS_SPEC {
    const RESET_VALUE: u32 = 0x2200;
}
