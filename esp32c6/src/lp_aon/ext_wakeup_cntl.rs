#[doc = "Register `EXT_WAKEUP_CNTL` reader"]
pub struct R(crate::R<EXT_WAKEUP_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP_CNTL` writer"]
pub struct W(crate::W<EXT_WAKEUP_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP_CNTL_SPEC>;
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
impl From<crate::W<EXT_WAKEUP_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_WAKEUP_STATUS` reader - need_des"]
pub type EXT_WAKEUP_STATUS_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_STATUS_CLR` writer - need_des"]
pub type EXT_WAKEUP_STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP_CNTL_SPEC, O>;
#[doc = "Field `EXT_WAKEUP_SEL` reader - need_des"]
pub type EXT_WAKEUP_SEL_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_SEL` writer - need_des"]
pub type EXT_WAKEUP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, EXT_WAKEUP_CNTL_SPEC, 8, O>;
#[doc = "Field `EXT_WAKEUP_LV` reader - need_des"]
pub type EXT_WAKEUP_LV_R = crate::FieldReader;
#[doc = "Field `EXT_WAKEUP_LV` writer - need_des"]
pub type EXT_WAKEUP_LV_W<'a, const O: u8> = crate::FieldWriter<'a, EXT_WAKEUP_CNTL_SPEC, 8, O>;
#[doc = "Field `EXT_WAKEUP_FILTER` reader - need_des"]
pub type EXT_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP_FILTER` writer - need_des"]
pub type EXT_WAKEUP_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP_CNTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_status(&self) -> EXT_WAKEUP_STATUS_R {
        EXT_WAKEUP_STATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_sel(&self) -> EXT_WAKEUP_SEL_R {
        EXT_WAKEUP_SEL_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&self) -> EXT_WAKEUP_LV_R {
        EXT_WAKEUP_LV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&self) -> EXT_WAKEUP_FILTER_R {
        EXT_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CNTL")
            .field(
                "ext_wakeup_status",
                &format_args!("{}", self.ext_wakeup_status().bits()),
            )
            .field(
                "ext_wakeup_sel",
                &format_args!("{}", self.ext_wakeup_sel().bits()),
            )
            .field(
                "ext_wakeup_lv",
                &format_args!("{}", self.ext_wakeup_lv().bits()),
            )
            .field(
                "ext_wakeup_filter",
                &format_args!("{}", self.ext_wakeup_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_status_clr(&mut self) -> EXT_WAKEUP_STATUS_CLR_W<14> {
        EXT_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_sel(&mut self) -> EXT_WAKEUP_SEL_W<15> {
        EXT_WAKEUP_SEL_W::new(self)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_lv(&mut self) -> EXT_WAKEUP_LV_W<23> {
        EXT_WAKEUP_LV_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_filter(&mut self) -> EXT_WAKEUP_FILTER_W<31> {
        EXT_WAKEUP_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup_cntl](index.html) module"]
pub struct EXT_WAKEUP_CNTL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup_cntl::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_cntl::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CNTL to value 0"]
impl crate::Resettable for EXT_WAKEUP_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
