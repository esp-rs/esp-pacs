#[doc = "Register `SLP_WAKEUP_CNTL3` reader"]
pub struct R(crate::R<SLP_WAKEUP_CNTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CNTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CNTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CNTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKEUP_CNTL3` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL3_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_MIN_SLP_VAL` reader - need_des"]
pub type LP_MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `LP_MIN_SLP_VAL` writer - need_des"]
pub type LP_MIN_SLP_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, SLP_WAKEUP_CNTL3_SPEC, 8, O>;
#[doc = "Field `HP_MIN_SLP_VAL` reader - need_des"]
pub type HP_MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `HP_MIN_SLP_VAL` writer - need_des"]
pub type HP_MIN_SLP_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, SLP_WAKEUP_CNTL3_SPEC, 8, O>;
#[doc = "Field `SLEEP_PRT_SEL` reader - need_des"]
pub type SLEEP_PRT_SEL_R = crate::FieldReader;
#[doc = "Field `SLEEP_PRT_SEL` writer - need_des"]
pub type SLEEP_PRT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SLP_WAKEUP_CNTL3_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_min_slp_val(&self) -> LP_MIN_SLP_VAL_R {
        LP_MIN_SLP_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn hp_min_slp_val(&self) -> HP_MIN_SLP_VAL_R {
        HP_MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn sleep_prt_sel(&self) -> SLEEP_PRT_SEL_R {
        SLEEP_PRT_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL3")
            .field(
                "lp_min_slp_val",
                &format_args!("{}", self.lp_min_slp_val().bits()),
            )
            .field(
                "hp_min_slp_val",
                &format_args!("{}", self.hp_min_slp_val().bits()),
            )
            .field(
                "sleep_prt_sel",
                &format_args!("{}", self.sleep_prt_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_min_slp_val(&mut self) -> LP_MIN_SLP_VAL_W<0> {
        LP_MIN_SLP_VAL_W::new(self)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_min_slp_val(&mut self) -> HP_MIN_SLP_VAL_W<8> {
        HP_MIN_SLP_VAL_W::new(self)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_prt_sel(&mut self) -> SLEEP_PRT_SEL_W<16> {
        SLEEP_PRT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl3](index.html) module"]
pub struct SLP_WAKEUP_CNTL3_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cntl3::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl3::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL3 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
