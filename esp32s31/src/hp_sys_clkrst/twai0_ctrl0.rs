#[doc = "Register `TWAI0_CTRL0` reader"]
pub type R = crate::R<TWAI0_CTRL0_SPEC>;
#[doc = "Register `TWAI0_CTRL0` writer"]
pub type W = crate::W<TWAI0_CTRL0_SPEC>;
#[doc = "Field `REG_TWAI0_APB_CLK_EN` reader - need_des"]
pub type REG_TWAI0_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TWAI0_APB_CLK_EN` writer - need_des"]
pub type REG_TWAI0_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TWAI0_CLK_SRC_SEL` reader - need_des"]
pub type REG_TWAI0_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_TWAI0_CLK_SRC_SEL` writer - need_des"]
pub type REG_TWAI0_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TWAI0_CLK_EN` reader - need_des"]
pub type REG_TWAI0_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TWAI0_CLK_EN` writer - need_des"]
pub type REG_TWAI0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TWAI0_RST_EN` reader - need_des"]
pub type REG_TWAI0_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_TWAI0_RST_EN` writer - need_des"]
pub type REG_TWAI0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TWAI0_FORCE_NORST` reader - need_des"]
pub type REG_TWAI0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `REG_TWAI0_FORCE_NORST` writer - need_des"]
pub type REG_TWAI0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_apb_clk_en(&self) -> REG_TWAI0_APB_CLK_EN_R {
        REG_TWAI0_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_clk_src_sel(&self) -> REG_TWAI0_CLK_SRC_SEL_R {
        REG_TWAI0_CLK_SRC_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_clk_en(&self) -> REG_TWAI0_CLK_EN_R {
        REG_TWAI0_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_rst_en(&self) -> REG_TWAI0_RST_EN_R {
        REG_TWAI0_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_force_norst(&self) -> REG_TWAI0_FORCE_NORST_R {
        REG_TWAI0_FORCE_NORST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0_CTRL0")
            .field("reg_twai0_apb_clk_en", &self.reg_twai0_apb_clk_en())
            .field("reg_twai0_clk_src_sel", &self.reg_twai0_clk_src_sel())
            .field("reg_twai0_clk_en", &self.reg_twai0_clk_en())
            .field("reg_twai0_rst_en", &self.reg_twai0_rst_en())
            .field("reg_twai0_force_norst", &self.reg_twai0_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_apb_clk_en(&mut self) -> REG_TWAI0_APB_CLK_EN_W<'_, TWAI0_CTRL0_SPEC> {
        REG_TWAI0_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_clk_src_sel(&mut self) -> REG_TWAI0_CLK_SRC_SEL_W<'_, TWAI0_CTRL0_SPEC> {
        REG_TWAI0_CLK_SRC_SEL_W::new(self, 1)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_clk_en(&mut self) -> REG_TWAI0_CLK_EN_W<'_, TWAI0_CTRL0_SPEC> {
        REG_TWAI0_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_rst_en(&mut self) -> REG_TWAI0_RST_EN_W<'_, TWAI0_CTRL0_SPEC> {
        REG_TWAI0_RST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn reg_twai0_force_norst(&mut self) -> REG_TWAI0_FORCE_NORST_W<'_, TWAI0_CTRL0_SPEC> {
        REG_TWAI0_FORCE_NORST_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`twai0_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai0_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAI0_CTRL0_SPEC;
impl crate::RegisterSpec for TWAI0_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twai0_ctrl0::R`](R) reader structure"]
impl crate::Readable for TWAI0_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twai0_ctrl0::W`](W) writer structure"]
impl crate::Writable for TWAI0_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWAI0_CTRL0 to value 0"]
impl crate::Resettable for TWAI0_CTRL0_SPEC {}
