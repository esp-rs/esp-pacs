#[doc = "Register `APB_CONF` reader"]
pub type R = crate::R<APB_CONF_SPEC>;
#[doc = "Register `APB_CONF` writer"]
pub type W = crate::W<APB_CONF_SPEC>;
#[doc = "Field `APB_FIFO_MASK` reader - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub type APB_FIFO_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit to enable the clock for RMT memory."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit to enable the clock for RMT memory."]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to power down RMT memory."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to power down RMT memory."]
pub type MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PU` reader - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub type MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to power down RMT memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CONF")
            .field("apb_fifo_mask", &self.apb_fifo_mask())
            .field("mem_tx_wrap_en", &self.mem_tx_wrap_en())
            .field("mem_clk_force_on", &self.mem_clk_force_on())
            .field("mem_force_pd", &self.mem_force_pd())
            .field("mem_force_pu", &self.mem_force_pu())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<APB_CONF_SPEC> {
        APB_FIFO_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<APB_CONF_SPEC> {
        MEM_TX_WRAP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<APB_CONF_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to power down RMT memory."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<APB_CONF_SPEC> {
        MEM_FORCE_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<APB_CONF_SPEC> {
        MEM_FORCE_PU_W::new(self, 4)
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<APB_CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "RMT apb configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_CONF_SPEC;
impl crate::RegisterSpec for APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_conf::R`](R) reader structure"]
impl crate::Readable for APB_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_conf::W`](W) writer structure"]
impl crate::Writable for APB_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_CONF to value 0x04"]
impl crate::Resettable for APB_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
