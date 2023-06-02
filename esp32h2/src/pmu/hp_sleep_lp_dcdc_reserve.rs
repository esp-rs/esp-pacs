#[doc = "Register `HP_SLEEP_LP_DCDC_RESERVE` writer"]
pub struct W(crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>;
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
impl From<crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_SLEEP_LP_DCDC_RESERVE` writer - need_des"]
pub type HP_SLEEP_LP_DCDC_RESERVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_LP_DCDC_RESERVE_SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_LP_DCDC_RESERVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_lp_dcdc_reserve(&mut self) -> HP_SLEEP_LP_DCDC_RESERVE_W<0> {
        HP_SLEEP_LP_DCDC_RESERVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_sleep_lp_dcdc_reserve](index.html) module"]
pub struct HP_SLEEP_LP_DCDC_RESERVE_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hp_sleep_lp_dcdc_reserve::W](W) writer structure"]
impl crate::Writable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_DCDC_RESERVE to value 0"]
impl crate::Resettable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
