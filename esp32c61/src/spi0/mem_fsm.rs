#[doc = "Register `MEM_FSM` reader"]
pub type R = crate::R<MEM_FSM_SPEC>;
#[doc = "Register `MEM_FSM` writer"]
pub type W = crate::W<MEM_FSM_SPEC>;
#[doc = "Field `MEM_LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type MEM_LOCK_DELAY_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type MEM_LOCK_DELAY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MEM_FLASH_LOCK_EN` reader - The lock enable for FLASH to lock spi0 trans req.1: Enable. 0: Disable."]
pub type MEM_FLASH_LOCK_EN_R = crate::BitReader;
#[doc = "Field `MEM_FLASH_LOCK_EN` writer - The lock enable for FLASH to lock spi0 trans req.1: Enable. 0: Disable."]
pub type MEM_FLASH_LOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SRAM_LOCK_EN` reader - The lock enable for external RAM to lock spi0 trans req.1: Enable. 0: Disable."]
pub type MEM_SRAM_LOCK_EN_R = crate::BitReader;
#[doc = "Field `MEM_SRAM_LOCK_EN` writer - The lock enable for external RAM to lock spi0 trans req.1: Enable. 0: Disable."]
pub type MEM_SRAM_LOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 7:18 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn mem_lock_delay_time(&self) -> MEM_LOCK_DELAY_TIME_R {
        MEM_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19 - The lock enable for FLASH to lock spi0 trans req.1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn mem_flash_lock_en(&self) -> MEM_FLASH_LOCK_EN_R {
        MEM_FLASH_LOCK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The lock enable for external RAM to lock spi0 trans req.1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn mem_sram_lock_en(&self) -> MEM_SRAM_LOCK_EN_R {
        MEM_SRAM_LOCK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_FSM")
            .field("mem_lock_delay_time", &self.mem_lock_delay_time())
            .field("mem_flash_lock_en", &self.mem_flash_lock_en())
            .field("mem_sram_lock_en", &self.mem_sram_lock_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:18 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn mem_lock_delay_time(&mut self) -> MEM_LOCK_DELAY_TIME_W<MEM_FSM_SPEC> {
        MEM_LOCK_DELAY_TIME_W::new(self, 7)
    }
    #[doc = "Bit 19 - The lock enable for FLASH to lock spi0 trans req.1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn mem_flash_lock_en(&mut self) -> MEM_FLASH_LOCK_EN_W<MEM_FSM_SPEC> {
        MEM_FLASH_LOCK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - The lock enable for external RAM to lock spi0 trans req.1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn mem_sram_lock_en(&mut self) -> MEM_SRAM_LOCK_EN_W<MEM_FSM_SPEC> {
        MEM_SRAM_LOCK_EN_W::new(self, 20)
    }
}
#[doc = "SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_fsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_fsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_FSM_SPEC;
impl crate::RegisterSpec for MEM_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_fsm::R`](R) reader structure"]
impl crate::Readable for MEM_FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_fsm::W`](W) writer structure"]
impl crate::Writable for MEM_FSM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_FSM to value 0x0200"]
impl crate::Resettable for MEM_FSM_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
