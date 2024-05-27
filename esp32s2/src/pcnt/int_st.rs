///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `CNT_THR_EVENT_U(0-3)` reader - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U%s_INT interrupt.
pub type CNT_THR_EVENT_U_R = crate::BitReader;
impl R {
    ///The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field
    #[inline(always)]
    pub fn cnt_thr_event_u(&self, n: u8) -> CNT_THR_EVENT_U_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u_iter(&self) -> impl Iterator<Item = CNT_THR_EVENT_U_R> + '_ {
        (0..4).map(move |n| CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The masked interrupt status bit for the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("cnt_thr_event_u0", &self.cnt_thr_event_u0())
            .field("cnt_thr_event_u1", &self.cnt_thr_event_u1())
            .field("cnt_thr_event_u2", &self.cnt_thr_event_u2())
            .field("cnt_thr_event_u3", &self.cnt_thr_event_u3())
            .finish()
    }
}
/**Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
