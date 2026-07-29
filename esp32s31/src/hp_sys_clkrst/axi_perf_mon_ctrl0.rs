#[doc = "Register `AXI_PERF_MON_CTRL0` reader"]
pub type R = crate::R<AXI_PERF_MON_CTRL0_SPEC>;
#[doc = "Register `AXI_PERF_MON_CTRL0` writer"]
pub type W = crate::W<AXI_PERF_MON_CTRL0_SPEC>;
#[doc = "Field `RST_EN` reader - axi_perf_mon rst en"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - axi_perf_mon rst en"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - axi_perf_mon clk en"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - axi_perf_mon clk en"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NORST` reader - need_des"]
pub type FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FORCE_NORST` writer - need_des"]
pub type FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - axi_perf_mon rst en"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf_mon clk en"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&self) -> FORCE_NORST_R {
        FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_PERF_MON_CTRL0")
            .field("rst_en", &self.rst_en())
            .field("clk_en", &self.clk_en())
            .field("force_norst", &self.force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - axi_perf_mon rst en"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, AXI_PERF_MON_CTRL0_SPEC> {
        RST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf_mon clk en"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, AXI_PERF_MON_CTRL0_SPEC> {
        CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, AXI_PERF_MON_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERF_MON_CTRL0_SPEC;
impl crate::RegisterSpec for AXI_PERF_MON_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_perf_mon_ctrl0::R`](R) reader structure"]
impl crate::Readable for AXI_PERF_MON_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_perf_mon_ctrl0::W`](W) writer structure"]
impl crate::Writable for AXI_PERF_MON_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_PERF_MON_CTRL0 to value 0"]
impl crate::Resettable for AXI_PERF_MON_CTRL0_SPEC {}
