#[doc = "Register `MISC_CTRL0` reader"]
pub type R = crate::R<MISC_CTRL0_SPEC>;
#[doc = "Register `MISC_CTRL0` writer"]
pub type W = crate::W<MISC_CTRL0_SPEC>;
#[doc = "Field `REG_MISC_CPU_CLK_EN` reader - need_des"]
pub type REG_MISC_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_MISC_CPU_CLK_EN` writer - need_des"]
pub type REG_MISC_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MISC_SYS_CLK_EN` reader - need_des"]
pub type REG_MISC_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_MISC_SYS_CLK_EN` writer - need_des"]
pub type REG_MISC_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SYSREG_APB_CLK_EN` reader - need_des"]
pub type REG_SYSREG_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_SYSREG_APB_CLK_EN` writer - need_des"]
pub type REG_SYSREG_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_HP_CLKRST_APB_CLK_EN` reader - need_des"]
pub type REG_HP_CLKRST_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_HP_CLKRST_APB_CLK_EN` writer - need_des"]
pub type REG_HP_CLKRST_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_misc_cpu_clk_en(&self) -> REG_MISC_CPU_CLK_EN_R {
        REG_MISC_CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_misc_sys_clk_en(&self) -> REG_MISC_SYS_CLK_EN_R {
        REG_MISC_SYS_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_sysreg_apb_clk_en(&self) -> REG_SYSREG_APB_CLK_EN_R {
        REG_SYSREG_APB_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_hp_clkrst_apb_clk_en(&self) -> REG_HP_CLKRST_APB_CLK_EN_R {
        REG_HP_CLKRST_APB_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CTRL0")
            .field("reg_misc_cpu_clk_en", &self.reg_misc_cpu_clk_en())
            .field("reg_misc_sys_clk_en", &self.reg_misc_sys_clk_en())
            .field("reg_sysreg_apb_clk_en", &self.reg_sysreg_apb_clk_en())
            .field("reg_hp_clkrst_apb_clk_en", &self.reg_hp_clkrst_apb_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_misc_cpu_clk_en(&mut self) -> REG_MISC_CPU_CLK_EN_W<'_, MISC_CTRL0_SPEC> {
        REG_MISC_CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_misc_sys_clk_en(&mut self) -> REG_MISC_SYS_CLK_EN_W<'_, MISC_CTRL0_SPEC> {
        REG_MISC_SYS_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_sysreg_apb_clk_en(&mut self) -> REG_SYSREG_APB_CLK_EN_W<'_, MISC_CTRL0_SPEC> {
        REG_SYSREG_APB_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_hp_clkrst_apb_clk_en(&mut self) -> REG_HP_CLKRST_APB_CLK_EN_W<'_, MISC_CTRL0_SPEC> {
        REG_HP_CLKRST_APB_CLK_EN_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_CTRL0_SPEC;
impl crate::RegisterSpec for MISC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_ctrl0::R`](R) reader structure"]
impl crate::Readable for MISC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc_ctrl0::W`](W) writer structure"]
impl crate::Writable for MISC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CTRL0 to value 0x0f"]
impl crate::Resettable for MISC_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
