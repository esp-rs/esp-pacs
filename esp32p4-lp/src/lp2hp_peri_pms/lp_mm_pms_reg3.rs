#[doc = "Register `LP_MM_PMS_REG3` reader"]
pub type R = crate::R<LP_MM_PMS_REG3_SPEC>;
#[doc = "Register `LP_MM_PMS_REG3` writer"]
pub type W = crate::W<LP_MM_PMS_REG3_SPEC>;
#[doc = "Field `REG_LP_MM_HP_GPIO_ALLOW` reader - NA"]
pub type REG_LP_MM_HP_GPIO_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_HP_GPIO_ALLOW` writer - NA"]
pub type REG_LP_MM_HP_GPIO_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_HP_IOMUX_ALLOW` reader - NA"]
pub type REG_LP_MM_HP_IOMUX_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_HP_IOMUX_ALLOW` writer - NA"]
pub type REG_LP_MM_HP_IOMUX_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_HP_SYSTIMER_ALLOW` reader - NA"]
pub type REG_LP_MM_HP_SYSTIMER_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_HP_SYSTIMER_ALLOW` writer - NA"]
pub type REG_LP_MM_HP_SYSTIMER_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_HP_SYS_REG_ALLOW` reader - NA"]
pub type REG_LP_MM_HP_SYS_REG_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_HP_SYS_REG_ALLOW` writer - NA"]
pub type REG_LP_MM_HP_SYS_REG_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_HP_CLKRST_ALLOW` reader - NA"]
pub type REG_LP_MM_HP_CLKRST_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_HP_CLKRST_ALLOW` writer - NA"]
pub type REG_LP_MM_HP_CLKRST_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_gpio_allow(&self) -> REG_LP_MM_HP_GPIO_ALLOW_R {
        REG_LP_MM_HP_GPIO_ALLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_iomux_allow(&self) -> REG_LP_MM_HP_IOMUX_ALLOW_R {
        REG_LP_MM_HP_IOMUX_ALLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_systimer_allow(&self) -> REG_LP_MM_HP_SYSTIMER_ALLOW_R {
        REG_LP_MM_HP_SYSTIMER_ALLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_sys_reg_allow(&self) -> REG_LP_MM_HP_SYS_REG_ALLOW_R {
        REG_LP_MM_HP_SYS_REG_ALLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_clkrst_allow(&self) -> REG_LP_MM_HP_CLKRST_ALLOW_R {
        REG_LP_MM_HP_CLKRST_ALLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_MM_PMS_REG3")
            .field("reg_lp_mm_hp_gpio_allow", &self.reg_lp_mm_hp_gpio_allow())
            .field("reg_lp_mm_hp_iomux_allow", &self.reg_lp_mm_hp_iomux_allow())
            .field(
                "reg_lp_mm_hp_systimer_allow",
                &self.reg_lp_mm_hp_systimer_allow(),
            )
            .field(
                "reg_lp_mm_hp_sys_reg_allow",
                &self.reg_lp_mm_hp_sys_reg_allow(),
            )
            .field(
                "reg_lp_mm_hp_clkrst_allow",
                &self.reg_lp_mm_hp_clkrst_allow(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_gpio_allow(
        &mut self,
    ) -> REG_LP_MM_HP_GPIO_ALLOW_W<'_, LP_MM_PMS_REG3_SPEC> {
        REG_LP_MM_HP_GPIO_ALLOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_iomux_allow(
        &mut self,
    ) -> REG_LP_MM_HP_IOMUX_ALLOW_W<'_, LP_MM_PMS_REG3_SPEC> {
        REG_LP_MM_HP_IOMUX_ALLOW_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_systimer_allow(
        &mut self,
    ) -> REG_LP_MM_HP_SYSTIMER_ALLOW_W<'_, LP_MM_PMS_REG3_SPEC> {
        REG_LP_MM_HP_SYSTIMER_ALLOW_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_sys_reg_allow(
        &mut self,
    ) -> REG_LP_MM_HP_SYS_REG_ALLOW_W<'_, LP_MM_PMS_REG3_SPEC> {
        REG_LP_MM_HP_SYS_REG_ALLOW_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_hp_clkrst_allow(
        &mut self,
    ) -> REG_LP_MM_HP_CLKRST_ALLOW_W<'_, LP_MM_PMS_REG3_SPEC> {
        REG_LP_MM_HP_CLKRST_ALLOW_W::new(self, 4)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_MM_PMS_REG3_SPEC;
impl crate::RegisterSpec for LP_MM_PMS_REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_mm_pms_reg3::R`](R) reader structure"]
impl crate::Readable for LP_MM_PMS_REG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_mm_pms_reg3::W`](W) writer structure"]
impl crate::Writable for LP_MM_PMS_REG3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_MM_PMS_REG3 to value 0"]
impl crate::Resettable for LP_MM_PMS_REG3_SPEC {}
