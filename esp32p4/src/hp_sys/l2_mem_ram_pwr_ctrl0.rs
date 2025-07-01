#[doc = "Register `L2_MEM_RAM_PWR_CTRL0` reader"]
pub type R = crate::R<L2_MEM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Register `L2_MEM_RAM_PWR_CTRL0` writer"]
pub type W = crate::W<L2_MEM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Field `REG_L2_MEM_CLK_FORCE_ON` reader - l2ram clk_gating force on"]
pub type REG_L2_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_CLK_FORCE_ON` writer - l2ram clk_gating force on"]
pub type REG_L2_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - l2ram clk_gating force on"]
    #[inline(always)]
    pub fn reg_l2_mem_clk_force_on(&self) -> REG_L2_MEM_CLK_FORCE_ON_R {
        REG_L2_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_RAM_PWR_CTRL0")
            .field("reg_l2_mem_clk_force_on", &self.reg_l2_mem_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - l2ram clk_gating force on"]
    #[inline(always)]
    pub fn reg_l2_mem_clk_force_on(
        &mut self,
    ) -> REG_L2_MEM_CLK_FORCE_ON_W<L2_MEM_RAM_PWR_CTRL0_SPEC> {
        REG_L2_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_ram_pwr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_ram_pwr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_RAM_PWR_CTRL0_SPEC;
impl crate::RegisterSpec for L2_MEM_RAM_PWR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_ram_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for L2_MEM_RAM_PWR_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_ram_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for L2_MEM_RAM_PWR_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_RAM_PWR_CTRL0 to value 0"]
impl crate::Resettable for L2_MEM_RAM_PWR_CTRL0_SPEC {}
