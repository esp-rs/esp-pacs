#[doc = "Register `ETM_CTRL0` reader"]
pub type R = crate::R<ETM_CTRL0_SPEC>;
#[doc = "Register `ETM_CTRL0` writer"]
pub type W = crate::W<ETM_CTRL0_SPEC>;
#[doc = "Field `REG_ETM_APB_CLK_EN` reader - need_des"]
pub type REG_ETM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_ETM_APB_CLK_EN` writer - need_des"]
pub type REG_ETM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ETM_RST_EN` reader - need_des"]
pub type REG_ETM_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_ETM_RST_EN` writer - need_des"]
pub type REG_ETM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ETM_FORCE_NORST` reader - need_des"]
pub type REG_ETM_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `REG_ETM_FORCE_NORST` writer - need_des"]
pub type REG_ETM_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SOC_ETM_CLK_SEL` reader - need_des"]
pub type REG_SOC_ETM_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `REG_SOC_ETM_CLK_SEL` writer - need_des"]
pub type REG_SOC_ETM_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SOC_ETM_CLK_EN` reader - need_des"]
pub type REG_SOC_ETM_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_SOC_ETM_CLK_EN` writer - need_des"]
pub type REG_SOC_ETM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_etm_apb_clk_en(&self) -> REG_ETM_APB_CLK_EN_R {
        REG_ETM_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_etm_rst_en(&self) -> REG_ETM_RST_EN_R {
        REG_ETM_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_etm_force_norst(&self) -> REG_ETM_FORCE_NORST_R {
        REG_ETM_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn reg_soc_etm_clk_sel(&self) -> REG_SOC_ETM_CLK_SEL_R {
        REG_SOC_ETM_CLK_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn reg_soc_etm_clk_en(&self) -> REG_SOC_ETM_CLK_EN_R {
        REG_SOC_ETM_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CTRL0")
            .field("reg_etm_apb_clk_en", &self.reg_etm_apb_clk_en())
            .field("reg_etm_rst_en", &self.reg_etm_rst_en())
            .field("reg_etm_force_norst", &self.reg_etm_force_norst())
            .field("reg_soc_etm_clk_sel", &self.reg_soc_etm_clk_sel())
            .field("reg_soc_etm_clk_en", &self.reg_soc_etm_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_etm_apb_clk_en(&mut self) -> REG_ETM_APB_CLK_EN_W<'_, ETM_CTRL0_SPEC> {
        REG_ETM_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_etm_rst_en(&mut self) -> REG_ETM_RST_EN_W<'_, ETM_CTRL0_SPEC> {
        REG_ETM_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_etm_force_norst(&mut self) -> REG_ETM_FORCE_NORST_W<'_, ETM_CTRL0_SPEC> {
        REG_ETM_FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn reg_soc_etm_clk_sel(&mut self) -> REG_SOC_ETM_CLK_SEL_W<'_, ETM_CTRL0_SPEC> {
        REG_SOC_ETM_CLK_SEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn reg_soc_etm_clk_en(&mut self) -> REG_SOC_ETM_CLK_EN_W<'_, ETM_CTRL0_SPEC> {
        REG_SOC_ETM_CLK_EN_W::new(self, 6)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_CTRL0_SPEC;
impl crate::RegisterSpec for ETM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_ctrl0::R`](R) reader structure"]
impl crate::Readable for ETM_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_ctrl0::W`](W) writer structure"]
impl crate::Writable for ETM_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_CTRL0 to value 0x40"]
impl crate::Resettable for ETM_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
