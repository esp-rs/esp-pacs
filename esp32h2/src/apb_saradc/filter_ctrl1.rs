#[doc = "Register `FILTER_CTRL1` reader"]
pub struct R(crate::R<FILTER_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CTRL1` writer"]
pub struct W(crate::W<FILTER_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CTRL1_SPEC>;
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
impl From<crate::W<FILTER_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_FILTER_FACTOR1` reader - Factor of saradc filter1"]
pub type APB_SARADC_FILTER_FACTOR1_R = crate::FieldReader;
#[doc = "Field `APB_SARADC_FILTER_FACTOR1` writer - Factor of saradc filter1"]
pub type APB_SARADC_FILTER_FACTOR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, FILTER_CTRL1_SPEC, 3, O>;
#[doc = "Field `APB_SARADC_FILTER_FACTOR0` reader - Factor of saradc filter0"]
pub type APB_SARADC_FILTER_FACTOR0_R = crate::FieldReader;
#[doc = "Field `APB_SARADC_FILTER_FACTOR0` writer - Factor of saradc filter0"]
pub type APB_SARADC_FILTER_FACTOR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, FILTER_CTRL1_SPEC, 3, O>;
impl R {
    #[doc = "Bits 26:28 - Factor of saradc filter1"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor1(&self) -> APB_SARADC_FILTER_FACTOR1_R {
        APB_SARADC_FILTER_FACTOR1_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Factor of saradc filter0"]
    #[inline(always)]
    pub fn apb_saradc_filter_factor0(&self) -> APB_SARADC_FILTER_FACTOR0_R {
        APB_SARADC_FILTER_FACTOR0_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL1")
            .field(
                "apb_saradc_filter_factor1",
                &format_args!("{}", self.apb_saradc_filter_factor1().bits()),
            )
            .field(
                "apb_saradc_filter_factor0",
                &format_args!("{}", self.apb_saradc_filter_factor0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 26:28 - Factor of saradc filter1"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_filter_factor1(&mut self) -> APB_SARADC_FILTER_FACTOR1_W<26> {
        APB_SARADC_FILTER_FACTOR1_W::new(self)
    }
    #[doc = "Bits 29:31 - Factor of saradc filter0"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_filter_factor0(&mut self) -> APB_SARADC_FILTER_FACTOR0_W<29> {
        APB_SARADC_FILTER_FACTOR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_ctrl1](index.html) module"]
pub struct FILTER_CTRL1_SPEC;
impl crate::RegisterSpec for FILTER_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_ctrl1::R](R) reader structure"]
impl crate::Readable for FILTER_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_ctrl1::W](W) writer structure"]
impl crate::Writable for FILTER_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_CTRL1 to value 0"]
impl crate::Resettable for FILTER_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
