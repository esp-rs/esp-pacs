///Register `RO_PD_CONF` reader
pub type R = crate::R<RO_PD_CONF_SPEC>;
///Register `RO_PD_CONF` writer
pub type W = crate::W<RO_PD_CONF_SPEC>;
///Field `IN_RO_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
pub type IN_RO_RAM_CLK_FO_R = crate::BitReader;
///Field `IN_RO_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
pub type IN_RO_RAM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
    #[inline(always)]
    pub fn in_ro_ram_clk_fo(&self) -> IN_RO_RAM_CLK_FO_R {
        IN_RO_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RO_PD_CONF")
            .field("in_ro_ram_clk_fo", &self.in_ro_ram_clk_fo())
            .finish()
    }
}
impl W {
    ///Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA.
    #[inline(always)]
    #[must_use]
    pub fn in_ro_ram_clk_fo(&mut self) -> IN_RO_RAM_CLK_FO_W<RO_PD_CONF_SPEC> {
        IN_RO_RAM_CLK_FO_W::new(self, 6)
    }
}
/**RX CHx reorder power config register. Available on CH0

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
///`reset()` method sets RO_PD_CONF to value 0
impl crate::Resettable for RO_PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
