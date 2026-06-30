#[doc = "Register `BUS_CLOCK_GATE_BYPASS` reader"]
pub type R = crate::R<BUS_CLOCK_GATE_BYPASS_SPEC>;
#[doc = "Register `BUS_CLOCK_GATE_BYPASS` writer"]
pub type W = crate::W<BUS_CLOCK_GATE_BYPASS_SPEC>;
#[doc = "Field `AHB_CLK_GATING_BYPASS` reader - Set 1 to bypass the clock gating for ahb bus"]
pub type AHB_CLK_GATING_BYPASS_R = crate::BitReader;
#[doc = "Field `AHB_CLK_GATING_BYPASS` writer - Set 1 to bypass the clock gating for ahb bus"]
pub type AHB_CLK_GATING_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_CLK_GATING_BYPASS` reader - Set 1 to bypass the clock gating for apb bus"]
pub type APB_CLK_GATING_BYPASS_R = crate::BitReader;
#[doc = "Field `APB_CLK_GATING_BYPASS` writer - Set 1 to bypass the clock gating for apb bus"]
pub type APB_CLK_GATING_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_CLK_GATING_BYPASS` reader - Set 1 to bypass the clock gating for axi bus"]
pub type AXI_CLK_GATING_BYPASS_R = crate::BitReader;
#[doc = "Field `AXI_CLK_GATING_BYPASS` writer - Set 1 to bypass the clock gating for axi bus"]
pub type AXI_CLK_GATING_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_GATING_BYPASS` reader - Set 1 to bypass the clock gating for mem bus"]
pub type MEM_CLK_GATING_BYPASS_R = crate::BitReader;
#[doc = "Field `MEM_CLK_GATING_BYPASS` writer - Set 1 to bypass the clock gating for mem bus"]
pub type MEM_CLK_GATING_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_MTX_CLK_GATING_BYPASS` reader - Set 1 to bypass the clock gating for ahb mtx"]
pub type AHB_MTX_CLK_GATING_BYPASS_R = crate::BitReader;
#[doc = "Field `AHB_MTX_CLK_GATING_BYPASS` writer - Set 1 to bypass the clock gating for ahb mtx"]
pub type AHB_MTX_CLK_GATING_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to bypass the clock gating for ahb bus"]
    #[inline(always)]
    pub fn ahb_clk_gating_bypass(&self) -> AHB_CLK_GATING_BYPASS_R {
        AHB_CLK_GATING_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to bypass the clock gating for apb bus"]
    #[inline(always)]
    pub fn apb_clk_gating_bypass(&self) -> APB_CLK_GATING_BYPASS_R {
        APB_CLK_GATING_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to bypass the clock gating for axi bus"]
    #[inline(always)]
    pub fn axi_clk_gating_bypass(&self) -> AXI_CLK_GATING_BYPASS_R {
        AXI_CLK_GATING_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to bypass the clock gating for mem bus"]
    #[inline(always)]
    pub fn mem_clk_gating_bypass(&self) -> MEM_CLK_GATING_BYPASS_R {
        MEM_CLK_GATING_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to bypass the clock gating for ahb mtx"]
    #[inline(always)]
    pub fn ahb_mtx_clk_gating_bypass(&self) -> AHB_MTX_CLK_GATING_BYPASS_R {
        AHB_MTX_CLK_GATING_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_CLOCK_GATE_BYPASS")
            .field("ahb_clk_gating_bypass", &self.ahb_clk_gating_bypass())
            .field("apb_clk_gating_bypass", &self.apb_clk_gating_bypass())
            .field("axi_clk_gating_bypass", &self.axi_clk_gating_bypass())
            .field("mem_clk_gating_bypass", &self.mem_clk_gating_bypass())
            .field(
                "ahb_mtx_clk_gating_bypass",
                &self.ahb_mtx_clk_gating_bypass(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to bypass the clock gating for ahb bus"]
    #[inline(always)]
    pub fn ahb_clk_gating_bypass(
        &mut self,
    ) -> AHB_CLK_GATING_BYPASS_W<'_, BUS_CLOCK_GATE_BYPASS_SPEC> {
        AHB_CLK_GATING_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to bypass the clock gating for apb bus"]
    #[inline(always)]
    pub fn apb_clk_gating_bypass(
        &mut self,
    ) -> APB_CLK_GATING_BYPASS_W<'_, BUS_CLOCK_GATE_BYPASS_SPEC> {
        APB_CLK_GATING_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to bypass the clock gating for axi bus"]
    #[inline(always)]
    pub fn axi_clk_gating_bypass(
        &mut self,
    ) -> AXI_CLK_GATING_BYPASS_W<'_, BUS_CLOCK_GATE_BYPASS_SPEC> {
        AXI_CLK_GATING_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to bypass the clock gating for mem bus"]
    #[inline(always)]
    pub fn mem_clk_gating_bypass(
        &mut self,
    ) -> MEM_CLK_GATING_BYPASS_W<'_, BUS_CLOCK_GATE_BYPASS_SPEC> {
        MEM_CLK_GATING_BYPASS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 1 to bypass the clock gating for ahb mtx"]
    #[inline(always)]
    pub fn ahb_mtx_clk_gating_bypass(
        &mut self,
    ) -> AHB_MTX_CLK_GATING_BYPASS_W<'_, BUS_CLOCK_GATE_BYPASS_SPEC> {
        AHB_MTX_CLK_GATING_BYPASS_W::new(self, 4)
    }
}
#[doc = "bus clock gating bypass configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clock_gate_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clock_gate_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_CLOCK_GATE_BYPASS_SPEC;
impl crate::RegisterSpec for BUS_CLOCK_GATE_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clock_gate_bypass::R`](R) reader structure"]
impl crate::Readable for BUS_CLOCK_GATE_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_clock_gate_bypass::W`](W) writer structure"]
impl crate::Writable for BUS_CLOCK_GATE_BYPASS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CLOCK_GATE_BYPASS to value 0x1f"]
impl crate::Resettable for BUS_CLOCK_GATE_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
