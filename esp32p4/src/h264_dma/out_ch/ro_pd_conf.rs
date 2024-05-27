///Register `RO_PD_CONF` reader
pub type R = crate::R<RO_PD_CONF_SPEC>;
///Register `RO_PD_CONF` writer
pub type W = crate::W<RO_PD_CONF_SPEC>;
///Field `OUT_RO_RAM_FORCE_PD` reader - dma reorder ram power down
pub type OUT_RO_RAM_FORCE_PD_R = crate::BitReader;
///Field `OUT_RO_RAM_FORCE_PD` writer - dma reorder ram power down
pub type OUT_RO_RAM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_RO_RAM_FORCE_PU` reader - dma reorder ram power up
pub type OUT_RO_RAM_FORCE_PU_R = crate::BitReader;
///Field `OUT_RO_RAM_FORCE_PU` writer - dma reorder ram power up
pub type OUT_RO_RAM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_RO_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
pub type OUT_RO_RAM_CLK_FO_R = crate::BitReader;
///Field `OUT_RO_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
pub type OUT_RO_RAM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - dma reorder ram power down
    #[inline(always)]
    pub fn out_ro_ram_force_pd(&self) -> OUT_RO_RAM_FORCE_PD_R {
        OUT_RO_RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - dma reorder ram power up
    #[inline(always)]
    pub fn out_ro_ram_force_pu(&self) -> OUT_RO_RAM_FORCE_PU_R {
        OUT_RO_RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
    #[inline(always)]
    pub fn out_ro_ram_clk_fo(&self) -> OUT_RO_RAM_CLK_FO_R {
        OUT_RO_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RO_PD_CONF")
            .field("out_ro_ram_force_pd", &self.out_ro_ram_force_pd())
            .field("out_ro_ram_force_pu", &self.out_ro_ram_force_pu())
            .field("out_ro_ram_clk_fo", &self.out_ro_ram_clk_fo())
            .finish()
    }
}
impl W {
    ///Bit 4 - dma reorder ram power down
    #[inline(always)]
    #[must_use]
    pub fn out_ro_ram_force_pd(&mut self) -> OUT_RO_RAM_FORCE_PD_W<RO_PD_CONF_SPEC> {
        OUT_RO_RAM_FORCE_PD_W::new(self, 4)
    }
    ///Bit 5 - dma reorder ram power up
    #[inline(always)]
    #[must_use]
    pub fn out_ro_ram_force_pu(&mut self) -> OUT_RO_RAM_FORCE_PU_W<RO_PD_CONF_SPEC> {
        OUT_RO_RAM_FORCE_PU_W::new(self, 5)
    }
    ///Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
    #[inline(always)]
    #[must_use]
    pub fn out_ro_ram_clk_fo(&mut self) -> OUT_RO_RAM_CLK_FO_W<RO_PD_CONF_SPEC> {
        OUT_RO_RAM_CLK_FO_W::new(self, 6)
    }
}
/**TX CHx reorder power config register. Available on CH0

You can [`read`](crate::generic::Reg::read) this register and get [`ro_pd_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ro_pd_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RO_PD_CONF_SPEC;
impl crate::RegisterSpec for RO_PD_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ro_pd_conf::R`](R) reader structure
impl crate::Readable for RO_PD_CONF_SPEC {}
///`write(|w| ..)` method takes [`ro_pd_conf::W`](W) writer structure
impl crate::Writable for RO_PD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RO_PD_CONF to value 0x20
impl crate::Resettable for RO_PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
