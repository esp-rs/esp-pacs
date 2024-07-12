#[doc = "Register `PD_CONF` reader"]
pub type R = crate::R<PD_CONF_SPEC>;
#[doc = "Register `PD_CONF` writer"]
pub type W = crate::W<PD_CONF_SPEC>;
#[doc = "Field `DMA_RAM_FORCE_PD` reader - Set this bit to force power down DMA internal memory."]
pub type DMA_RAM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DMA_RAM_FORCE_PD` writer - Set this bit to force power down DMA internal memory."]
pub type DMA_RAM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RAM_FORCE_PU` reader - Set this bit to force power up DMA internal memory"]
pub type DMA_RAM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DMA_RAM_FORCE_PU` writer - Set this bit to force power up DMA internal memory"]
pub type DMA_RAM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type DMA_RAM_CLK_FO_R = crate::BitReader;
#[doc = "Field `DMA_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type DMA_RAM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set this bit to force power down DMA internal memory."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&self) -> DMA_RAM_FORCE_PD_R {
        DMA_RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to force power up DMA internal memory"]
    #[inline(always)]
    pub fn dma_ram_force_pu(&self) -> DMA_RAM_FORCE_PU_R {
        DMA_RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&self) -> DMA_RAM_CLK_FO_R {
        DMA_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_CONF")
            .field("dma_ram_force_pd", &self.dma_ram_force_pd())
            .field("dma_ram_force_pu", &self.dma_ram_force_pu())
            .field("dma_ram_clk_fo", &self.dma_ram_clk_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit to force power down DMA internal memory."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_force_pd(&mut self) -> DMA_RAM_FORCE_PD_W<PD_CONF_SPEC> {
        DMA_RAM_FORCE_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to force power up DMA internal memory"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_force_pu(&mut self) -> DMA_RAM_FORCE_PU_W<PD_CONF_SPEC> {
        DMA_RAM_FORCE_PU_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_clk_fo(&mut self) -> DMA_RAM_CLK_FO_W<PD_CONF_SPEC> {
        DMA_RAM_CLK_FO_W::new(self, 6)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PD_CONF to value 0x20"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
