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
#[doc = "Field `CNT_THR_EVENT_U0` reader - This is the interrupt status bit for channel0 event."]
pub type CNT_THR_EVENT_U0_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U1` reader - This is the interrupt status bit for channel1 event."]
pub type CNT_THR_EVENT_U1_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U2` reader - This is the interrupt status bit for channel2 event."]
pub type CNT_THR_EVENT_U2_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U3` reader - This is the interrupt status bit for channel3 event."]
pub type CNT_THR_EVENT_U3_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U4` reader - This is the interrupt status bit for channel4 event."]
pub type CNT_THR_EVENT_U4_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U5` reader - This is the interrupt status bit for channel5 event."]
pub type CNT_THR_EVENT_U5_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U6` reader - This is the interrupt status bit for channel6 event."]
pub type CNT_THR_EVENT_U6_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U7` reader - This is the interrupt status bit for channel7 event."]
pub type CNT_THR_EVENT_U7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the interrupt status bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U0_R {
        CNT_THR_EVENT_U0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt status bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U1_R {
        CNT_THR_EVENT_U1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt status bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U2_R {
        CNT_THR_EVENT_U2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt status bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U3_R {
        CNT_THR_EVENT_U3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt status bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4(&self) -> CNT_THR_EVENT_U4_R {
        CNT_THR_EVENT_U4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt status bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5(&self) -> CNT_THR_EVENT_U5_R {
        CNT_THR_EVENT_U5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt status bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6(&self) -> CNT_THR_EVENT_U6_R {
        CNT_THR_EVENT_U6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt status bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7(&self) -> CNT_THR_EVENT_U7_R {
        CNT_THR_EVENT_U7_R::new(((self.bits >> 7) & 1) != 0)
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
            .field(
                "cnt_thr_event_u4",
                &format_args!("{}", self.cnt_thr_event_u4().bit()),
            )
            .field(
                "cnt_thr_event_u5",
                &format_args!("{}", self.cnt_thr_event_u5().bit()),
            )
            .field(
                "cnt_thr_event_u6",
                &format_args!("{}", self.cnt_thr_event_u6().bit()),
            )
            .field(
                "cnt_thr_event_u7",
                &format_args!("{}", self.cnt_thr_event_u7().bit()),
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
