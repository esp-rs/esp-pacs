#[doc = "Register `PAD_BIST_GMAC0` reader"]
pub type R = crate::R<PAD_BIST_GMAC0_SPEC>;
#[doc = "Register `PAD_BIST_GMAC0` writer"]
pub type W = crate::W<PAD_BIST_GMAC0_SPEC>;
#[doc = "Field `OUT_CLK_SEL` reader - need_des"]
pub type OUT_CLK_SEL_R = crate::BitReader;
#[doc = "Field `OUT_CLK_SEL` writer - need_des"]
pub type OUT_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CLK_EN` reader - need_des"]
pub type OUT_CLK_EN_R = crate::BitReader;
#[doc = "Field `OUT_CLK_EN` writer - need_des"]
pub type OUT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CLK_DIV_NUM` reader - need_des"]
pub type OUT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `OUT_CLK_DIV_NUM` writer - need_des"]
pub type OUT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IN_CLK_SEL` reader - need_des"]
pub type IN_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `IN_CLK_SEL` writer - need_des"]
pub type IN_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_CLK_EN` reader - need_des"]
pub type IN_CLK_EN_R = crate::BitReader;
#[doc = "Field `IN_CLK_EN` writer - need_des"]
pub type IN_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - need_des"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - need_des"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn out_clk_sel(&self) -> OUT_CLK_SEL_R {
        OUT_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn out_clk_en(&self) -> OUT_CLK_EN_R {
        OUT_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - need_des"]
    #[inline(always)]
    pub fn out_clk_div_num(&self) -> OUT_CLK_DIV_NUM_R {
        OUT_CLK_DIV_NUM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - need_des"]
    #[inline(always)]
    pub fn in_clk_sel(&self) -> IN_CLK_SEL_R {
        IN_CLK_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn in_clk_en(&self) -> IN_CLK_EN_R {
        IN_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_BIST_GMAC0")
            .field("out_clk_sel", &self.out_clk_sel())
            .field("out_clk_en", &self.out_clk_en())
            .field("out_clk_div_num", &self.out_clk_div_num())
            .field("in_clk_sel", &self.in_clk_sel())
            .field("in_clk_en", &self.in_clk_en())
            .field("rst_en", &self.rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn out_clk_sel(&mut self) -> OUT_CLK_SEL_W<'_, PAD_BIST_GMAC0_SPEC> {
        OUT_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn out_clk_en(&mut self) -> OUT_CLK_EN_W<'_, PAD_BIST_GMAC0_SPEC> {
        OUT_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - need_des"]
    #[inline(always)]
    pub fn out_clk_div_num(&mut self) -> OUT_CLK_DIV_NUM_W<'_, PAD_BIST_GMAC0_SPEC> {
        OUT_CLK_DIV_NUM_W::new(self, 2)
    }
    #[doc = "Bits 6:8 - need_des"]
    #[inline(always)]
    pub fn in_clk_sel(&mut self) -> IN_CLK_SEL_W<'_, PAD_BIST_GMAC0_SPEC> {
        IN_CLK_SEL_W::new(self, 6)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn in_clk_en(&mut self) -> IN_CLK_EN_W<'_, PAD_BIST_GMAC0_SPEC> {
        IN_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, PAD_BIST_GMAC0_SPEC> {
        RST_EN_W::new(self, 10)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_bist_gmac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_bist_gmac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_BIST_GMAC0_SPEC;
impl crate::RegisterSpec for PAD_BIST_GMAC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_bist_gmac0::R`](R) reader structure"]
impl crate::Readable for PAD_BIST_GMAC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_bist_gmac0::W`](W) writer structure"]
impl crate::Writable for PAD_BIST_GMAC0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_BIST_GMAC0 to value 0x04"]
impl crate::Resettable for PAD_BIST_GMAC0_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
