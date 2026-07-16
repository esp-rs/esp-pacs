#[doc = "Register `TSENS_CTRL` reader"]
pub type R = crate::R<TSENS_CTRL_SPEC>;
#[doc = "Register `TSENS_CTRL` writer"]
pub type W = crate::W<TSENS_CTRL_SPEC>;
#[doc = "Field `LP_TSENS_CLK_EN` reader - need_des"]
pub type LP_TSENS_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_TSENS_CLK_EN` writer - need_des"]
pub type LP_TSENS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TSENS_RST_EN` reader - need_des"]
pub type LP_TSENS_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_TSENS_RST_EN` writer - need_des"]
pub type LP_TSENS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_tsens_clk_en(&self) -> LP_TSENS_CLK_EN_R {
        LP_TSENS_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_tsens_rst_en(&self) -> LP_TSENS_RST_EN_R {
        LP_TSENS_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSENS_CTRL")
            .field("lp_tsens_clk_en", &self.lp_tsens_clk_en())
            .field("lp_tsens_rst_en", &self.lp_tsens_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_tsens_clk_en(&mut self) -> LP_TSENS_CLK_EN_W<'_, TSENS_CTRL_SPEC> {
        LP_TSENS_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_tsens_rst_en(&mut self) -> LP_TSENS_RST_EN_W<'_, TSENS_CTRL_SPEC> {
        LP_TSENS_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSENS_CTRL_SPEC;
impl crate::RegisterSpec for TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_ctrl::R`](R) reader structure"]
impl crate::Readable for TSENS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsens_ctrl::W`](W) writer structure"]
impl crate::Writable for TSENS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSENS_CTRL to value 0"]
impl crate::Resettable for TSENS_CTRL_SPEC {}
