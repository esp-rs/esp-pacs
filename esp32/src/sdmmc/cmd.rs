#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - Command index."]
pub type INDEX_R = crate::FieldReader;
#[doc = "Field `INDEX` writer - Command index."]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_SPEC, 6, O>;
#[doc = "Field `RESPONSE_EXPECT` reader - 0: No response expected from card; 1: Response expected from card."]
pub type RESPONSE_EXPECT_R = crate::BitReader;
#[doc = "Field `RESPONSE_EXPECT` writer - 0: No response expected from card; 1: Response expected from card."]
pub type RESPONSE_EXPECT_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `RESPONSE_LENGTH` reader - 0: Short response expected from card; 1: Long response expected from card."]
pub type RESPONSE_LENGTH_R = crate::BitReader;
#[doc = "Field `RESPONSE_LENGTH` writer - 0: Short response expected from card; 1: Long response expected from card."]
pub type RESPONSE_LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `CHECK_RESPONSE_CRC` reader - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub type CHECK_RESPONSE_CRC_R = crate::BitReader;
#[doc = "Field `CHECK_RESPONSE_CRC` writer - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub type CHECK_RESPONSE_CRC_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `DATA_EXPECTED` reader - 0: No data transfer expected; 1: Data transfer expected."]
pub type DATA_EXPECTED_R = crate::BitReader;
#[doc = "Field `DATA_EXPECTED` writer - 0: No data transfer expected; 1: Data transfer expected."]
pub type DATA_EXPECTED_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `READ_WRITE` reader - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub type READ_WRITE_R = crate::BitReader;
#[doc = "Field `READ_WRITE` writer - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub type READ_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `TRANSFER_MODE` reader - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub type TRANSFER_MODE_R = crate::BitReader;
#[doc = "Field `TRANSFER_MODE` writer - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub type TRANSFER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `SEND_AUTO_STOP` reader - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub type SEND_AUTO_STOP_R = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP` writer - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub type SEND_AUTO_STOP_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub type WAIT_PRVDATA_COMPLETE_R = crate::BitReader;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub type WAIT_PRVDATA_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `STOP_ABORT_CMD` reader - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub type STOP_ABORT_CMD_R = crate::BitReader;
#[doc = "Field `STOP_ABORT_CMD` writer - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub type STOP_ABORT_CMD_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `SEND_INITIALIZATION` reader - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub type SEND_INITIALIZATION_R = crate::BitReader;
#[doc = "Field `SEND_INITIALIZATION` writer - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub type SEND_INITIALIZATION_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `CARD_NUMBER` reader - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub type CARD_NUMBER_R = crate::FieldReader;
#[doc = "Field `CARD_NUMBER` writer - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub type CARD_NUMBER_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_SPEC, 5, O>;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_R = crate::BitReader;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `READ_CEATA_DEVICE` reader - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub type READ_CEATA_DEVICE_R = crate::BitReader;
#[doc = "Field `READ_CEATA_DEVICE` writer - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub type READ_CEATA_DEVICE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `CCS_EXPECTED` reader - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub type CCS_EXPECTED_R = crate::BitReader;
#[doc = "Field `CCS_EXPECTED` writer - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub type CCS_EXPECTED_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `USE_HOLE` reader - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub type USE_HOLE_R = crate::BitReader;
#[doc = "Field `USE_HOLE` writer - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub type USE_HOLE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `START_CMD` reader - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub type START_CMD_R = crate::BitReader;
#[doc = "Field `START_CMD` writer - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub type START_CMD_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    pub fn card_number(&self) -> CARD_NUMBER_R {
        CARD_NUMBER_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    pub fn use_hole(&self) -> USE_HOLE_R {
        USE_HOLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("index", &format_args!("{}", self.index().bits()))
            .field(
                "response_expect",
                &format_args!("{}", self.response_expect().bit()),
            )
            .field(
                "response_length",
                &format_args!("{}", self.response_length().bit()),
            )
            .field(
                "check_response_crc",
                &format_args!("{}", self.check_response_crc().bit()),
            )
            .field(
                "data_expected",
                &format_args!("{}", self.data_expected().bit()),
            )
            .field("read_write", &format_args!("{}", self.read_write().bit()))
            .field(
                "transfer_mode",
                &format_args!("{}", self.transfer_mode().bit()),
            )
            .field(
                "send_auto_stop",
                &format_args!("{}", self.send_auto_stop().bit()),
            )
            .field(
                "wait_prvdata_complete",
                &format_args!("{}", self.wait_prvdata_complete().bit()),
            )
            .field(
                "stop_abort_cmd",
                &format_args!("{}", self.stop_abort_cmd().bit()),
            )
            .field(
                "send_initialization",
                &format_args!("{}", self.send_initialization().bit()),
            )
            .field(
                "card_number",
                &format_args!("{}", self.card_number().bits()),
            )
            .field(
                "update_clock_registers_only",
                &format_args!("{}", self.update_clock_registers_only().bit()),
            )
            .field(
                "read_ceata_device",
                &format_args!("{}", self.read_ceata_device().bit()),
            )
            .field(
                "ccs_expected",
                &format_args!("{}", self.ccs_expected().bit()),
            )
            .field("use_hole", &format_args!("{}", self.use_hole().bit()))
            .field("start_cmd", &format_args!("{}", self.start_cmd().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<0> {
        INDEX_W::new(self)
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    #[must_use]
    pub fn response_expect(&mut self) -> RESPONSE_EXPECT_W<6> {
        RESPONSE_EXPECT_W::new(self)
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    #[must_use]
    pub fn response_length(&mut self) -> RESPONSE_LENGTH_W<7> {
        RESPONSE_LENGTH_W::new(self)
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    #[must_use]
    pub fn check_response_crc(&mut self) -> CHECK_RESPONSE_CRC_W<8> {
        CHECK_RESPONSE_CRC_W::new(self)
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    #[must_use]
    pub fn data_expected(&mut self) -> DATA_EXPECTED_W<9> {
        DATA_EXPECTED_W::new(self)
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    #[must_use]
    pub fn read_write(&mut self) -> READ_WRITE_W<10> {
        READ_WRITE_W::new(self)
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_mode(&mut self) -> TRANSFER_MODE_W<11> {
        TRANSFER_MODE_W::new(self)
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop(&mut self) -> SEND_AUTO_STOP_W<12> {
        SEND_AUTO_STOP_W::new(self)
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE\\] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    #[must_use]
    pub fn wait_prvdata_complete(&mut self) -> WAIT_PRVDATA_COMPLETE_W<13> {
        WAIT_PRVDATA_COMPLETE_W::new(self)
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    #[must_use]
    pub fn stop_abort_cmd(&mut self) -> STOP_ABORT_CMD_W<14> {
        STOP_ABORT_CMD_W::new(self)
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    #[must_use]
    pub fn send_initialization(&mut self) -> SEND_INITIALIZATION_W<15> {
        SEND_INITIALIZATION_W::new(self)
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    #[must_use]
    pub fn card_number(&mut self) -> CARD_NUMBER_W<16> {
        CARD_NUMBER_W::new(self)
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    #[must_use]
    pub fn update_clock_registers_only(&mut self) -> UPDATE_CLOCK_REGISTERS_ONLY_W<21> {
        UPDATE_CLOCK_REGISTERS_ONLY_W::new(self)
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    #[must_use]
    pub fn read_ceata_device(&mut self) -> READ_CEATA_DEVICE_W<22> {
        READ_CEATA_DEVICE_W::new(self)
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn ccs_expected(&mut self) -> CCS_EXPECTED_W<23> {
        CCS_EXPECTED_W::new(self)
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    #[must_use]
    pub fn use_hole(&mut self) -> USE_HOLE_W<29> {
        USE_HOLE_W::new(self)
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    #[must_use]
    pub fn start_cmd(&mut self) -> START_CMD_W<31> {
        START_CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command and boot configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0x2000_0000"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
