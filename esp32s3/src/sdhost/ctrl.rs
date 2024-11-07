#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CONTROLLER_RESET` reader - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
pub type CONTROLLER_RESET_R = crate::BitReader;
#[doc = "Field `CONTROLLER_RESET` writer - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
pub type CONTROLLER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_RESET` reader - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
pub type FIFO_RESET_R = crate::BitReader;
#[doc = "Field `FIFO_RESET` writer - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
pub type FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RESET` reader - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
pub type DMA_RESET_R = crate::BitReader;
#[doc = "Field `DMA_RESET` writer - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
pub type DMA_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
pub type INT_ENABLE_R = crate::BitReader;
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
pub type INT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_WAIT` reader - For sending read-wait to SDIO cards."]
pub type READ_WAIT_R = crate::BitReader;
#[doc = "Field `READ_WAIT` writer - For sending read-wait to SDIO cards."]
pub type READ_WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_IRQ_RESPONSE` reader - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
pub type SEND_IRQ_RESPONSE_R = crate::BitReader;
#[doc = "Field `SEND_IRQ_RESPONSE` writer - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
pub type SEND_IRQ_RESPONSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_READ_DATA` reader - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
pub type ABORT_READ_DATA_R = crate::BitReader;
#[doc = "Field `ABORT_READ_DATA` writer - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
pub type ABORT_READ_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_CCSD` reader - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
pub type SEND_CCSD_R = crate::BitReader;
#[doc = "Field `SEND_CCSD` writer - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
pub type SEND_CCSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
pub type SEND_AUTO_STOP_CCSD_R = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
pub type SEND_AUTO_STOP_CCSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_R = crate::BitReader;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("controller_reset", &self.controller_reset())
            .field("fifo_reset", &self.fifo_reset())
            .field("dma_reset", &self.dma_reset())
            .field("int_enable", &self.int_enable())
            .field("read_wait", &self.read_wait())
            .field("send_irq_response", &self.send_irq_response())
            .field("abort_read_data", &self.abort_read_data())
            .field("send_ccsd", &self.send_ccsd())
            .field("send_auto_stop_ccsd", &self.send_auto_stop_ccsd())
            .field(
                "ceata_device_interrupt_status",
                &self.ceata_device_interrupt_status(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - To reset controller, firmware should set this bit. This bit is auto-cleared after two AHB and two sdhost_cclk_in clock cycles."]
    #[inline(always)]
    pub fn controller_reset(&mut self) -> CONTROLLER_RESET_W<CTRL_SPEC> {
        CONTROLLER_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation. Note: FIFO pointers will be out of reset after 2 cycles of system clocks in addition to synchronization delay (2 cycles of card clock), after the fifo_reset is cleared."]
    #[inline(always)]
    pub fn fifo_reset(&mut self) -> FIFO_RESET_W<CTRL_SPEC> {
        FIFO_RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline(always)]
    pub fn dma_reset(&mut self) -> DMA_RESET_W<CTRL_SPEC> {
        DMA_RESET_W::new(self, 2)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit. 0: Disable; 1: Enable."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> INT_ENABLE_W<CTRL_SPEC> {
        INT_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 6 - For sending read-wait to SDIO cards."]
    #[inline(always)]
    pub fn read_wait(&mut self) -> READ_WAIT_W<CTRL_SPEC> {
        READ_WAIT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40 and waits for interrupt response from MMC card(s). In the meantime, if host wants SD/MMC to exit waiting for interrupt state, it can set this bit, at which time SD/MMC command state-machine sends CMD40 response on bus and returns to idle state."]
    #[inline(always)]
    pub fn send_irq_response(&mut self) -> SEND_IRQ_RESPONSE_W<CTRL_SPEC> {
        SEND_IRQ_RESPONSE_W::new(self, 7)
    }
    #[doc = "Bit 8 - After a suspend-command is issued during a read-operation, software polls the card to find when the suspend-event occurred. Once the suspend-event has occurred, software sets the bit which will reset the data state machine that is waiting for the next block of data. This bit is automatically cleared once the data state machine is reset to idle."]
    #[inline(always)]
    pub fn abort_read_data(&mut self) -> ABORT_READ_DATA_W<CTRL_SPEC> {
        ABORT_READ_DATA_W::new(self, 8)
    }
    #[doc = "Bit 9 - When set, SD/MMC sends CCSD to the CE-ATA device. Software sets this bit only if the current command is expecting CCS (that is, RW_BLK), and if interrupts are enabled for the CE-ATA device. Once the CCSD pattern is sent to the device, SD/MMC automatically clears the SDHOST_SEND_CCSD bit. It also sets the Command Done (CD) bit in the SDHOST_RINTSTS_REG register, and generates an interrupt for the host, in case the Command Done interrupt is not masked. NOTE: Once the SDHOST_SEND_CCSD bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, within the boundary conditions the CCSD may be sent to the CE-ATA device, even if the device has signalled CCS."]
    #[inline(always)]
    pub fn send_ccsd(&mut self) -> SEND_CCSD_W<CTRL_SPEC> {
        SEND_CCSD_W::new(self, 9)
    }
    #[doc = "Bit 10 - Always Set SDHOST_SEND_AUTO_STOP_CCSD and SDHOST_SEND_CCSD bits together; SDHOST_SEND_AUTO_STOP_CCSD should not be set independently of send_ccsd. When set, SD/MMC automatically sends an internally-generated STOP command (CMD12) to the CE-ATA device. After sending this internally-generated STOP command, the Auto Command Done (ACD) bit in SDHOST_RINTSTS_REG is set and an interrupt is generated for the host, in case the ACD interrupt is not masked. After sending the Command Completion Signal Disable (CCSD), SD/MMC automatically clears the SDHOST_SEND_AUTO_STOP_CCSD bit."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&mut self) -> SEND_AUTO_STOP_CCSD_W<CTRL_SPEC> {
        SEND_AUTO_STOP_CCSD_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software should appropriately write to this bit after the power-on reset or any other reset to the CE-ATA device. After reset, the CE-ATA device's interrupt is usually disabled (nIEN = 1). If the host enables the CE-ATA device's interrupt, then software should set this bit."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&mut self) -> CEATA_DEVICE_INTERRUPT_STATUS_W<CTRL_SPEC> {
        CEATA_DEVICE_INTERRUPT_STATUS_W::new(self, 11)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
