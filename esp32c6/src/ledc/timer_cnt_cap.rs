#[doc = "Register `TIMER%s_CNT_CAP` reader"]
pub struct R(crate::R<TIMER_CNT_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CNT_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CNT_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CNT_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_CNT_CAP` reader - This register stores ledc timer%s count value."]
pub type TIMER_CNT_CAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores ledc timer%s count value."]
    #[inline(always)]
    pub fn timer_cnt_cap(&self) -> TIMER_CNT_CAP_R {
        TIMER_CNT_CAP_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CNT_CAP")
            .field(
                "timer_cnt_cap",
                &format_args!("{}", self.timer_cnt_cap().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_CNT_CAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Ledc timer%s count value capture register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_cnt_cap](index.html) module"]
pub struct TIMER_CNT_CAP_SPEC;
impl crate::RegisterSpec for TIMER_CNT_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_cnt_cap::R](R) reader structure"]
impl crate::Readable for TIMER_CNT_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER%s_CNT_CAP to value 0"]
impl crate::Resettable for TIMER_CNT_CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
