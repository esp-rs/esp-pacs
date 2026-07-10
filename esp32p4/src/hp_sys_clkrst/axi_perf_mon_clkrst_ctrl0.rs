#[doc = "Register `AXI_PERF_MON_CLKRST_CTRL0` reader"]
pub type R = crate::R<AXI_PERF_MON_CLKRST_CTRL0_SPEC>;
#[doc = "Register `AXI_PERF_MON_CLKRST_CTRL0` writer"]
pub type W = crate::W<AXI_PERF_MON_CLKRST_CTRL0_SPEC>;
#[doc = "Field `AXI_PERF_MON_SYS_CLK_EN` reader - Configures axi_perf_mon clk enable"]
pub type AXI_PERF_MON_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `AXI_PERF_MON_SYS_CLK_EN` writer - Configures axi_perf_mon clk enable"]
pub type AXI_PERF_MON_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_PERF_MON_SYS_RST_EN` reader - Configures axi_perf_mon rst enable"]
pub type AXI_PERF_MON_SYS_RST_EN_R = crate::BitReader;
#[doc = "Field `AXI_PERF_MON_SYS_RST_EN` writer - Configures axi_perf_mon rst enable"]
pub type AXI_PERF_MON_SYS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures axi_perf_mon clk enable"]
    #[inline(always)]
    pub fn axi_perf_mon_sys_clk_en(&self) -> AXI_PERF_MON_SYS_CLK_EN_R {
        AXI_PERF_MON_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures axi_perf_mon rst enable"]
    #[inline(always)]
    pub fn axi_perf_mon_sys_rst_en(&self) -> AXI_PERF_MON_SYS_RST_EN_R {
        AXI_PERF_MON_SYS_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_PERF_MON_CLKRST_CTRL0")
            .field("axi_perf_mon_sys_clk_en", &self.axi_perf_mon_sys_clk_en())
            .field("axi_perf_mon_sys_rst_en", &self.axi_perf_mon_sys_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures axi_perf_mon clk enable"]
    #[inline(always)]
    pub fn axi_perf_mon_sys_clk_en(
        &mut self,
    ) -> AXI_PERF_MON_SYS_CLK_EN_W<'_, AXI_PERF_MON_CLKRST_CTRL0_SPEC> {
        AXI_PERF_MON_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures axi_perf_mon rst enable"]
    #[inline(always)]
    pub fn axi_perf_mon_sys_rst_en(
        &mut self,
    ) -> AXI_PERF_MON_SYS_RST_EN_W<'_, AXI_PERF_MON_CLKRST_CTRL0_SPEC> {
        AXI_PERF_MON_SYS_RST_EN_W::new(self, 1)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_clkrst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_clkrst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERF_MON_CLKRST_CTRL0_SPEC;
impl crate::RegisterSpec for AXI_PERF_MON_CLKRST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_perf_mon_clkrst_ctrl0::R`](R) reader structure"]
impl crate::Readable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_perf_mon_clkrst_ctrl0::W`](W) writer structure"]
impl crate::Writable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_PERF_MON_CLKRST_CTRL0 to value 0"]
impl crate::Resettable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {}
