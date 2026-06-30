#[doc = "Register `LP_AONCLKRST_APLL_DIV` reader"]
pub type R = crate::R<LP_AONCLKRST_APLL_DIV_SPEC>;
#[doc = "Register `LP_AONCLKRST_APLL_DIV` writer"]
pub type W = crate::W<LP_AONCLKRST_APLL_DIV_SPEC>;
#[doc = "Field `LP_AONCLKRST_APLL_REF_DIV` reader - apll ref div value"]
pub type LP_AONCLKRST_APLL_REF_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_APLL_REF_DIV` writer - apll ref div value"]
pub type LP_AONCLKRST_APLL_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_APLL_OUT_DIV` reader - apll out div value"]
pub type LP_AONCLKRST_APLL_OUT_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_APLL_OUT_DIV` writer - apll out div value"]
pub type LP_AONCLKRST_APLL_OUT_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - apll ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_apll_ref_div(&self) -> LP_AONCLKRST_APLL_REF_DIV_R {
        LP_AONCLKRST_APLL_REF_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - apll out div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_apll_out_div(&self) -> LP_AONCLKRST_APLL_OUT_DIV_R {
        LP_AONCLKRST_APLL_OUT_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_APLL_DIV")
            .field(
                "lp_aonclkrst_apll_ref_div",
                &self.lp_aonclkrst_apll_ref_div(),
            )
            .field(
                "lp_aonclkrst_apll_out_div",
                &self.lp_aonclkrst_apll_out_div(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - apll ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_apll_ref_div(
        &mut self,
    ) -> LP_AONCLKRST_APLL_REF_DIV_W<'_, LP_AONCLKRST_APLL_DIV_SPEC> {
        LP_AONCLKRST_APLL_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - apll out div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_apll_out_div(
        &mut self,
    ) -> LP_AONCLKRST_APLL_OUT_DIV_W<'_, LP_AONCLKRST_APLL_DIV_SPEC> {
        LP_AONCLKRST_APLL_OUT_DIV_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_apll_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_apll_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_APLL_DIV_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_APLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_apll_div::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_APLL_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_apll_div::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_APLL_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_APLL_DIV to value 0x85"]
impl crate::Resettable for LP_AONCLKRST_APLL_DIV_SPEC {
    const RESET_VALUE: u32 = 0x85;
}
