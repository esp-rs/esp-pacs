#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT_THR_EVENT_U0` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
pub type CNT_THR_EVENT_U0_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U1` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
pub type CNT_THR_EVENT_U1_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U2` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
pub type CNT_THR_EVENT_U2_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U3` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
pub type CNT_THR_EVENT_U3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U0_R {
        CNT_THR_EVENT_U0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U1_R {
        CNT_THR_EVENT_U1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U2_R {
        CNT_THR_EVENT_U2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U3_R {
        CNT_THR_EVENT_U3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "cnt_thr_event_u0",
                &format_args!("{}", self.cnt_thr_event_u0().bit()),
            )
            .field(
                "cnt_thr_event_u1",
                &format_args!("{}", self.cnt_thr_event_u1().bit()),
            )
            .field(
                "cnt_thr_event_u2",
                &format_args!("{}", self.cnt_thr_event_u2().bit()),
            )
            .field(
                "cnt_thr_event_u3",
                &format_args!("{}", self.cnt_thr_event_u3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
