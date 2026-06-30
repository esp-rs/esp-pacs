#[doc = "Register `LP_AONCLKRST_CPLL_DIV` reader"]
pub type R = crate::R<LP_AONCLKRST_CPLL_DIV_SPEC>;
#[doc = "Register `LP_AONCLKRST_CPLL_DIV` writer"]
pub type W = crate::W<LP_AONCLKRST_CPLL_DIV_SPEC>;
#[doc = "Field `LP_AONCLKRST_CPLL_REF_DIV` reader - cpll ref div value"]
pub type LP_AONCLKRST_CPLL_REF_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_REF_DIV` writer - cpll ref div value"]
pub type LP_AONCLKRST_CPLL_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_AONCLKRST_CPLL_FB_DIV` reader - cpll fb div value"]
pub type LP_AONCLKRST_CPLL_FB_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_FB_DIV` writer - cpll fb div value"]
pub type LP_AONCLKRST_CPLL_FB_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - cpll ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_ref_div(&self) -> LP_AONCLKRST_CPLL_REF_DIV_R {
        LP_AONCLKRST_CPLL_REF_DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - cpll fb div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_fb_div(&self) -> LP_AONCLKRST_CPLL_FB_DIV_R {
        LP_AONCLKRST_CPLL_FB_DIV_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_CPLL_DIV")
            .field(
                "lp_aonclkrst_cpll_ref_div",
                &self.lp_aonclkrst_cpll_ref_div(),
            )
            .field("lp_aonclkrst_cpll_fb_div", &self.lp_aonclkrst_cpll_fb_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - cpll ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_ref_div(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_REF_DIV_W<'_, LP_AONCLKRST_CPLL_DIV_SPEC> {
        LP_AONCLKRST_CPLL_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 4:11 - cpll fb div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_fb_div(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_FB_DIV_W<'_, LP_AONCLKRST_CPLL_DIV_SPEC> {
        LP_AONCLKRST_CPLL_FB_DIV_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_cpll_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_cpll_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_CPLL_DIV_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_CPLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_cpll_div::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_CPLL_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_cpll_div::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_CPLL_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_CPLL_DIV to value 0xf2"]
impl crate::Resettable for LP_AONCLKRST_CPLL_DIV_SPEC {
    const RESET_VALUE: u32 = 0xf2;
}
