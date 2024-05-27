///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `CNT_THR_EVENT_U(0-3)` writer - Set this bit to clear the PCNT_CNT_THR_EVENT_U%s_INT interrupt.
pub type CNT_THR_EVENT_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set this bit to clear the PCNT_CNT_THR_EVENT_U(0-3)_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u(&mut self, n: u8) -> CNT_THR_EVENT_U_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_THR_EVENT_U_W::new(self, n)
    }
    ///Bit 0 - Set this bit to clear the PCNT_CNT_THR_EVENT_U0_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u0(&mut self) -> CNT_THR_EVENT_U_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the PCNT_CNT_THR_EVENT_U1_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u1(&mut self) -> CNT_THR_EVENT_U_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the PCNT_CNT_THR_EVENT_U2_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u2(&mut self) -> CNT_THR_EVENT_U_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the PCNT_CNT_THR_EVENT_U3_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u3(&mut self) -> CNT_THR_EVENT_U_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 3)
    }
}
/**Interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
