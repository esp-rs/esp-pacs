#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_TSENS_INT_RAW` reader - saradc tsens interrupt raw"]
pub type APB_SARADC_TSENS_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_TSENS_INT_RAW` writer - saradc tsens interrupt raw"]
pub type APB_SARADC_TSENS_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_RAW` reader - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_RAW` writer - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_RAW` reader - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_RAW` writer - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_RAW` reader - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_RAW` writer - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_RAW` reader - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_RAW` writer - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` reader - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` writer - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` reader - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` writer - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 25 - saradc tsens interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_tsens_int_raw(&self) -> APB_SARADC_TSENS_INT_RAW_R {
        APB_SARADC_TSENS_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_raw(&self) -> APB_SARADC_THRES1_LOW_INT_RAW_R {
        APB_SARADC_THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_raw(&self) -> APB_SARADC_THRES0_LOW_INT_RAW_R {
        APB_SARADC_THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_raw(&self) -> APB_SARADC_THRES1_HIGH_INT_RAW_R {
        APB_SARADC_THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_raw(&self) -> APB_SARADC_THRES0_HIGH_INT_RAW_R {
        APB_SARADC_THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_raw(&self) -> APB_SARADC2_DONE_INT_RAW_R {
        APB_SARADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_raw(&self) -> APB_SARADC1_DONE_INT_RAW_R {
        APB_SARADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "apb_saradc_tsens_int_raw",
                &format_args!("{}", self.apb_saradc_tsens_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres1_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres1_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_high_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_high_int_raw().bit()),
            )
            .field(
                "apb_saradc2_done_int_raw",
                &format_args!("{}", self.apb_saradc2_done_int_raw().bit()),
            )
            .field(
                "apb_saradc1_done_int_raw",
                &format_args!("{}", self.apb_saradc1_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - saradc tsens interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_tsens_int_raw(&mut self) -> APB_SARADC_TSENS_INT_RAW_W<25> {
        APB_SARADC_TSENS_INT_RAW_W::new(self)
    }
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_low_int_raw(&mut self) -> APB_SARADC_THRES1_LOW_INT_RAW_W<26> {
        APB_SARADC_THRES1_LOW_INT_RAW_W::new(self)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_low_int_raw(&mut self) -> APB_SARADC_THRES0_LOW_INT_RAW_W<27> {
        APB_SARADC_THRES0_LOW_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_high_int_raw(&mut self) -> APB_SARADC_THRES1_HIGH_INT_RAW_W<28> {
        APB_SARADC_THRES1_HIGH_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_high_int_raw(&mut self) -> APB_SARADC_THRES0_HIGH_INT_RAW_W<29> {
        APB_SARADC_THRES0_HIGH_INT_RAW_W::new(self)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc2_done_int_raw(&mut self) -> APB_SARADC2_DONE_INT_RAW_W<30> {
        APB_SARADC2_DONE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc1_done_int_raw(&mut self) -> APB_SARADC1_DONE_INT_RAW_W<31> {
        APB_SARADC1_DONE_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc int register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
