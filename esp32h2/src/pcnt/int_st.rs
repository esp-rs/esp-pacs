#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `CNT_THR_EVENT_U(0-3)` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U%s_INT interrupt."]
pub type CNT_THR_EVENT_U_R = crate::BitReader;
impl R {
    #[doc = "The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field"]
    #[inline(always)]
    pub fn cnt_thr_event_u(&self, n: u8) -> CNT_THR_EVENT_U_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u_iter(&self) -> impl Iterator<Item = CNT_THR_EVENT_U_R> + '_ {
        (0..4).map(move |n| CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 3) & 1) != 0)
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
