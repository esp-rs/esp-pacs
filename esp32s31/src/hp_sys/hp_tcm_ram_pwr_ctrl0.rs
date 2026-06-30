#[doc = "Register `HP_TCM_RAM_PWR_CTRL0` reader"]
pub type R = crate::R<HP_TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Register `HP_TCM_RAM_PWR_CTRL0` writer"]
pub type W = crate::W<HP_TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "Field `HP_REG_HP_TCM_CLK_FORCE_ON` reader - hp_tcm clk gatig force on"]
pub type HP_REG_HP_TCM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_REG_HP_TCM_CLK_FORCE_ON` writer - hp_tcm clk gatig force on"]
pub type HP_REG_HP_TCM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_clk_force_on(&self) -> HP_REG_HP_TCM_CLK_FORCE_ON_R {
        HP_REG_HP_TCM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_RAM_PWR_CTRL0")
            .field(
                "hp_reg_hp_tcm_clk_force_on",
                &self.hp_reg_hp_tcm_clk_force_on(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - hp_tcm clk gatig force on"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_clk_force_on(
        &mut self,
    ) -> HP_REG_HP_TCM_CLK_FORCE_ON_W<'_, HP_TCM_RAM_PWR_CTRL0_SPEC> {
        HP_REG_HP_TCM_CLK_FORCE_ON_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_ram_pwr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_ram_pwr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_RAM_PWR_CTRL0_SPEC;
impl crate::RegisterSpec for HP_TCM_RAM_PWR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_ram_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for HP_TCM_RAM_PWR_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_ram_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for HP_TCM_RAM_PWR_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_TCM_RAM_PWR_CTRL0 to value 0"]
impl crate::Resettable for HP_TCM_RAM_PWR_CTRL0_SPEC {}
