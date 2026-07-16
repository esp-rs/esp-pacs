#[doc = "Register `LCDCAM_LCDCAM_CTRL0` reader"]
pub type R = crate::R<LCDCAM_LCDCAM_CTRL0_SPEC>;
#[doc = "Register `LCDCAM_LCDCAM_CTRL0` writer"]
pub type W = crate::W<LCDCAM_LCDCAM_CTRL0_SPEC>;
#[doc = "Field `LCDCAM_CLK_SRC_SEL` reader - need_des"]
pub type LCDCAM_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LCDCAM_CLK_SRC_SEL` writer - need_des"]
pub type LCDCAM_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCDCAM_CLK_EN` reader - need_des"]
pub type LCDCAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `LCDCAM_CLK_EN` writer - need_des"]
pub type LCDCAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCAM_CLK_DIV_NUM` reader - need_des"]
pub type LCDCAM_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LCDCAM_CLK_DIV_NUM` writer - need_des"]
pub type LCDCAM_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCDCAM_CLK_DIV_NUMERATOR` reader - need_des"]
pub type LCDCAM_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `LCDCAM_CLK_DIV_NUMERATOR` writer - need_des"]
pub type LCDCAM_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCDCAM_CLK_DIV_DENOMINATOR` reader - need_des"]
pub type LCDCAM_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `LCDCAM_CLK_DIV_DENOMINATOR` writer - need_des"]
pub type LCDCAM_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_src_sel(&self) -> LCDCAM_CLK_SRC_SEL_R {
        LCDCAM_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_en(&self) -> LCDCAM_CLK_EN_R {
        LCDCAM_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_num(&self) -> LCDCAM_CLK_DIV_NUM_R {
        LCDCAM_CLK_DIV_NUM_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 11:18 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_numerator(&self) -> LCDCAM_CLK_DIV_NUMERATOR_R {
        LCDCAM_CLK_DIV_NUMERATOR_R::new(((self.bits >> 11) & 0xff) as u8)
    }
    #[doc = "Bits 19:26 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_denominator(&self) -> LCDCAM_CLK_DIV_DENOMINATOR_R {
        LCDCAM_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCDCAM_LCDCAM_CTRL0")
            .field("lcdcam_clk_src_sel", &self.lcdcam_clk_src_sel())
            .field("lcdcam_clk_en", &self.lcdcam_clk_en())
            .field("lcdcam_clk_div_num", &self.lcdcam_clk_div_num())
            .field("lcdcam_clk_div_numerator", &self.lcdcam_clk_div_numerator())
            .field(
                "lcdcam_clk_div_denominator",
                &self.lcdcam_clk_div_denominator(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_src_sel(&mut self) -> LCDCAM_CLK_SRC_SEL_W<'_, LCDCAM_LCDCAM_CTRL0_SPEC> {
        LCDCAM_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_en(&mut self) -> LCDCAM_CLK_EN_W<'_, LCDCAM_LCDCAM_CTRL0_SPEC> {
        LCDCAM_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_num(&mut self) -> LCDCAM_CLK_DIV_NUM_W<'_, LCDCAM_LCDCAM_CTRL0_SPEC> {
        LCDCAM_CLK_DIV_NUM_W::new(self, 3)
    }
    #[doc = "Bits 11:18 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_numerator(
        &mut self,
    ) -> LCDCAM_CLK_DIV_NUMERATOR_W<'_, LCDCAM_LCDCAM_CTRL0_SPEC> {
        LCDCAM_CLK_DIV_NUMERATOR_W::new(self, 11)
    }
    #[doc = "Bits 19:26 - need_des"]
    #[inline(always)]
    pub fn lcdcam_clk_div_denominator(
        &mut self,
    ) -> LCDCAM_CLK_DIV_DENOMINATOR_W<'_, LCDCAM_LCDCAM_CTRL0_SPEC> {
        LCDCAM_CLK_DIV_DENOMINATOR_W::new(self, 19)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_lcdcam_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_lcdcam_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCDCAM_LCDCAM_CTRL0_SPEC;
impl crate::RegisterSpec for LCDCAM_LCDCAM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdcam_lcdcam_ctrl0::R`](R) reader structure"]
impl crate::Readable for LCDCAM_LCDCAM_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcdcam_lcdcam_ctrl0::W`](W) writer structure"]
impl crate::Writable for LCDCAM_LCDCAM_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCAM_LCDCAM_CTRL0 to value 0"]
impl crate::Resettable for LCDCAM_LCDCAM_CTRL0_SPEC {}
