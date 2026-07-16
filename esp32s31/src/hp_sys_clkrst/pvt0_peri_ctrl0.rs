#[doc = "Register `PVT0_PERI_CTRL0` reader"]
pub type R = crate::R<PVT0_PERI_CTRL0_SPEC>;
#[doc = "Register `PVT0_PERI_CTRL0` writer"]
pub type W = crate::W<PVT0_PERI_CTRL0_SPEC>;
#[doc = "Field `PVT0_PERI_GROUP1_RST_EN` reader - need_des"]
pub type PVT0_PERI_GROUP1_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP1_RST_EN` writer - need_des"]
pub type PVT0_PERI_GROUP1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP2_RST_EN` reader - need_des"]
pub type PVT0_PERI_GROUP2_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP2_RST_EN` writer - need_des"]
pub type PVT0_PERI_GROUP2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP3_RST_EN` reader - need_des"]
pub type PVT0_PERI_GROUP3_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP3_RST_EN` writer - need_des"]
pub type PVT0_PERI_GROUP3_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP4_RST_EN` reader - need_des"]
pub type PVT0_PERI_GROUP4_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP4_RST_EN` writer - need_des"]
pub type PVT0_PERI_GROUP4_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP1_CLK_EN` reader - TOP power domian pvt clk en"]
pub type PVT0_PERI_GROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP1_CLK_EN` writer - TOP power domian pvt clk en"]
pub type PVT0_PERI_GROUP1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP2_CLK_EN` reader - TOP power domian pvt clk en"]
pub type PVT0_PERI_GROUP2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP2_CLK_EN` writer - TOP power domian pvt clk en"]
pub type PVT0_PERI_GROUP2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP3_CLK_EN` reader - modem power domian pvt clk en"]
pub type PVT0_PERI_GROUP3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP3_CLK_EN` writer - modem power domian pvt clk en"]
pub type PVT0_PERI_GROUP3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_PERI_GROUP4_CLK_EN` reader - modem power domian pvt clk en"]
pub type PVT0_PERI_GROUP4_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_PERI_GROUP4_CLK_EN` writer - modem power domian pvt clk en"]
pub type PVT0_PERI_GROUP4_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group1_rst_en(&self) -> PVT0_PERI_GROUP1_RST_EN_R {
        PVT0_PERI_GROUP1_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group2_rst_en(&self) -> PVT0_PERI_GROUP2_RST_EN_R {
        PVT0_PERI_GROUP2_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group3_rst_en(&self) -> PVT0_PERI_GROUP3_RST_EN_R {
        PVT0_PERI_GROUP3_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group4_rst_en(&self) -> PVT0_PERI_GROUP4_RST_EN_R {
        PVT0_PERI_GROUP4_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group1_clk_en(&self) -> PVT0_PERI_GROUP1_CLK_EN_R {
        PVT0_PERI_GROUP1_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group2_clk_en(&self) -> PVT0_PERI_GROUP2_CLK_EN_R {
        PVT0_PERI_GROUP2_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group3_clk_en(&self) -> PVT0_PERI_GROUP3_CLK_EN_R {
        PVT0_PERI_GROUP3_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group4_clk_en(&self) -> PVT0_PERI_GROUP4_CLK_EN_R {
        PVT0_PERI_GROUP4_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT0_PERI_CTRL0")
            .field("pvt0_peri_group1_rst_en", &self.pvt0_peri_group1_rst_en())
            .field("pvt0_peri_group2_rst_en", &self.pvt0_peri_group2_rst_en())
            .field("pvt0_peri_group3_rst_en", &self.pvt0_peri_group3_rst_en())
            .field("pvt0_peri_group4_rst_en", &self.pvt0_peri_group4_rst_en())
            .field("pvt0_peri_group1_clk_en", &self.pvt0_peri_group1_clk_en())
            .field("pvt0_peri_group2_clk_en", &self.pvt0_peri_group2_clk_en())
            .field("pvt0_peri_group3_clk_en", &self.pvt0_peri_group3_clk_en())
            .field("pvt0_peri_group4_clk_en", &self.pvt0_peri_group4_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group1_rst_en(
        &mut self,
    ) -> PVT0_PERI_GROUP1_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP1_RST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group2_rst_en(
        &mut self,
    ) -> PVT0_PERI_GROUP2_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP2_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group3_rst_en(
        &mut self,
    ) -> PVT0_PERI_GROUP3_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP3_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn pvt0_peri_group4_rst_en(
        &mut self,
    ) -> PVT0_PERI_GROUP4_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP4_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group1_clk_en(
        &mut self,
    ) -> PVT0_PERI_GROUP1_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP1_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group2_clk_en(
        &mut self,
    ) -> PVT0_PERI_GROUP2_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP2_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group3_clk_en(
        &mut self,
    ) -> PVT0_PERI_GROUP3_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP3_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn pvt0_peri_group4_clk_en(
        &mut self,
    ) -> PVT0_PERI_GROUP4_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        PVT0_PERI_GROUP4_CLK_EN_W::new(self, 7)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt0_peri_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt0_peri_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PVT0_PERI_CTRL0_SPEC;
impl crate::RegisterSpec for PVT0_PERI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt0_peri_ctrl0::R`](R) reader structure"]
impl crate::Readable for PVT0_PERI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pvt0_peri_ctrl0::W`](W) writer structure"]
impl crate::Writable for PVT0_PERI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT0_PERI_CTRL0 to value 0"]
impl crate::Resettable for PVT0_PERI_CTRL0_SPEC {}
