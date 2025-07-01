#[doc = "Register `PERI_CLK_CTRL116` reader"]
pub type R = crate::R<PERI_CLK_CTRL116_SPEC>;
#[doc = "Register `PERI_CLK_CTRL116` writer"]
pub type W = crate::W<PERI_CLK_CTRL116_SPEC>;
#[doc = "Field `GPSPI2_CLK_SRC_SEL` reader - Reserved"]
pub type GPSPI2_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `GPSPI2_CLK_SRC_SEL` writer - Reserved"]
pub type GPSPI2_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPSPI2_HS_CLK_EN` reader - Reserved"]
pub type GPSPI2_HS_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_HS_CLK_EN` writer - Reserved"]
pub type GPSPI2_HS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI2_HS_CLK_DIV_NUM` reader - Reserved"]
pub type GPSPI2_HS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI2_HS_CLK_DIV_NUM` writer - Reserved"]
pub type GPSPI2_HS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI2_MST_CLK_DIV_NUM` reader - Reserved"]
pub type GPSPI2_MST_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI2_MST_CLK_DIV_NUM` writer - Reserved"]
pub type GPSPI2_MST_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI2_MST_CLK_EN` reader - Reserved"]
pub type GPSPI2_MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI2_MST_CLK_EN` writer - Reserved"]
pub type GPSPI2_MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSPI3_CLK_SRC_SEL` reader - Reserved"]
pub type GPSPI3_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `GPSPI3_CLK_SRC_SEL` writer - Reserved"]
pub type GPSPI3_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPSPI3_HS_CLK_EN` reader - Reserved"]
pub type GPSPI3_HS_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI3_HS_CLK_EN` writer - Reserved"]
pub type GPSPI3_HS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_clk_src_sel(&self) -> GPSPI2_CLK_SRC_SEL_R {
        GPSPI2_CLK_SRC_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_en(&self) -> GPSPI2_HS_CLK_EN_R {
        GPSPI2_HS_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_div_num(&self) -> GPSPI2_HS_CLK_DIV_NUM_R {
        GPSPI2_HS_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_div_num(&self) -> GPSPI2_MST_CLK_DIV_NUM_R {
        GPSPI2_MST_CLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_en(&self) -> GPSPI2_MST_CLK_EN_R {
        GPSPI2_MST_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_clk_src_sel(&self) -> GPSPI3_CLK_SRC_SEL_R {
        GPSPI3_CLK_SRC_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_hs_clk_en(&self) -> GPSPI3_HS_CLK_EN_R {
        GPSPI3_HS_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL116")
            .field("gpspi2_clk_src_sel", &self.gpspi2_clk_src_sel())
            .field("gpspi2_hs_clk_en", &self.gpspi2_hs_clk_en())
            .field("gpspi2_hs_clk_div_num", &self.gpspi2_hs_clk_div_num())
            .field("gpspi2_mst_clk_div_num", &self.gpspi2_mst_clk_div_num())
            .field("gpspi2_mst_clk_en", &self.gpspi2_mst_clk_en())
            .field("gpspi3_clk_src_sel", &self.gpspi3_clk_src_sel())
            .field("gpspi3_hs_clk_en", &self.gpspi3_hs_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_clk_src_sel(&mut self) -> GPSPI2_CLK_SRC_SEL_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI2_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_en(&mut self) -> GPSPI2_HS_CLK_EN_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI2_HS_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_hs_clk_div_num(&mut self) -> GPSPI2_HS_CLK_DIV_NUM_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI2_HS_CLK_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 12:19 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_div_num(&mut self) -> GPSPI2_MST_CLK_DIV_NUM_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI2_MST_CLK_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn gpspi2_mst_clk_en(&mut self) -> GPSPI2_MST_CLK_EN_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI2_MST_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_clk_src_sel(&mut self) -> GPSPI3_CLK_SRC_SEL_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI3_CLK_SRC_SEL_W::new(self, 21)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_hs_clk_en(&mut self) -> GPSPI3_HS_CLK_EN_W<PERI_CLK_CTRL116_SPEC> {
        GPSPI3_HS_CLK_EN_W::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl116::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl116::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL116_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL116_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl116::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL116_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl116::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL116_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL116 to value 0x0110_0008"]
impl crate::Resettable for PERI_CLK_CTRL116_SPEC {
    const RESET_VALUE: u32 = 0x0110_0008;
}
