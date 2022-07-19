#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES1_LOW_INT_CLR` writer - Need add description"]
pub type THRES1_LOW_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 26>;
#[doc = "Field `THRES0_LOW_INT_CLR` writer - Need add description"]
pub type THRES0_LOW_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 27>;
#[doc = "Field `THRES1_HIGH_INT_CLR` writer - Need add description"]
pub type THRES1_HIGH_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 28>;
#[doc = "Field `THRES0_HIGH_INT_CLR` writer - Need add description"]
pub type THRES0_HIGH_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 29>;
#[doc = "Field `APB_SARADC2_DONE_INT_CLR` writer - Need add description"]
pub type APB_SARADC2_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 30>;
#[doc = "Field `APB_SARADC1_DONE_INT_CLR` writer - Need add description"]
pub type APB_SARADC1_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 26 - Need add description"]
    #[inline(always)]
    pub fn thres1_low_int_clr(&mut self) -> THRES1_LOW_INT_CLR_W {
        THRES1_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres0_low_int_clr(&mut self) -> THRES0_LOW_INT_CLR_W {
        THRES0_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres1_high_int_clr(&mut self) -> THRES1_HIGH_INT_CLR_W {
        THRES1_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres0_high_int_clr(&mut self) -> THRES0_HIGH_INT_CLR_W {
        THRES0_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_clr(&mut self) -> APB_SARADC2_DONE_INT_CLR_W {
        APB_SARADC2_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_clr(&mut self) -> APB_SARADC1_DONE_INT_CLR_W {
        APB_SARADC1_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
