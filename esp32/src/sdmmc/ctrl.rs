#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROLLER_RESET` reader - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
pub type CONTROLLER_RESET_R = crate::BitReader;
#[doc = "Field `CONTROLLER_RESET` writer - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
pub type CONTROLLER_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FIFO_RESET` reader - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
pub type FIFO_RESET_R = crate::BitReader;
#[doc = "Field `FIFO_RESET` writer - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
pub type FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `DMA_RESET` reader - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
pub type DMA_RESET_R = crate::BitReader;
#[doc = "Field `DMA_RESET` writer - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
pub type DMA_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
pub type INT_ENABLE_R = crate::BitReader;
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
pub type INT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `READ_WAIT` reader - For sending read-wait to SDIO cards."]
pub type READ_WAIT_R = crate::BitReader;
#[doc = "Field `READ_WAIT` writer - For sending read-wait to SDIO cards."]
pub type READ_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SEND_IRQ_RESPONSE` reader - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
pub type SEND_IRQ_RESPONSE_R = crate::BitReader;
#[doc = "Field `SEND_IRQ_RESPONSE` writer - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
pub type SEND_IRQ_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `ABORT_READ_DATA` reader - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
pub type ABORT_READ_DATA_R = crate::BitReader;
#[doc = "Field `ABORT_READ_DATA` writer - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
pub type ABORT_READ_DATA_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SEND_CCSD` reader - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
pub type SEND_CCSD_R = crate::BitReader;
#[doc = "Field `SEND_CCSD` writer - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
pub type SEND_CCSD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
pub type SEND_AUTO_STOP_CCSD_R = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
pub type SEND_AUTO_STOP_CCSD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_R = crate::BitReader;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
    #[inline(always)]
    pub fn controller_reset(&self) -> CONTROLLER_RESET_R {
        CONTROLLER_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FIFO_RESET_R {
        FIFO_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline(always)]
    pub fn dma_reset(&self) -> DMA_RESET_R {
        DMA_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - For sending read-wait to SDIO cards."]
    #[inline(always)]
    pub fn read_wait(&self) -> READ_WAIT_R {
        READ_WAIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SEND_IRQ_RESPONSE_R {
        SEND_IRQ_RESPONSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
    #[inline(always)]
    pub fn abort_read_data(&self) -> ABORT_READ_DATA_R {
        ABORT_READ_DATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SEND_CCSD_R {
        SEND_CCSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SEND_AUTO_STOP_CCSD_R {
        SEND_AUTO_STOP_CCSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CEATA_DEVICE_INTERRUPT_STATUS_R {
        CEATA_DEVICE_INTERRUPT_STATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field(
                "controller_reset",
                &format_args!("{}", self.controller_reset().bit()),
            )
            .field("fifo_reset", &format_args!("{}", self.fifo_reset().bit()))
            .field("dma_reset", &format_args!("{}", self.dma_reset().bit()))
            .field("int_enable", &format_args!("{}", self.int_enable().bit()))
            .field("read_wait", &format_args!("{}", self.read_wait().bit()))
            .field(
                "send_irq_response",
                &format_args!("{}", self.send_irq_response().bit()),
            )
            .field(
                "abort_read_data",
                &format_args!("{}", self.abort_read_data().bit()),
            )
            .field("send_ccsd", &format_args!("{}", self.send_ccsd().bit()))
            .field(
                "send_auto_stop_ccsd",
                &format_args!("{}", self.send_auto_stop_ccsd().bit()),
            )
            .field(
                "ceata_device_interrupt_status",
                &format_args!("{}", self.ceata_device_interrupt_status().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn controller_reset(&mut self) -> CONTROLLER_RESET_W<0> {
        CONTROLLER_RESET_W::new(self)
    }
    #[doc = "Bit 1 - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_reset(&mut self) -> FIFO_RESET_W<1> {
        FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 2 - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline(always)]
    #[must_use]
    pub fn dma_reset(&mut self) -> DMA_RESET_W<2> {
        DMA_RESET_W::new(self)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> INT_ENABLE_W<4> {
        INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - For sending read-wait to SDIO cards."]
    #[inline(always)]
    #[must_use]
    pub fn read_wait(&mut self) -> READ_WAIT_W<6> {
        READ_WAIT_W::new(self)
    }
    #[doc = "Bit 7 - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
    #[inline(always)]
    #[must_use]
    pub fn send_irq_response(&mut self) -> SEND_IRQ_RESPONSE_W<7> {
        SEND_IRQ_RESPONSE_W::new(self)
    }
    #[doc = "Bit 8 - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
    #[inline(always)]
    #[must_use]
    pub fn abort_read_data(&mut self) -> ABORT_READ_DATA_W<8> {
        ABORT_READ_DATA_W::new(self)
    }
    #[doc = "Bit 9 - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
    #[inline(always)]
    #[must_use]
    pub fn send_ccsd(&mut self) -> SEND_CCSD_W<9> {
        SEND_CCSD_W::new(self)
    }
    #[doc = "Bit 10 - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop_ccsd(&mut self) -> SEND_AUTO_STOP_CCSD_W<10> {
        SEND_AUTO_STOP_CCSD_W::new(self)
    }
    #[doc = "Bit 11 - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ceata_device_interrupt_status(&mut self) -> CEATA_DEVICE_INTERRUPT_STATUS_W<11> {
        CEATA_DEVICE_INTERRUPT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
