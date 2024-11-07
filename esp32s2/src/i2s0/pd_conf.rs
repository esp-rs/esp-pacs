#[doc = "Register `PD_CONF` reader"]
pub type R = crate::R<PD_CONF_SPEC>;
#[doc = "Register `PD_CONF` writer"]
pub type W = crate::W<PD_CONF_SPEC>;
#[doc = "Field `FIFO_FORCE_PD` reader - Force FIFO power-down."]
pub type FIFO_FORCE_PD_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PD` writer - Force FIFO power-down."]
pub type FIFO_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FORCE_PU` reader - Force FIFO power-up."]
pub type FIFO_FORCE_PU_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PU` writer - Force FIFO power-up."]
pub type FIFO_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC_MEM_FORCE_PD` reader - Force I2S memory power-down."]
pub type PLC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PD` writer - Force I2S memory power-down."]
pub type PLC_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC_MEM_FORCE_PU` reader - Force I2S memory power-up."]
pub type PLC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PU` writer - Force I2S memory power-up."]
pub type PLC_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RAM_FORCE_PD` reader - Force DMA FIFO power-down."]
pub type DMA_RAM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DMA_RAM_FORCE_PD` writer - Force DMA FIFO power-down."]
pub type DMA_RAM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RAM_FORCE_PU` reader - Force DMA FIFO power-up."]
pub type DMA_RAM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DMA_RAM_FORCE_PU` writer - Force DMA FIFO power-up."]
pub type DMA_RAM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RAM_CLK_FO` reader - Set this bit to force on DMA RAM clock."]
pub type DMA_RAM_CLK_FO_R = crate::BitReader;
#[doc = "Field `DMA_RAM_CLK_FO` writer - Set this bit to force on DMA RAM clock."]
pub type DMA_RAM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force FIFO power-down."]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force FIFO power-up."]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force I2S memory power-down."]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force I2S memory power-up."]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force DMA FIFO power-down."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&self) -> DMA_RAM_FORCE_PD_R {
        DMA_RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force DMA FIFO power-up."]
    #[inline(always)]
    pub fn dma_ram_force_pu(&self) -> DMA_RAM_FORCE_PU_R {
        DMA_RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to force on DMA RAM clock."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&self) -> DMA_RAM_CLK_FO_R {
        DMA_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_CONF")
            .field("fifo_force_pd", &self.fifo_force_pd())
            .field("fifo_force_pu", &self.fifo_force_pu())
            .field("plc_mem_force_pd", &self.plc_mem_force_pd())
            .field("plc_mem_force_pu", &self.plc_mem_force_pu())
            .field("dma_ram_force_pd", &self.dma_ram_force_pd())
            .field("dma_ram_force_pu", &self.dma_ram_force_pu())
            .field("dma_ram_clk_fo", &self.dma_ram_clk_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Force FIFO power-down."]
    #[inline(always)]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W<PD_CONF_SPEC> {
        FIFO_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force FIFO power-up."]
    #[inline(always)]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W<PD_CONF_SPEC> {
        FIFO_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force I2S memory power-down."]
    #[inline(always)]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W<PD_CONF_SPEC> {
        PLC_MEM_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force I2S memory power-up."]
    #[inline(always)]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W<PD_CONF_SPEC> {
        PLC_MEM_FORCE_PU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Force DMA FIFO power-down."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&mut self) -> DMA_RAM_FORCE_PD_W<PD_CONF_SPEC> {
        DMA_RAM_FORCE_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force DMA FIFO power-up."]
    #[inline(always)]
    pub fn dma_ram_force_pu(&mut self) -> DMA_RAM_FORCE_PU_W<PD_CONF_SPEC> {
        DMA_RAM_FORCE_PU_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to force on DMA RAM clock."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&mut self) -> DMA_RAM_CLK_FO_W<PD_CONF_SPEC> {
        DMA_RAM_CLK_FO_W::new(self, 6)
    }
}
#[doc = "I2S power-down configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_conf::R`](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_conf::W`](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD_CONF to value 0x2a"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x2a;
}
