#[doc = "Register `TIMER%s_VALUE` reader"]
pub struct R(crate::R<TIMER_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - reg_lstimer0_cnt."]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - reg_lstimer0_cnt."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_VALUE")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LEDC_LSTIMER%s_VALUE.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_value](index.html) module"]
pub struct TIMER_VALUE_SPEC;
impl crate::RegisterSpec for TIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_value::R](R) reader structure"]
impl crate::Readable for TIMER_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER%s_VALUE to value 0"]
impl crate::Resettable for TIMER_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
