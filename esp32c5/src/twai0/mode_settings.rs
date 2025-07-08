#[doc = "Register `MODE_SETTINGS` reader"]
pub type R = crate::R<MODE_SETTINGS_SPEC>;
#[doc = "Register `MODE_SETTINGS` writer"]
pub type W = crate::W<MODE_SETTINGS_SPEC>;
#[doc = "Field `RST` writer - Soft reset. Writing logic 1 resets CTU CAN FD. After writing logic 1, logic 0 does not need to be written, this bit is automatically cleared. 0: invalid 1: reset."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMM` reader - Bus monitoring mode. In this mode CTU CAN FD only receives frames and sends only recessive bits on CAN bus. When a dominant bit is sent, it is re-routed internally so that bus value is not changed. When this mode is enabled, CTU CAN FD will not transmit any frame from TXT Buffers, 0b0 - BMM_DISABLED - Bus monitoring mode disabled. 0b1 - BMM_ENABLED - Bus monitoring mode enabled."]
pub type BMM_R = crate::BitReader;
#[doc = "Field `BMM` writer - Bus monitoring mode. In this mode CTU CAN FD only receives frames and sends only recessive bits on CAN bus. When a dominant bit is sent, it is re-routed internally so that bus value is not changed. When this mode is enabled, CTU CAN FD will not transmit any frame from TXT Buffers, 0b0 - BMM_DISABLED - Bus monitoring mode disabled. 0b1 - BMM_ENABLED - Bus monitoring mode enabled."]
pub type BMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STM` reader - Self Test Mode. In this mode transmitted frame is considered valid even if dominant acknowledge was not received. 0b0 - STM_DISABLED - Self test mode disabled. 0b1 - STM_ENABLED - Self test mode enabled."]
pub type STM_R = crate::BitReader;
#[doc = "Field `STM` writer - Self Test Mode. In this mode transmitted frame is considered valid even if dominant acknowledge was not received. 0b0 - STM_DISABLED - Self test mode disabled. 0b1 - STM_ENABLED - Self test mode enabled."]
pub type STM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFM` reader - Acceptance Filters Mode. If enabled, only RX frames which pass Frame filters are stored in RX buffer. If disabled, every received frame is stored to RX buffer. This bit has meaning only if there is at least one filter available. Otherwise, this bit is reserved. 0b0 - AFM_DISABLED - Acceptance filter mode disabled 0b1 - AFM_ENABLED - Acceptance filter mode enabled"]
pub type AFM_R = crate::BitReader;
#[doc = "Field `AFM` writer - Acceptance Filters Mode. If enabled, only RX frames which pass Frame filters are stored in RX buffer. If disabled, every received frame is stored to RX buffer. This bit has meaning only if there is at least one filter available. Otherwise, this bit is reserved. 0b0 - AFM_DISABLED - Acceptance filter mode disabled 0b1 - AFM_ENABLED - Acceptance filter mode enabled"]
pub type AFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDE` reader - Flexible data rate enable. When flexible data rate is enabled CTU CAN FD recognizes CAN FD frames (FDF bit = 1). 0b0 - FDE_DISABLE - Flexible data-rate support disabled. 0b1 - FDE_ENABLE - Flexible data-rate support enabled."]
pub type FDE_R = crate::BitReader;
#[doc = "Field `FDE` writer - Flexible data rate enable. When flexible data rate is enabled CTU CAN FD recognizes CAN FD frames (FDF bit = 1). 0b0 - FDE_DISABLE - Flexible data-rate support disabled. 0b1 - FDE_ENABLE - Flexible data-rate support enabled."]
pub type FDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTTM` reader - Time triggered transmission mode. 0b0 - TTTM_DISABLED - 0b1 - TTTM_ENABLED -"]
pub type TTTM_R = crate::BitReader;
#[doc = "Field `TTTM` writer - Time triggered transmission mode. 0b0 - TTTM_DISABLED - 0b1 - TTTM_ENABLED -"]
pub type TTTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - Restricted operation mode. 0b0 - ROM_DISABLED - Restricted operation mode is disabled. 0b1 - ROM_ENABLED - Restricted operation mode is enabled."]
pub type ROM_R = crate::BitReader;
#[doc = "Field `ROM` writer - Restricted operation mode. 0b0 - ROM_DISABLED - Restricted operation mode is disabled. 0b1 - ROM_ENABLED - Restricted operation mode is enabled."]
pub type ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACF` reader - Acknowledge Forbidden Mode. When enabled, acknowledge is not sent even if received CRC matches the calculated one. 0b0 - ACF_DISABLED - Acknowledge forbidden mode disabled. 0b1 - ACF_ENABLED - Acknowledge forbidden mode enabled."]
pub type ACF_R = crate::BitReader;
#[doc = "Field `ACF` writer - Acknowledge Forbidden Mode. When enabled, acknowledge is not sent even if received CRC matches the calculated one. 0b0 - ACF_DISABLED - Acknowledge forbidden mode disabled. 0b1 - ACF_ENABLED - Acknowledge forbidden mode enabled."]
pub type ACF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTM` reader - Test Mode. In test mode several registers have special features. Reffer to description of Test mode for further Details."]
pub type TSTM_R = crate::BitReader;
#[doc = "Field `TSTM` writer - Test Mode. In test mode several registers have special features. Reffer to description of Test mode for further Details."]
pub type TSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBAM` reader - RX Buffer automatic mode. 0b0 - RXBAM_DISABLED - 0b1 - RXBAM_ENABLED -"]
pub type RXBAM_R = crate::BitReader;
#[doc = "Field `RXBAM` writer - RX Buffer automatic mode. 0b0 - RXBAM_DISABLED - 0b1 - RXBAM_ENABLED -"]
pub type RXBAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBBM` reader - TXT Buffer Backup mode\\\\ 0b0 - TXBBM_DISABLED - TXT Buffer Backup mode disabled\\\\ 0b1 - TXBBM_ENABLED - TXT Buffer Backup mode enabled\\\\"]
pub type TXBBM_R = crate::BitReader;
#[doc = "Field `TXBBM` writer - TXT Buffer Backup mode\\\\ 0b0 - TXBBM_DISABLED - TXT Buffer Backup mode disabled\\\\ 0b1 - TXBBM_ENABLED - TXT Buffer Backup mode enabled\\\\"]
pub type TXBBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAM` reader - Self-acknowledge mode.\\\\ 0b0 - SAM_DISABLE - Do not send dominant ACK bit when CTU CAN FD sends Acknowledge bit.\\\\ 0b1 - SAM_ENABLE - Send dominant ACK bit when CTU CAN FD transmits CAN frame.\\\\"]
pub type SAM_R = crate::BitReader;
#[doc = "Field `SAM` writer - Self-acknowledge mode.\\\\ 0b0 - SAM_DISABLE - Do not send dominant ACK bit when CTU CAN FD sends Acknowledge bit.\\\\ 0b1 - SAM_ENABLE - Send dominant ACK bit when CTU CAN FD transmits CAN frame.\\\\"]
pub type SAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRLE` reader - Retransmitt Limit Enable. If enabled, CTU CAN FD only attempts to retransmitt each frame up to RTR_TH times. 0b0 - RTRLE_DISABLED - Retransmitt limit is disabled. 0b1 - RTRLE_ENABLED - Retransmitt limit is enabled."]
pub type RTRLE_R = crate::BitReader;
#[doc = "Field `RTRLE` writer - Retransmitt Limit Enable. If enabled, CTU CAN FD only attempts to retransmitt each frame up to RTR_TH times. 0b0 - RTRLE_DISABLED - Retransmitt limit is disabled. 0b1 - RTRLE_ENABLED - Retransmitt limit is enabled."]
pub type RTRLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRTH` reader - Retransmitt Limit Threshold. Maximal amount of retransmission attempts when SETTINGS\\[RTRLE\\] is en- Abled."]
pub type RTRTH_R = crate::FieldReader;
#[doc = "Field `RTRTH` writer - Retransmitt Limit Threshold. Maximal amount of retransmission attempts when SETTINGS\\[RTRLE\\] is en- Abled."]
pub type RTRTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ILBP` reader - Internal Loop Back mode. When enabled, CTU CAN FD receives any frame it transmitts. 0b0 - INT_LOOP_DISABLED - Internal loop-back is disabled. 0b1 - INT_LOOP_ENABLED - Internal loop-back is enabled."]
pub type ILBP_R = crate::BitReader;
#[doc = "Field `ILBP` writer - Internal Loop Back mode. When enabled, CTU CAN FD receives any frame it transmitts. 0b0 - INT_LOOP_DISABLED - Internal loop-back is disabled. 0b1 - INT_LOOP_ENABLED - Internal loop-back is enabled."]
pub type ILBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA` reader - Main enable bit of CTU CAN FD. When enabled, CTU CAN FD communicates on CAN bus. When disabled, it is bus-off and does not take part of CAN bus communication. 0b0 - CTU_CAN_DISABLED - The CAN Core is disabled. 0b1 - CTU_CAN_ENABLED - The CAN Core is enabled."]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - Main enable bit of CTU CAN FD. When enabled, CTU CAN FD communicates on CAN bus. When disabled, it is bus-off and does not take part of CAN bus communication. 0b0 - CTU_CAN_DISABLED - The CAN Core is disabled. 0b1 - CTU_CAN_ENABLED - The CAN Core is enabled."]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISOFD` reader - Non ISO FD. When this bit is set, CTU CAN FD is compliant to NON-ISO CAN FD specification (no stuff count field). This bit should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - ISO_FD - The CAN Controller conforms to ISO CAN FD specification. 0b1 - NON_ISO_FD - The CAN Controller conforms to NON ISO CAN FD specification. CANFD 1.0"]
pub type NISOFD_R = crate::BitReader;
#[doc = "Field `NISOFD` writer - Non ISO FD. When this bit is set, CTU CAN FD is compliant to NON-ISO CAN FD specification (no stuff count field). This bit should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - ISO_FD - The CAN Controller conforms to ISO CAN FD specification. 0b1 - NON_ISO_FD - The CAN Controller conforms to NON ISO CAN FD specification. CANFD 1.0"]
pub type NISOFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEX` reader - Protocol exception handling. When this bit is set, CTU CAN FD will start integration upon detection of protocol exception. This should be modified only when SETTINGS\\[ENA\\] = ’0’. 0b0 - PROTOCOL_EXCEPTION_DISABLED - Protocol exception handling is disabled. 0b1 - PROTOCOL_EXCEPTION_ENABLED - Protocol exception handling is enabled."]
pub type PEX_R = crate::BitReader;
#[doc = "Field `PEX` writer - Protocol exception handling. When this bit is set, CTU CAN FD will start integration upon detection of protocol exception. This should be modified only when SETTINGS\\[ENA\\] = ’0’. 0b0 - PROTOCOL_EXCEPTION_DISABLED - Protocol exception handling is disabled. 0b1 - PROTOCOL_EXCEPTION_ENABLED - Protocol exception handling is enabled."]
pub type PEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBFBO` reader - All TXT buffers shall go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b0 - TXTBUF_FAILED_BUS_OFF_DISABLED - TXT Buffers dont go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b1 - TXTBUF_FAILED_BUS_OFF_ENABLED - TXT Buffers go to \"TX failed\" state when CTU CAN FD becomes bus-off."]
pub type TBFBO_R = crate::BitReader;
#[doc = "Field `TBFBO` writer - All TXT buffers shall go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b0 - TXTBUF_FAILED_BUS_OFF_DISABLED - TXT Buffers dont go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b1 - TXTBUF_FAILED_BUS_OFF_ENABLED - TXT Buffers go to \"TX failed\" state when CTU CAN FD becomes bus-off."]
pub type TBFBO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDRF` reader - Frame filters drop Remote frames. 0b0 - DROP_RF_DISABLED - Frame filters accept RTR frames. 0b1 - DROP_RF_ENABLED - Frame filters drop RTR frames."]
pub type FDRF_R = crate::BitReader;
#[doc = "Field `FDRF` writer - Frame filters drop Remote frames. 0b0 - DROP_RF_DISABLED - Frame filters accept RTR frames. 0b1 - DROP_RF_ENABLED - Frame filters drop RTR frames."]
pub type FDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCHKE` reader - Enable Parity checks in TXT Buffers and RX Buffer."]
pub type PCHKE_R = crate::BitReader;
#[doc = "Field `PCHKE` writer - Enable Parity checks in TXT Buffers and RX Buffer."]
pub type PCHKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Bus monitoring mode. In this mode CTU CAN FD only receives frames and sends only recessive bits on CAN bus. When a dominant bit is sent, it is re-routed internally so that bus value is not changed. When this mode is enabled, CTU CAN FD will not transmit any frame from TXT Buffers, 0b0 - BMM_DISABLED - Bus monitoring mode disabled. 0b1 - BMM_ENABLED - Bus monitoring mode enabled."]
    #[inline(always)]
    pub fn bmm(&self) -> BMM_R {
        BMM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode. In this mode transmitted frame is considered valid even if dominant acknowledge was not received. 0b0 - STM_DISABLED - Self test mode disabled. 0b1 - STM_ENABLED - Self test mode enabled."]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acceptance Filters Mode. If enabled, only RX frames which pass Frame filters are stored in RX buffer. If disabled, every received frame is stored to RX buffer. This bit has meaning only if there is at least one filter available. Otherwise, this bit is reserved. 0b0 - AFM_DISABLED - Acceptance filter mode disabled 0b1 - AFM_ENABLED - Acceptance filter mode enabled"]
    #[inline(always)]
    pub fn afm(&self) -> AFM_R {
        AFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flexible data rate enable. When flexible data rate is enabled CTU CAN FD recognizes CAN FD frames (FDF bit = 1). 0b0 - FDE_DISABLE - Flexible data-rate support disabled. 0b1 - FDE_ENABLE - Flexible data-rate support enabled."]
    #[inline(always)]
    pub fn fde(&self) -> FDE_R {
        FDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time triggered transmission mode. 0b0 - TTTM_DISABLED - 0b1 - TTTM_ENABLED -"]
    #[inline(always)]
    pub fn tttm(&self) -> TTTM_R {
        TTTM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Restricted operation mode. 0b0 - ROM_DISABLED - Restricted operation mode is disabled. 0b1 - ROM_ENABLED - Restricted operation mode is enabled."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge Forbidden Mode. When enabled, acknowledge is not sent even if received CRC matches the calculated one. 0b0 - ACF_DISABLED - Acknowledge forbidden mode disabled. 0b1 - ACF_ENABLED - Acknowledge forbidden mode enabled."]
    #[inline(always)]
    pub fn acf(&self) -> ACF_R {
        ACF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Test Mode. In test mode several registers have special features. Reffer to description of Test mode for further Details."]
    #[inline(always)]
    pub fn tstm(&self) -> TSTM_R {
        TSTM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX Buffer automatic mode. 0b0 - RXBAM_DISABLED - 0b1 - RXBAM_ENABLED -"]
    #[inline(always)]
    pub fn rxbam(&self) -> RXBAM_R {
        RXBAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXT Buffer Backup mode\\\\ 0b0 - TXBBM_DISABLED - TXT Buffer Backup mode disabled\\\\ 0b1 - TXBBM_ENABLED - TXT Buffer Backup mode enabled\\\\"]
    #[inline(always)]
    pub fn txbbm(&self) -> TXBBM_R {
        TXBBM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self-acknowledge mode.\\\\ 0b0 - SAM_DISABLE - Do not send dominant ACK bit when CTU CAN FD sends Acknowledge bit.\\\\ 0b1 - SAM_ENABLE - Send dominant ACK bit when CTU CAN FD transmits CAN frame.\\\\"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Retransmitt Limit Enable. If enabled, CTU CAN FD only attempts to retransmitt each frame up to RTR_TH times. 0b0 - RTRLE_DISABLED - Retransmitt limit is disabled. 0b1 - RTRLE_ENABLED - Retransmitt limit is enabled."]
    #[inline(always)]
    pub fn rtrle(&self) -> RTRLE_R {
        RTRLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Retransmitt Limit Threshold. Maximal amount of retransmission attempts when SETTINGS\\[RTRLE\\] is en- Abled."]
    #[inline(always)]
    pub fn rtrth(&self) -> RTRTH_R {
        RTRTH_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Internal Loop Back mode. When enabled, CTU CAN FD receives any frame it transmitts. 0b0 - INT_LOOP_DISABLED - Internal loop-back is disabled. 0b1 - INT_LOOP_ENABLED - Internal loop-back is enabled."]
    #[inline(always)]
    pub fn ilbp(&self) -> ILBP_R {
        ILBP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Main enable bit of CTU CAN FD. When enabled, CTU CAN FD communicates on CAN bus. When disabled, it is bus-off and does not take part of CAN bus communication. 0b0 - CTU_CAN_DISABLED - The CAN Core is disabled. 0b1 - CTU_CAN_ENABLED - The CAN Core is enabled."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Non ISO FD. When this bit is set, CTU CAN FD is compliant to NON-ISO CAN FD specification (no stuff count field). This bit should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - ISO_FD - The CAN Controller conforms to ISO CAN FD specification. 0b1 - NON_ISO_FD - The CAN Controller conforms to NON ISO CAN FD specification. CANFD 1.0"]
    #[inline(always)]
    pub fn nisofd(&self) -> NISOFD_R {
        NISOFD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protocol exception handling. When this bit is set, CTU CAN FD will start integration upon detection of protocol exception. This should be modified only when SETTINGS\\[ENA\\] = ’0’. 0b0 - PROTOCOL_EXCEPTION_DISABLED - Protocol exception handling is disabled. 0b1 - PROTOCOL_EXCEPTION_ENABLED - Protocol exception handling is enabled."]
    #[inline(always)]
    pub fn pex(&self) -> PEX_R {
        PEX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - All TXT buffers shall go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b0 - TXTBUF_FAILED_BUS_OFF_DISABLED - TXT Buffers dont go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b1 - TXTBUF_FAILED_BUS_OFF_ENABLED - TXT Buffers go to \"TX failed\" state when CTU CAN FD becomes bus-off."]
    #[inline(always)]
    pub fn tbfbo(&self) -> TBFBO_R {
        TBFBO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Frame filters drop Remote frames. 0b0 - DROP_RF_DISABLED - Frame filters accept RTR frames. 0b1 - DROP_RF_ENABLED - Frame filters drop RTR frames."]
    #[inline(always)]
    pub fn fdrf(&self) -> FDRF_R {
        FDRF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Parity checks in TXT Buffers and RX Buffer."]
    #[inline(always)]
    pub fn pchke(&self) -> PCHKE_R {
        PCHKE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_SETTINGS")
            .field("bmm", &self.bmm())
            .field("stm", &self.stm())
            .field("afm", &self.afm())
            .field("fde", &self.fde())
            .field("tttm", &self.tttm())
            .field("rom", &self.rom())
            .field("acf", &self.acf())
            .field("tstm", &self.tstm())
            .field("rxbam", &self.rxbam())
            .field("txbbm", &self.txbbm())
            .field("sam", &self.sam())
            .field("rtrle", &self.rtrle())
            .field("rtrth", &self.rtrth())
            .field("ilbp", &self.ilbp())
            .field("ena", &self.ena())
            .field("nisofd", &self.nisofd())
            .field("pex", &self.pex())
            .field("tbfbo", &self.tbfbo())
            .field("fdrf", &self.fdrf())
            .field("pchke", &self.pchke())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Soft reset. Writing logic 1 resets CTU CAN FD. After writing logic 1, logic 0 does not need to be written, this bit is automatically cleared. 0: invalid 1: reset."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<MODE_SETTINGS_SPEC> {
        RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bus monitoring mode. In this mode CTU CAN FD only receives frames and sends only recessive bits on CAN bus. When a dominant bit is sent, it is re-routed internally so that bus value is not changed. When this mode is enabled, CTU CAN FD will not transmit any frame from TXT Buffers, 0b0 - BMM_DISABLED - Bus monitoring mode disabled. 0b1 - BMM_ENABLED - Bus monitoring mode enabled."]
    #[inline(always)]
    pub fn bmm(&mut self) -> BMM_W<MODE_SETTINGS_SPEC> {
        BMM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Self Test Mode. In this mode transmitted frame is considered valid even if dominant acknowledge was not received. 0b0 - STM_DISABLED - Self test mode disabled. 0b1 - STM_ENABLED - Self test mode enabled."]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W<MODE_SETTINGS_SPEC> {
        STM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Acceptance Filters Mode. If enabled, only RX frames which pass Frame filters are stored in RX buffer. If disabled, every received frame is stored to RX buffer. This bit has meaning only if there is at least one filter available. Otherwise, this bit is reserved. 0b0 - AFM_DISABLED - Acceptance filter mode disabled 0b1 - AFM_ENABLED - Acceptance filter mode enabled"]
    #[inline(always)]
    pub fn afm(&mut self) -> AFM_W<MODE_SETTINGS_SPEC> {
        AFM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flexible data rate enable. When flexible data rate is enabled CTU CAN FD recognizes CAN FD frames (FDF bit = 1). 0b0 - FDE_DISABLE - Flexible data-rate support disabled. 0b1 - FDE_ENABLE - Flexible data-rate support enabled."]
    #[inline(always)]
    pub fn fde(&mut self) -> FDE_W<MODE_SETTINGS_SPEC> {
        FDE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Time triggered transmission mode. 0b0 - TTTM_DISABLED - 0b1 - TTTM_ENABLED -"]
    #[inline(always)]
    pub fn tttm(&mut self) -> TTTM_W<MODE_SETTINGS_SPEC> {
        TTTM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Restricted operation mode. 0b0 - ROM_DISABLED - Restricted operation mode is disabled. 0b1 - ROM_ENABLED - Restricted operation mode is enabled."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<MODE_SETTINGS_SPEC> {
        ROM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Acknowledge Forbidden Mode. When enabled, acknowledge is not sent even if received CRC matches the calculated one. 0b0 - ACF_DISABLED - Acknowledge forbidden mode disabled. 0b1 - ACF_ENABLED - Acknowledge forbidden mode enabled."]
    #[inline(always)]
    pub fn acf(&mut self) -> ACF_W<MODE_SETTINGS_SPEC> {
        ACF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Test Mode. In test mode several registers have special features. Reffer to description of Test mode for further Details."]
    #[inline(always)]
    pub fn tstm(&mut self) -> TSTM_W<MODE_SETTINGS_SPEC> {
        TSTM_W::new(self, 8)
    }
    #[doc = "Bit 9 - RX Buffer automatic mode. 0b0 - RXBAM_DISABLED - 0b1 - RXBAM_ENABLED -"]
    #[inline(always)]
    pub fn rxbam(&mut self) -> RXBAM_W<MODE_SETTINGS_SPEC> {
        RXBAM_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXT Buffer Backup mode\\\\ 0b0 - TXBBM_DISABLED - TXT Buffer Backup mode disabled\\\\ 0b1 - TXBBM_ENABLED - TXT Buffer Backup mode enabled\\\\"]
    #[inline(always)]
    pub fn txbbm(&mut self) -> TXBBM_W<MODE_SETTINGS_SPEC> {
        TXBBM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Self-acknowledge mode.\\\\ 0b0 - SAM_DISABLE - Do not send dominant ACK bit when CTU CAN FD sends Acknowledge bit.\\\\ 0b1 - SAM_ENABLE - Send dominant ACK bit when CTU CAN FD transmits CAN frame.\\\\"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W<MODE_SETTINGS_SPEC> {
        SAM_W::new(self, 11)
    }
    #[doc = "Bit 16 - Retransmitt Limit Enable. If enabled, CTU CAN FD only attempts to retransmitt each frame up to RTR_TH times. 0b0 - RTRLE_DISABLED - Retransmitt limit is disabled. 0b1 - RTRLE_ENABLED - Retransmitt limit is enabled."]
    #[inline(always)]
    pub fn rtrle(&mut self) -> RTRLE_W<MODE_SETTINGS_SPEC> {
        RTRLE_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - Retransmitt Limit Threshold. Maximal amount of retransmission attempts when SETTINGS\\[RTRLE\\] is en- Abled."]
    #[inline(always)]
    pub fn rtrth(&mut self) -> RTRTH_W<MODE_SETTINGS_SPEC> {
        RTRTH_W::new(self, 17)
    }
    #[doc = "Bit 21 - Internal Loop Back mode. When enabled, CTU CAN FD receives any frame it transmitts. 0b0 - INT_LOOP_DISABLED - Internal loop-back is disabled. 0b1 - INT_LOOP_ENABLED - Internal loop-back is enabled."]
    #[inline(always)]
    pub fn ilbp(&mut self) -> ILBP_W<MODE_SETTINGS_SPEC> {
        ILBP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Main enable bit of CTU CAN FD. When enabled, CTU CAN FD communicates on CAN bus. When disabled, it is bus-off and does not take part of CAN bus communication. 0b0 - CTU_CAN_DISABLED - The CAN Core is disabled. 0b1 - CTU_CAN_ENABLED - The CAN Core is enabled."]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<MODE_SETTINGS_SPEC> {
        ENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - Non ISO FD. When this bit is set, CTU CAN FD is compliant to NON-ISO CAN FD specification (no stuff count field). This bit should be modified only when SETTINGS\\[ENA\\]=0. 0b0 - ISO_FD - The CAN Controller conforms to ISO CAN FD specification. 0b1 - NON_ISO_FD - The CAN Controller conforms to NON ISO CAN FD specification. CANFD 1.0"]
    #[inline(always)]
    pub fn nisofd(&mut self) -> NISOFD_W<MODE_SETTINGS_SPEC> {
        NISOFD_W::new(self, 23)
    }
    #[doc = "Bit 24 - Protocol exception handling. When this bit is set, CTU CAN FD will start integration upon detection of protocol exception. This should be modified only when SETTINGS\\[ENA\\] = ’0’. 0b0 - PROTOCOL_EXCEPTION_DISABLED - Protocol exception handling is disabled. 0b1 - PROTOCOL_EXCEPTION_ENABLED - Protocol exception handling is enabled."]
    #[inline(always)]
    pub fn pex(&mut self) -> PEX_W<MODE_SETTINGS_SPEC> {
        PEX_W::new(self, 24)
    }
    #[doc = "Bit 25 - All TXT buffers shall go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b0 - TXTBUF_FAILED_BUS_OFF_DISABLED - TXT Buffers dont go to \"TX failed\" state when CTU CAN FD becomes bus-off. 0b1 - TXTBUF_FAILED_BUS_OFF_ENABLED - TXT Buffers go to \"TX failed\" state when CTU CAN FD becomes bus-off."]
    #[inline(always)]
    pub fn tbfbo(&mut self) -> TBFBO_W<MODE_SETTINGS_SPEC> {
        TBFBO_W::new(self, 25)
    }
    #[doc = "Bit 26 - Frame filters drop Remote frames. 0b0 - DROP_RF_DISABLED - Frame filters accept RTR frames. 0b1 - DROP_RF_ENABLED - Frame filters drop RTR frames."]
    #[inline(always)]
    pub fn fdrf(&mut self) -> FDRF_W<MODE_SETTINGS_SPEC> {
        FDRF_W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Parity checks in TXT Buffers and RX Buffer."]
    #[inline(always)]
    pub fn pchke(&mut self) -> PCHKE_W<MODE_SETTINGS_SPEC> {
        PCHKE_W::new(self, 27)
    }
}
#[doc = "TWAI FD mode setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_settings::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_settings::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SETTINGS_SPEC;
impl crate::RegisterSpec for MODE_SETTINGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_settings::R`](R) reader structure"]
impl crate::Readable for MODE_SETTINGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode_settings::W`](W) writer structure"]
impl crate::Writable for MODE_SETTINGS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE_SETTINGS to value 0x0200_0210"]
impl crate::Resettable for MODE_SETTINGS_SPEC {
    const RESET_VALUE: u32 = 0x0200_0210;
}
