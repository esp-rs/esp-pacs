#[doc = "Register `HP_ALIVE_SYSREG_CTRL0` reader"]
pub type R = crate::R<HP_ALIVE_SYSREG_CTRL0_SPEC>;
#[doc = "Register `HP_ALIVE_SYSREG_CTRL0` writer"]
pub type W = crate::W<HP_ALIVE_SYSREG_CTRL0_SPEC>;
#[doc = "Field `HP_ALIVE_SYSREG_APB_CLK_EN` reader - hp alive_sysreg clk en"]
pub type HP_ALIVE_SYSREG_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_ALIVE_SYSREG_APB_CLK_EN` writer - hp alive_sysreg clk en"]
pub type HP_ALIVE_SYSREG_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp alive_sysreg clk en"]
    #[inline(always)]
    pub fn hp_alive_sysreg_apb_clk_en(&self) -> HP_ALIVE_SYSREG_APB_CLK_EN_R {
        HP_ALIVE_SYSREG_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ALIVE_SYSREG_CTRL0")
            .field(
                "hp_alive_sysreg_apb_clk_en",
                &self.hp_alive_sysreg_apb_clk_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - hp alive_sysreg clk en"]
    #[inline(always)]
    pub fn hp_alive_sysreg_apb_clk_en(
        &mut self,
    ) -> HP_ALIVE_SYSREG_APB_CLK_EN_W<'_, HP_ALIVE_SYSREG_CTRL0_SPEC> {
        HP_ALIVE_SYSREG_APB_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_alive_sysreg_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_alive_sysreg_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ALIVE_SYSREG_CTRL0_SPEC;
impl crate::RegisterSpec for HP_ALIVE_SYSREG_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_alive_sysreg_ctrl0::R`](R) reader structure"]
impl crate::Readable for HP_ALIVE_SYSREG_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_alive_sysreg_ctrl0::W`](W) writer structure"]
impl crate::Writable for HP_ALIVE_SYSREG_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ALIVE_SYSREG_CTRL0 to value 0x01"]
impl crate::Resettable for HP_ALIVE_SYSREG_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
