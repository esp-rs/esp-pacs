#[doc = "Register `GPSPI3_CTRL0` reader"]
pub type R = crate::R<GPSPI3_CTRL0_SPEC>;
#[doc = "Register `GPSPI3_CTRL0` writer"]
pub type W = crate::W<GPSPI3_CTRL0_SPEC>;
#[doc = "Field `SYS_CLK_EN` reader - need_des"]
pub type SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - need_des"]
pub type SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_CLK_EN` reader - need_des"]
pub type APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_CLK_EN` writer - need_des"]
pub type APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - need_des"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - need_des"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NORST` reader - need_des"]
pub type FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FORCE_NORST` writer - need_des"]
pub type FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SRC_SEL` reader - need_des"]
pub type CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - need_des"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HS_CLK_EN` reader - need_des"]
pub type HS_CLK_EN_R = crate::BitReader;
#[doc = "Field `HS_CLK_EN` writer - need_des"]
pub type HS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_CLK_DIV_NUM` reader - need_des"]
pub type HS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `HS_CLK_DIV_NUM` writer - need_des"]
pub type HS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MST_CLK_DIV_NUM` reader - need_des"]
pub type MST_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MST_CLK_DIV_NUM` writer - need_des"]
pub type MST_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MST_CLK_EN` reader - need_des"]
pub type MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `MST_CLK_EN` writer - need_des"]
pub type MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SYS_CLK_EN_R {
        SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&self) -> APB_CLK_EN_R {
        APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_norst(&self) -> FORCE_NORST_R {
        FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn hs_clk_en(&self) -> HS_CLK_EN_R {
        HS_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - need_des"]
    #[inline(always)]
    pub fn hs_clk_div_num(&self) -> HS_CLK_DIV_NUM_R {
        HS_CLK_DIV_NUM_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn mst_clk_div_num(&self) -> MST_CLK_DIV_NUM_R {
        MST_CLK_DIV_NUM_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn mst_clk_en(&self) -> MST_CLK_EN_R {
        MST_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPSPI3_CTRL0")
            .field("sys_clk_en", &self.sys_clk_en())
            .field("apb_clk_en", &self.apb_clk_en())
            .field("rst_en", &self.rst_en())
            .field("force_norst", &self.force_norst())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("hs_clk_en", &self.hs_clk_en())
            .field("hs_clk_div_num", &self.hs_clk_div_num())
            .field("mst_clk_div_num", &self.mst_clk_div_num())
            .field("mst_clk_en", &self.mst_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SYS_CLK_EN_W<'_, GPSPI3_CTRL0_SPEC> {
        SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&mut self) -> APB_CLK_EN_W<'_, GPSPI3_CTRL0_SPEC> {
        APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, GPSPI3_CTRL0_SPEC> {
        RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, GPSPI3_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<'_, GPSPI3_CTRL0_SPEC> {
        CLK_SRC_SEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn hs_clk_en(&mut self) -> HS_CLK_EN_W<'_, GPSPI3_CTRL0_SPEC> {
        HS_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - need_des"]
    #[inline(always)]
    pub fn hs_clk_div_num(&mut self) -> HS_CLK_DIV_NUM_W<'_, GPSPI3_CTRL0_SPEC> {
        HS_CLK_DIV_NUM_W::new(self, 7)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn mst_clk_div_num(&mut self) -> MST_CLK_DIV_NUM_W<'_, GPSPI3_CTRL0_SPEC> {
        MST_CLK_DIV_NUM_W::new(self, 15)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn mst_clk_en(&mut self) -> MST_CLK_EN_W<'_, GPSPI3_CTRL0_SPEC> {
        MST_CLK_EN_W::new(self, 23)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi3_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi3_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSPI3_CTRL0_SPEC;
impl crate::RegisterSpec for GPSPI3_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpspi3_ctrl0::R`](R) reader structure"]
impl crate::Readable for GPSPI3_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpspi3_ctrl0::W`](W) writer structure"]
impl crate::Writable for GPSPI3_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPSPI3_CTRL0 to value 0x0080_0043"]
impl crate::Resettable for GPSPI3_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0080_0043;
}
