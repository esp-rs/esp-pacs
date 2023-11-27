#[doc = "Register `LP_AONCLKRST_DATE` reader"]
pub type R = crate::R<LP_AONCLKRST_DATE_SPEC>;
#[doc = "Register `LP_AONCLKRST_DATE` writer"]
pub type W = crate::W<LP_AONCLKRST_DATE_SPEC>;
#[doc = "Field `LP_AONCLKRST_CLK_EN` reader - need_des"]
pub type LP_AONCLKRST_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_CLK_EN` writer - need_des"]
pub type LP_AONCLKRST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_clk_en(&self) -> LP_AONCLKRST_CLK_EN_R {
        LP_AONCLKRST_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_DATE")
            .field(
                "lp_aonclkrst_clk_en",
                &format_args!("{}", self.lp_aonclkrst_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_en(&mut self) -> LP_AONCLKRST_CLK_EN_W<LP_AONCLKRST_DATE_SPEC> {
        LP_AONCLKRST_CLK_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_DATE_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_date::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_date::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_DATE to value 0"]
impl crate::Resettable for LP_AONCLKRST_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
