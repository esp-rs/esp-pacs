#[doc = "Register `ETM_CTRL0` reader"]
pub type R = crate::R<ETM_CTRL0_SPEC>;
#[doc = "Register `ETM_CTRL0` writer"]
pub type W = crate::W<ETM_CTRL0_SPEC>;
#[doc = "Field `ETM_APB_CLK_EN` reader - need_des"]
pub type ETM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ETM_APB_CLK_EN` writer - need_des"]
pub type ETM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_RST_EN` reader - need_des"]
pub type ETM_RST_EN_R = crate::BitReader;
#[doc = "Field `ETM_RST_EN` writer - need_des"]
pub type ETM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_FORCE_NORST` reader - need_des"]
pub type ETM_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `ETM_FORCE_NORST` writer - need_des"]
pub type ETM_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_ETM_CLK_SEL` reader - need_des"]
pub type SOC_ETM_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SOC_ETM_CLK_SEL` writer - need_des"]
pub type SOC_ETM_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOC_ETM_CLK_EN` reader - need_des"]
pub type SOC_ETM_CLK_EN_R = crate::BitReader;
#[doc = "Field `SOC_ETM_CLK_EN` writer - need_des"]
pub type SOC_ETM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn etm_apb_clk_en(&self) -> ETM_APB_CLK_EN_R {
        ETM_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn etm_rst_en(&self) -> ETM_RST_EN_R {
        ETM_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn etm_force_norst(&self) -> ETM_FORCE_NORST_R {
        ETM_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn soc_etm_clk_sel(&self) -> SOC_ETM_CLK_SEL_R {
        SOC_ETM_CLK_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn soc_etm_clk_en(&self) -> SOC_ETM_CLK_EN_R {
        SOC_ETM_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CTRL0")
            .field("etm_apb_clk_en", &self.etm_apb_clk_en())
            .field("etm_rst_en", &self.etm_rst_en())
            .field("etm_force_norst", &self.etm_force_norst())
            .field("soc_etm_clk_sel", &self.soc_etm_clk_sel())
            .field("soc_etm_clk_en", &self.soc_etm_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn etm_apb_clk_en(&mut self) -> ETM_APB_CLK_EN_W<'_, ETM_CTRL0_SPEC> {
        ETM_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn etm_rst_en(&mut self) -> ETM_RST_EN_W<'_, ETM_CTRL0_SPEC> {
        ETM_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn etm_force_norst(&mut self) -> ETM_FORCE_NORST_W<'_, ETM_CTRL0_SPEC> {
        ETM_FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn soc_etm_clk_sel(&mut self) -> SOC_ETM_CLK_SEL_W<'_, ETM_CTRL0_SPEC> {
        SOC_ETM_CLK_SEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn soc_etm_clk_en(&mut self) -> SOC_ETM_CLK_EN_W<'_, ETM_CTRL0_SPEC> {
        SOC_ETM_CLK_EN_W::new(self, 6)
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
