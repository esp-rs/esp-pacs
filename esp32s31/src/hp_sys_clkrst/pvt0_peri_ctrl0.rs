#[doc = "Register `PVT0_PERI_CTRL0` reader"]
pub type R = crate::R<PVT0_PERI_CTRL0_SPEC>;
#[doc = "Register `PVT0_PERI_CTRL0` writer"]
pub type W = crate::W<PVT0_PERI_CTRL0_SPEC>;
#[doc = "Field `GROUP1_RST_EN` reader - need_des"]
pub type GROUP1_RST_EN_R = crate::BitReader;
#[doc = "Field `GROUP1_RST_EN` writer - need_des"]
pub type GROUP1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP2_RST_EN` reader - need_des"]
pub type GROUP2_RST_EN_R = crate::BitReader;
#[doc = "Field `GROUP2_RST_EN` writer - need_des"]
pub type GROUP2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP3_RST_EN` reader - need_des"]
pub type GROUP3_RST_EN_R = crate::BitReader;
#[doc = "Field `GROUP3_RST_EN` writer - need_des"]
pub type GROUP3_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP4_RST_EN` reader - need_des"]
pub type GROUP4_RST_EN_R = crate::BitReader;
#[doc = "Field `GROUP4_RST_EN` writer - need_des"]
pub type GROUP4_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP1_CLK_EN` reader - TOP power domian pvt clk en"]
pub type GROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `GROUP1_CLK_EN` writer - TOP power domian pvt clk en"]
pub type GROUP1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP2_CLK_EN` reader - TOP power domian pvt clk en"]
pub type GROUP2_CLK_EN_R = crate::BitReader;
#[doc = "Field `GROUP2_CLK_EN` writer - TOP power domian pvt clk en"]
pub type GROUP2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP3_CLK_EN` reader - modem power domian pvt clk en"]
pub type GROUP3_CLK_EN_R = crate::BitReader;
#[doc = "Field `GROUP3_CLK_EN` writer - modem power domian pvt clk en"]
pub type GROUP3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP4_CLK_EN` reader - modem power domian pvt clk en"]
pub type GROUP4_CLK_EN_R = crate::BitReader;
#[doc = "Field `GROUP4_CLK_EN` writer - modem power domian pvt clk en"]
pub type GROUP4_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn group1_rst_en(&self) -> GROUP1_RST_EN_R {
        GROUP1_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn group2_rst_en(&self) -> GROUP2_RST_EN_R {
        GROUP2_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn group3_rst_en(&self) -> GROUP3_RST_EN_R {
        GROUP3_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn group4_rst_en(&self) -> GROUP4_RST_EN_R {
        GROUP4_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn group1_clk_en(&self) -> GROUP1_CLK_EN_R {
        GROUP1_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn group2_clk_en(&self) -> GROUP2_CLK_EN_R {
        GROUP2_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn group3_clk_en(&self) -> GROUP3_CLK_EN_R {
        GROUP3_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn group4_clk_en(&self) -> GROUP4_CLK_EN_R {
        GROUP4_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT0_PERI_CTRL0")
            .field("group1_rst_en", &self.group1_rst_en())
            .field("group2_rst_en", &self.group2_rst_en())
            .field("group3_rst_en", &self.group3_rst_en())
            .field("group4_rst_en", &self.group4_rst_en())
            .field("group1_clk_en", &self.group1_clk_en())
            .field("group2_clk_en", &self.group2_clk_en())
            .field("group3_clk_en", &self.group3_clk_en())
            .field("group4_clk_en", &self.group4_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn group1_rst_en(&mut self) -> GROUP1_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP1_RST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn group2_rst_en(&mut self) -> GROUP2_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP2_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn group3_rst_en(&mut self) -> GROUP3_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP3_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn group4_rst_en(&mut self) -> GROUP4_RST_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP4_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn group1_clk_en(&mut self) -> GROUP1_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP1_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TOP power domian pvt clk en"]
    #[inline(always)]
    pub fn group2_clk_en(&mut self) -> GROUP2_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP2_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn group3_clk_en(&mut self) -> GROUP3_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP3_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - modem power domian pvt clk en"]
    #[inline(always)]
    pub fn group4_clk_en(&mut self) -> GROUP4_CLK_EN_W<'_, PVT0_PERI_CTRL0_SPEC> {
        GROUP4_CLK_EN_W::new(self, 7)
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
