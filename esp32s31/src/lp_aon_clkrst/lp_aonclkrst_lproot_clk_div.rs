#[doc = "Register `LP_AONCLKRST_LPROOT_CLK_DIV` reader"]
pub type R = crate::R<LP_AONCLKRST_LPROOT_CLK_DIV_SPEC>;
#[doc = "Register `LP_AONCLKRST_LPROOT_CLK_DIV` writer"]
pub type W = crate::W<LP_AONCLKRST_LPROOT_CLK_DIV_SPEC>;
#[doc = "Field `LP_AONCLKRST_LPAON_CLK_DIV_NUM` reader - lp aon clock divide num"]
pub type LP_AONCLKRST_LPAON_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LPAON_CLK_DIV_NUM` writer - lp aon clock divide num"]
pub type LP_AONCLKRST_LPAON_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_LPPERI_CLK_DIV_NUM` reader - lp peri clock divide num"]
pub type LP_AONCLKRST_LPPERI_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LPPERI_CLK_DIV_NUM` writer - lp peri clock divide num"]
pub type LP_AONCLKRST_LPPERI_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - lp aon clock divide num"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpaon_clk_div_num(&self) -> LP_AONCLKRST_LPAON_CLK_DIV_NUM_R {
        LP_AONCLKRST_LPAON_CLK_DIV_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - lp peri clock divide num"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpperi_clk_div_num(&self) -> LP_AONCLKRST_LPPERI_CLK_DIV_NUM_R {
        LP_AONCLKRST_LPPERI_CLK_DIV_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LPROOT_CLK_DIV")
            .field(
                "lp_aonclkrst_lpaon_clk_div_num",
                &self.lp_aonclkrst_lpaon_clk_div_num(),
            )
            .field(
                "lp_aonclkrst_lpperi_clk_div_num",
                &self.lp_aonclkrst_lpperi_clk_div_num(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - lp aon clock divide num"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpaon_clk_div_num(
        &mut self,
    ) -> LP_AONCLKRST_LPAON_CLK_DIV_NUM_W<'_, LP_AONCLKRST_LPROOT_CLK_DIV_SPEC> {
        LP_AONCLKRST_LPAON_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - lp peri clock divide num"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpperi_clk_div_num(
        &mut self,
    ) -> LP_AONCLKRST_LPPERI_CLK_DIV_NUM_W<'_, LP_AONCLKRST_LPROOT_CLK_DIV_SPEC> {
        LP_AONCLKRST_LPPERI_CLK_DIV_NUM_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lproot_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lproot_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_LPROOT_CLK_DIV_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LPROOT_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lproot_clk_div::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_LPROOT_CLK_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lproot_clk_div::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_LPROOT_CLK_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LPROOT_CLK_DIV to value 0"]
impl crate::Resettable for LP_AONCLKRST_LPROOT_CLK_DIV_SPEC {}
