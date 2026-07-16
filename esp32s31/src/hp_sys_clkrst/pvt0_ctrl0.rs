#[doc = "Register `PVT0_CTRL0` reader"]
pub type R = crate::R<PVT0_CTRL0_SPEC>;
#[doc = "Register `PVT0_CTRL0` writer"]
pub type W = crate::W<PVT0_CTRL0_SPEC>;
#[doc = "Field `PVT0_SYS_CLK_EN` reader - need_des"]
pub type PVT0_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_SYS_CLK_EN` writer - need_des"]
pub type PVT0_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_TOP_RST_EN` reader - need_des"]
pub type PVT0_TOP_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_TOP_RST_EN` writer - need_des"]
pub type PVT0_TOP_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_APB_RST_EN` reader - need_des"]
pub type PVT0_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT0_APB_RST_EN` writer - need_des"]
pub type PVT0_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT0_CLK_DIV_NUM` reader - need_des"]
pub type PVT0_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PVT0_CLK_DIV_NUM` writer - need_des"]
pub type PVT0_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT0_CLK_EN` reader - need_des"]
pub type PVT0_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT0_CLK_EN` writer - need_des"]
pub type PVT0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pvt0_sys_clk_en(&self) -> PVT0_SYS_CLK_EN_R {
        PVT0_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pvt0_top_rst_en(&self) -> PVT0_TOP_RST_EN_R {
        PVT0_TOP_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pvt0_apb_rst_en(&self) -> PVT0_APB_RST_EN_R {
        PVT0_APB_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn pvt0_clk_div_num(&self) -> PVT0_CLK_DIV_NUM_R {
        PVT0_CLK_DIV_NUM_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn pvt0_clk_en(&self) -> PVT0_CLK_EN_R {
        PVT0_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT0_CTRL0")
            .field("pvt0_sys_clk_en", &self.pvt0_sys_clk_en())
            .field("pvt0_top_rst_en", &self.pvt0_top_rst_en())
            .field("pvt0_apb_rst_en", &self.pvt0_apb_rst_en())
            .field("pvt0_clk_div_num", &self.pvt0_clk_div_num())
            .field("pvt0_clk_en", &self.pvt0_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pvt0_sys_clk_en(&mut self) -> PVT0_SYS_CLK_EN_W<'_, PVT0_CTRL0_SPEC> {
        PVT0_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn pvt0_top_rst_en(&mut self) -> PVT0_TOP_RST_EN_W<'_, PVT0_CTRL0_SPEC> {
        PVT0_TOP_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pvt0_apb_rst_en(&mut self) -> PVT0_APB_RST_EN_W<'_, PVT0_CTRL0_SPEC> {
        PVT0_APB_RST_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn pvt0_clk_div_num(&mut self) -> PVT0_CLK_DIV_NUM_W<'_, PVT0_CTRL0_SPEC> {
        PVT0_CLK_DIV_NUM_W::new(self, 3)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn pvt0_clk_en(&mut self) -> PVT0_CLK_EN_W<'_, PVT0_CTRL0_SPEC> {
        PVT0_CLK_EN_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt0_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt0_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PVT0_CTRL0_SPEC;
impl crate::RegisterSpec for PVT0_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt0_ctrl0::R`](R) reader structure"]
impl crate::Readable for PVT0_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pvt0_ctrl0::W`](W) writer structure"]
impl crate::Writable for PVT0_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT0_CTRL0 to value 0"]
impl crate::Resettable for PVT0_CTRL0_SPEC {}
