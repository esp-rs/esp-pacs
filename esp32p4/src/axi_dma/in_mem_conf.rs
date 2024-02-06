#[doc = "Register `IN_MEM_CONF` reader"]
pub type R = crate::R<IN_MEM_CONF_SPEC>;
#[doc = "Register `IN_MEM_CONF` writer"]
pub type W = crate::W<IN_MEM_CONF_SPEC>;
#[doc = "Field `IN_MEM_CLK_FORCE_EN` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
pub type IN_MEM_CLK_FORCE_EN_R = crate::BitReader;
#[doc = "Field `IN_MEM_CLK_FORCE_EN` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
pub type IN_MEM_CLK_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MEM_FORCE_PU` reader - Force power up ram"]
pub type IN_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `IN_MEM_FORCE_PU` writer - Force power up ram"]
pub type IN_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_MEM_FORCE_PD` reader - Force power down ram"]
pub type IN_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `IN_MEM_FORCE_PD` writer - Force power down ram"]
pub type IN_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_CLK_FORCE_EN` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
pub type OUT_MEM_CLK_FORCE_EN_R = crate::BitReader;
#[doc = "Field `OUT_MEM_CLK_FORCE_EN` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
pub type OUT_MEM_CLK_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_FORCE_PU` reader - Force power up ram"]
pub type OUT_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `OUT_MEM_FORCE_PU` writer - Force power up ram"]
pub type OUT_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_FORCE_PD` reader - Force power down ram"]
pub type OUT_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `OUT_MEM_FORCE_PD` writer - Force power down ram"]
pub type OUT_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
    #[inline(always)]
    pub fn in_mem_clk_force_en(&self) -> IN_MEM_CLK_FORCE_EN_R {
        IN_MEM_CLK_FORCE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force power up ram"]
    #[inline(always)]
    pub fn in_mem_force_pu(&self) -> IN_MEM_FORCE_PU_R {
        IN_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force power down ram"]
    #[inline(always)]
    pub fn in_mem_force_pd(&self) -> IN_MEM_FORCE_PD_R {
        IN_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
    #[inline(always)]
    pub fn out_mem_clk_force_en(&self) -> OUT_MEM_CLK_FORCE_EN_R {
        OUT_MEM_CLK_FORCE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force power up ram"]
    #[inline(always)]
    pub fn out_mem_force_pu(&self) -> OUT_MEM_FORCE_PU_R {
        OUT_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force power down ram"]
    #[inline(always)]
    pub fn out_mem_force_pd(&self) -> OUT_MEM_FORCE_PD_R {
        OUT_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_MEM_CONF")
            .field(
                "in_mem_clk_force_en",
                &format_args!("{}", self.in_mem_clk_force_en().bit()),
            )
            .field(
                "in_mem_force_pu",
                &format_args!("{}", self.in_mem_force_pu().bit()),
            )
            .field(
                "in_mem_force_pd",
                &format_args!("{}", self.in_mem_force_pd().bit()),
            )
            .field(
                "out_mem_clk_force_en",
                &format_args!("{}", self.out_mem_clk_force_en().bit()),
            )
            .field(
                "out_mem_force_pu",
                &format_args!("{}", self.out_mem_force_pu().bit()),
            )
            .field(
                "out_mem_force_pd",
                &format_args!("{}", self.out_mem_force_pd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
    #[inline(always)]
    #[must_use]
    pub fn in_mem_clk_force_en(&mut self) -> IN_MEM_CLK_FORCE_EN_W<IN_MEM_CONF_SPEC> {
        IN_MEM_CLK_FORCE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force power up ram"]
    #[inline(always)]
    #[must_use]
    pub fn in_mem_force_pu(&mut self) -> IN_MEM_FORCE_PU_W<IN_MEM_CONF_SPEC> {
        IN_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force power down ram"]
    #[inline(always)]
    #[must_use]
    pub fn in_mem_force_pd(&mut self) -> IN_MEM_FORCE_PD_W<IN_MEM_CONF_SPEC> {
        IN_MEM_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in AXI_DMA. 0: A gate-clock will be used when accessing the RAM in AXI_DMA."]
    #[inline(always)]
    #[must_use]
    pub fn out_mem_clk_force_en(&mut self) -> OUT_MEM_CLK_FORCE_EN_W<IN_MEM_CONF_SPEC> {
        OUT_MEM_CLK_FORCE_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Force power up ram"]
    #[inline(always)]
    #[must_use]
    pub fn out_mem_force_pu(&mut self) -> OUT_MEM_FORCE_PU_W<IN_MEM_CONF_SPEC> {
        OUT_MEM_FORCE_PU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force power down ram"]
    #[inline(always)]
    #[must_use]
    pub fn out_mem_force_pd(&mut self) -> OUT_MEM_FORCE_PD_W<IN_MEM_CONF_SPEC> {
        OUT_MEM_FORCE_PD_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mem power configure register of Rx channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_mem_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_mem_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_MEM_CONF_SPEC;
impl crate::RegisterSpec for IN_MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_mem_conf::R`](R) reader structure"]
impl crate::Readable for IN_MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_mem_conf::W`](W) writer structure"]
impl crate::Writable for IN_MEM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_MEM_CONF to value 0"]
impl crate::Resettable for IN_MEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
