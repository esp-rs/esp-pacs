#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `CNT_THR_EVENT_U0_INT_ST` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
pub type CNT_THR_EVENT_U0_INT_ST_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U1_INT_ST` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
pub type CNT_THR_EVENT_U1_INT_ST_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U2_INT_ST` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
pub type CNT_THR_EVENT_U2_INT_ST_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U3_INT_ST` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
pub type CNT_THR_EVENT_U3_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_st(&self) -> CNT_THR_EVENT_U0_INT_ST_R {
        CNT_THR_EVENT_U0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_st(&self) -> CNT_THR_EVENT_U1_INT_ST_R {
        CNT_THR_EVENT_U1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_st(&self) -> CNT_THR_EVENT_U2_INT_ST_R {
        CNT_THR_EVENT_U2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_st(&self) -> CNT_THR_EVENT_U3_INT_ST_R {
        CNT_THR_EVENT_U3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "cnt_thr_event_u0_int_st",
                &format_args!("{}", self.cnt_thr_event_u0_int_st().bit()),
            )
            .field(
                "cnt_thr_event_u1_int_st",
                &format_args!("{}", self.cnt_thr_event_u1_int_st().bit()),
            )
            .field(
                "cnt_thr_event_u2_int_st",
                &format_args!("{}", self.cnt_thr_event_u2_int_st().bit()),
            )
            .field(
                "cnt_thr_event_u3_int_st",
                &format_args!("{}", self.cnt_thr_event_u3_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
