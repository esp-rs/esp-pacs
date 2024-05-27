///Register `MEM_CLK_CTRL` reader
pub type R = crate::R<MEM_CLK_CTRL_SPEC>;
///Register `MEM_CLK_CTRL` writer
pub type W = crate::W<MEM_CLK_CTRL_SPEC>;
///Field `DSI_BRIDGE_MEM_CLK_FORCE_ON` reader - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
pub type DSI_BRIDGE_MEM_CLK_FORCE_ON_R = crate::BitReader;
///Field `DSI_BRIDGE_MEM_CLK_FORCE_ON` writer - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
pub type DSI_BRIDGE_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSI_MEM_CLK_FORCE_ON` reader - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
pub type DSI_MEM_CLK_FORCE_ON_R = crate::BitReader;
///Field `DSI_MEM_CLK_FORCE_ON` writer - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
pub type DSI_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub fn dsi_bridge_mem_clk_force_on(&self) -> DSI_BRIDGE_MEM_CLK_FORCE_ON_R {
        DSI_BRIDGE_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub fn dsi_mem_clk_force_on(&self) -> DSI_MEM_CLK_FORCE_ON_R {
        DSI_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CLK_CTRL")
            .field(
                "dsi_bridge_mem_clk_force_on",
                &self.dsi_bridge_mem_clk_force_on(),
            )
            .field("dsi_mem_clk_force_on", &self.dsi_mem_clk_force_on())
            .finish()
    }
}
impl W {
    ///Bit 0 - this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    #[inline(always)]
    #[must_use]
    pub fn dsi_bridge_mem_clk_force_on(
        &mut self,
    ) -> DSI_BRIDGE_MEM_CLK_FORCE_ON_W<MEM_CLK_CTRL_SPEC> {
        DSI_BRIDGE_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    ///Bit 1 - this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    #[inline(always)]
    #[must_use]
    pub fn dsi_mem_clk_force_on(&mut self) -> DSI_MEM_CLK_FORCE_ON_W<MEM_CLK_CTRL_SPEC> {
        DSI_MEM_CLK_FORCE_ON_W::new(self, 1)
    }
}
/**dsi_bridge mem force on control register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_CLK_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CLK_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_clk_ctrl::R`](R) reader structure
impl crate::Readable for MEM_CLK_CTRL_SPEC {}
///`write(|w| ..)` method takes [`mem_clk_ctrl::W`](W) writer structure
impl crate::Writable for MEM_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_CLK_CTRL to value 0
impl crate::Resettable for MEM_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
