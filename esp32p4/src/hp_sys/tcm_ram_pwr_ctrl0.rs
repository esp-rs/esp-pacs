#[doc = "Register `TCM_RAM_PWR_CTRL0` reader"]
pub type R = crate::R<TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Register `TCM_RAM_PWR_CTRL0` writer"]
pub type W = crate::W<TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Field `REG_HP_TCM_CLK_FORCE_ON` reader - hp_tcm clk gatig force on"]
pub type REG_HP_TCM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_HP_TCM_CLK_FORCE_ON` writer - hp_tcm clk gatig force on"]
pub type REG_HP_TCM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn reg_hp_tcm_clk_force_on(&self) -> REG_HP_TCM_CLK_FORCE_ON_R {
        REG_HP_TCM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_RAM_PWR_CTRL0")
            .field("reg_hp_tcm_clk_force_on", &self.reg_hp_tcm_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn reg_hp_tcm_clk_force_on(&mut self) -> REG_HP_TCM_CLK_FORCE_ON_W<TCM_RAM_PWR_CTRL0_SPEC> {
        REG_HP_TCM_CLK_FORCE_ON_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_pwr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_pwr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_RAM_PWR_CTRL0_SPEC;
impl crate::RegisterSpec for TCM_RAM_PWR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_ram_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for TCM_RAM_PWR_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_ram_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for TCM_RAM_PWR_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCM_RAM_PWR_CTRL0 to value 0"]
impl crate::Resettable for TCM_RAM_PWR_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
