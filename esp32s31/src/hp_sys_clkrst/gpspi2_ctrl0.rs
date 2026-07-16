#[doc = "Register `GPSPI2_CTRL0` reader"]
pub type R = crate::R<GPSPI2_CTRL0_SPEC>;
#[doc = "Register `GPSPI2_CTRL0` writer"]
pub type W = crate::W<GPSPI2_CTRL0_SPEC>;
#[doc = "Field `GPSPI2_SYS_CLK_EN` reader - need_des"]
pub type GPSPI2_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_SYS_CLK_EN` writer - need_des"]
pub type GPSPI2_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_APB_CLK_EN` reader - need_des"]
pub type GPSPI2_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_APB_CLK_EN` writer - need_des"]
pub type GPSPI2_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_RST_EN` reader - need_des"]
pub type GPSPI2_RST_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_RST_EN` writer - need_des"]
pub type GPSPI2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_FORCE_NORST` reader - need_des"]
pub type GPSPI2_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `GPSPI2_FORCE_NORST` writer - need_des"]
pub type GPSPI2_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_CLK_SRC_SEL` reader - need_des"]
pub type GPSPI2_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `GPSPI2_CLK_SRC_SEL` writer - need_des"]
pub type GPSPI2_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPSPI2_HS_CLK_EN` reader - need_des"]
pub type GPSPI2_HS_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_HS_CLK_EN` writer - need_des"]
pub type GPSPI2_HS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_HS_CLK_DIV_NUM` reader - need_des"]
pub type GPSPI2_HS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI2_HS_CLK_DIV_NUM` writer - need_des"]
pub type GPSPI2_HS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI2_MST_CLK_DIV_NUM` reader - need_des"]
pub type GPSPI2_MST_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI2_MST_CLK_DIV_NUM` writer - need_des"]
pub type GPSPI2_MST_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI2_MST_CLK_EN` reader - need_des"]
pub type GPSPI2_MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_MST_CLK_EN` writer - need_des"]
pub type GPSPI2_MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn gpspi2_sys_clk_en(&self) -> GPSPI2_SYS_CLK_EN_R {
        GPSPI2_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn gpspi2_apb_clk_en(&self) -> GPSPI2_APB_CLK_EN_R {
        GPSPI2_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn gpspi2_rst_en(&self) -> GPSPI2_RST_EN_R {
        GPSPI2_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn gpspi2_force_norst(&self) -> GPSPI2_FORCE_NORST_R {
        GPSPI2_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn gpspi2_clk_src_sel(&self) -> GPSPI2_CLK_SRC_SEL_R {
        GPSPI2_CLK_SRC_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_en(&self) -> GPSPI2_HS_CLK_EN_R {
        GPSPI2_HS_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - need_des"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_div_num(&self) -> GPSPI2_HS_CLK_DIV_NUM_R {
        GPSPI2_HS_CLK_DIV_NUM_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_div_num(&self) -> GPSPI2_MST_CLK_DIV_NUM_R {
        GPSPI2_MST_CLK_DIV_NUM_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_en(&self) -> GPSPI2_MST_CLK_EN_R {
        GPSPI2_MST_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPSPI2_CTRL0")
            .field("gpspi2_sys_clk_en", &self.gpspi2_sys_clk_en())
            .field("gpspi2_apb_clk_en", &self.gpspi2_apb_clk_en())
            .field("gpspi2_rst_en", &self.gpspi2_rst_en())
            .field("gpspi2_force_norst", &self.gpspi2_force_norst())
            .field("gpspi2_clk_src_sel", &self.gpspi2_clk_src_sel())
            .field("gpspi2_hs_clk_en", &self.gpspi2_hs_clk_en())
            .field("gpspi2_hs_clk_div_num", &self.gpspi2_hs_clk_div_num())
            .field("gpspi2_mst_clk_div_num", &self.gpspi2_mst_clk_div_num())
            .field("gpspi2_mst_clk_en", &self.gpspi2_mst_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn gpspi2_sys_clk_en(&mut self) -> GPSPI2_SYS_CLK_EN_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn gpspi2_apb_clk_en(&mut self) -> GPSPI2_APB_CLK_EN_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn gpspi2_rst_en(&mut self) -> GPSPI2_RST_EN_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn gpspi2_force_norst(&mut self) -> GPSPI2_FORCE_NORST_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn gpspi2_clk_src_sel(&mut self) -> GPSPI2_CLK_SRC_SEL_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_CLK_SRC_SEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_en(&mut self) -> GPSPI2_HS_CLK_EN_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_HS_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - need_des"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_div_num(&mut self) -> GPSPI2_HS_CLK_DIV_NUM_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_HS_CLK_DIV_NUM_W::new(self, 7)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_div_num(&mut self) -> GPSPI2_MST_CLK_DIV_NUM_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_MST_CLK_DIV_NUM_W::new(self, 15)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_en(&mut self) -> GPSPI2_MST_CLK_EN_W<'_, GPSPI2_CTRL0_SPEC> {
        GPSPI2_MST_CLK_EN_W::new(self, 23)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi2_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi2_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSPI2_CTRL0_SPEC;
impl crate::RegisterSpec for GPSPI2_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpspi2_ctrl0::R`](R) reader structure"]
impl crate::Readable for GPSPI2_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpspi2_ctrl0::W`](W) writer structure"]
impl crate::Writable for GPSPI2_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPSPI2_CTRL0 to value 0x0080_0043"]
impl crate::Resettable for GPSPI2_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0080_0043;
}
