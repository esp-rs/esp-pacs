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
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_CLR` writer - saradc thres1 low interrupt clear"]
pub type APB_SARADC_THRES1_LOW_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_CLR` writer - saradc thres0 low interrupt clear"]
pub type APB_SARADC_THRES0_LOW_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_CLR` writer - saradc thres1 high interrupt clear"]
pub type APB_SARADC_THRES1_HIGH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_CLR` writer - saradc thres0 high interrupt clear"]
pub type APB_SARADC_THRES0_HIGH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `APB_SARADC2_DONE_INT_CLR` writer - saradc2 done interrupt clear"]
pub type APB_SARADC2_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `APB_SARADC1_DONE_INT_CLR` writer - saradc1 done interrupt clear"]
pub type APB_SARADC1_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 26 - saradc thres1 low interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_clr(&mut self) -> APB_SARADC_THRES1_LOW_INT_CLR_W<26> {
        APB_SARADC_THRES1_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_clr(&mut self) -> APB_SARADC_THRES0_LOW_INT_CLR_W<27> {
        APB_SARADC_THRES0_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_clr(&mut self) -> APB_SARADC_THRES1_HIGH_INT_CLR_W<28> {
        APB_SARADC_THRES1_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_clr(&mut self) -> APB_SARADC_THRES0_HIGH_INT_CLR_W<29> {
        APB_SARADC_THRES0_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - saradc2 done interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_clr(&mut self) -> APB_SARADC2_DONE_INT_CLR_W<30> {
        APB_SARADC2_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - saradc1 done interrupt clear"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_clr(&mut self) -> APB_SARADC1_DONE_INT_CLR_W<31> {
        APB_SARADC1_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc int register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
