#[doc = "Register `UHCI_CTRL0` reader"]
pub type R = crate::R<UHCI_CTRL0_SPEC>;
#[doc = "Register `UHCI_CTRL0` writer"]
pub type W = crate::W<UHCI_CTRL0_SPEC>;
#[doc = "Field `REG_UHCI_SYS_CLK_EN` reader - need_des"]
pub type REG_UHCI_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_UHCI_SYS_CLK_EN` writer - need_des"]
pub type REG_UHCI_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_UHCI_APB_CLK_EN` reader - need_des"]
pub type REG_UHCI_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_UHCI_APB_CLK_EN` writer - need_des"]
pub type REG_UHCI_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_UHCI_RST_EN` reader - need_des"]
pub type REG_UHCI_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_UHCI_RST_EN` writer - need_des"]
pub type REG_UHCI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_UHCI_FORCE_NORST` reader - need_des"]
pub type REG_UHCI_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `REG_UHCI_FORCE_NORST` writer - need_des"]
pub type REG_UHCI_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_sys_clk_en(&self) -> REG_UHCI_SYS_CLK_EN_R {
        REG_UHCI_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_apb_clk_en(&self) -> REG_UHCI_APB_CLK_EN_R {
        REG_UHCI_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_rst_en(&self) -> REG_UHCI_RST_EN_R {
        REG_UHCI_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_force_norst(&self) -> REG_UHCI_FORCE_NORST_R {
        REG_UHCI_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI_CTRL0")
            .field("reg_uhci_sys_clk_en", &self.reg_uhci_sys_clk_en())
            .field("reg_uhci_apb_clk_en", &self.reg_uhci_apb_clk_en())
            .field("reg_uhci_rst_en", &self.reg_uhci_rst_en())
            .field("reg_uhci_force_norst", &self.reg_uhci_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_sys_clk_en(&mut self) -> REG_UHCI_SYS_CLK_EN_W<'_, UHCI_CTRL0_SPEC> {
        REG_UHCI_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_apb_clk_en(&mut self) -> REG_UHCI_APB_CLK_EN_W<'_, UHCI_CTRL0_SPEC> {
        REG_UHCI_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_rst_en(&mut self) -> REG_UHCI_RST_EN_W<'_, UHCI_CTRL0_SPEC> {
        REG_UHCI_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_uhci_force_norst(&mut self) -> REG_UHCI_FORCE_NORST_W<'_, UHCI_CTRL0_SPEC> {
        REG_UHCI_FORCE_NORST_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHCI_CTRL0_SPEC;
impl crate::RegisterSpec for UHCI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci_ctrl0::R`](R) reader structure"]
impl crate::Readable for UHCI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhci_ctrl0::W`](W) writer structure"]
impl crate::Writable for UHCI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI_CTRL0 to value 0"]
impl crate::Resettable for UHCI_CTRL0_SPEC {}
