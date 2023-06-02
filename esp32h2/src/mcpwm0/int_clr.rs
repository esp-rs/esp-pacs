#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 0 stops."]
pub type TIMER0_STOP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER1_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 1 stops."]
pub type TIMER1_STOP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER2_STOP_INT_CLR` writer - Set this bit to clear the interrupt triggered when the timer 2 stops."]
pub type TIMER2_STOP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER0_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 0 TEZ event."]
pub type TIMER0_TEZ_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER1_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 1 TEZ event."]
pub type TIMER1_TEZ_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER2_TEZ_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 2 TEZ event."]
pub type TIMER2_TEZ_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER0_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 0 TEP event."]
pub type TIMER0_TEP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER1_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 1 TEP event."]
pub type TIMER1_TEP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIMER2_TEP_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM timer 2 TEP event."]
pub type TIMER2_TEP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT0_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f0 starts."]
pub type FAULT0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT1_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f1 starts."]
pub type FAULT1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT2_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f2 starts."]
pub type FAULT2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT0_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f0 ends."]
pub type FAULT0_CLR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT1_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f1 ends."]
pub type FAULT1_CLR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `FAULT2_CLR_INT_CLR` writer - Set this bit to clear the interrupt triggered when event_f2 ends."]
pub type FAULT2_CLR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR0_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 0 TEA event"]
pub type CMPR0_TEA_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR1_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 1 TEA event"]
pub type CMPR1_TEA_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR2_TEA_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 2 TEA event"]
pub type CMPR2_TEA_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR0_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 0 TEB event"]
pub type CMPR0_TEB_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR1_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 1 TEB event"]
pub type CMPR1_TEB_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CMPR2_TEB_INT_CLR` writer - Set this bit to clear the interrupt triggered by a PWM operator 2 TEB event"]
pub type CMPR2_TEB_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ0_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type TZ0_CBC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ1_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type TZ1_CBC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ2_CBC_INT_CLR` writer - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type TZ2_CBC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ0_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM0."]
pub type TZ0_OST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ1_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM1."]
pub type TZ1_OST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TZ2_OST_INT_CLR` writer - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM2."]
pub type TZ2_OST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CAP0_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 0."]
pub type CAP0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CAP1_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 1."]
pub type CAP1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CAP2_INT_CLR` writer - Set this bit to clear the interrupt triggered by capture on channel 2."]
pub type CAP2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_stop_int_clr(&mut self) -> TIMER0_STOP_INT_CLR_W<0> {
        TIMER0_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop_int_clr(&mut self) -> TIMER1_STOP_INT_CLR_W<1> {
        TIMER1_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop_int_clr(&mut self) -> TIMER2_STOP_INT_CLR_W<2> {
        TIMER2_STOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez_int_clr(&mut self) -> TIMER0_TEZ_INT_CLR_W<3> {
        TIMER0_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez_int_clr(&mut self) -> TIMER1_TEZ_INT_CLR_W<4> {
        TIMER1_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez_int_clr(&mut self) -> TIMER2_TEZ_INT_CLR_W<5> {
        TIMER2_TEZ_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep_int_clr(&mut self) -> TIMER0_TEP_INT_CLR_W<6> {
        TIMER0_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep_int_clr(&mut self) -> TIMER1_TEP_INT_CLR_W<7> {
        TIMER1_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep_int_clr(&mut self) -> TIMER2_TEP_INT_CLR_W<8> {
        TIMER2_TEP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault0_int_clr(&mut self) -> FAULT0_INT_CLR_W<9> {
        FAULT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault1_int_clr(&mut self) -> FAULT1_INT_CLR_W<10> {
        FAULT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault2_int_clr(&mut self) -> FAULT2_INT_CLR_W<11> {
        FAULT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the interrupt triggered when event_f0 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr_int_clr(&mut self) -> FAULT0_CLR_INT_CLR_W<12> {
        FAULT0_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the interrupt triggered when event_f1 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr_int_clr(&mut self) -> FAULT1_CLR_INT_CLR_W<13> {
        FAULT1_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the interrupt triggered when event_f2 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr_int_clr(&mut self) -> FAULT2_CLR_INT_CLR_W<14> {
        FAULT2_CLR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_tea_int_clr(&mut self) -> CMPR0_TEA_INT_CLR_W<15> {
        CMPR0_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_tea_int_clr(&mut self) -> CMPR1_TEA_INT_CLR_W<16> {
        CMPR1_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_tea_int_clr(&mut self) -> CMPR2_TEA_INT_CLR_W<17> {
        CMPR2_TEA_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_teb_int_clr(&mut self) -> CMPR0_TEB_INT_CLR_W<18> {
        CMPR0_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_teb_int_clr(&mut self) -> CMPR1_TEB_INT_CLR_W<19> {
        CMPR1_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to clear the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_teb_int_clr(&mut self) -> CMPR2_TEB_INT_CLR_W<20> {
        CMPR2_TEB_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn tz0_cbc_int_clr(&mut self) -> TZ0_CBC_INT_CLR_W<21> {
        TZ0_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn tz1_cbc_int_clr(&mut self) -> TZ1_CBC_INT_CLR_W<22> {
        TZ1_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to clear the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    #[must_use]
    pub fn tz2_cbc_int_clr(&mut self) -> TZ2_CBC_INT_CLR_W<23> {
        TZ2_CBC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn tz0_ost_int_clr(&mut self) -> TZ0_OST_INT_CLR_W<24> {
        TZ0_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn tz1_ost_int_clr(&mut self) -> TZ1_OST_INT_CLR_W<25> {
        TZ1_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to clear the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    #[must_use]
    pub fn tz2_ost_int_clr(&mut self) -> TZ2_OST_INT_CLR_W<26> {
        TZ2_OST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to clear the interrupt triggered by capture on channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn cap0_int_clr(&mut self) -> CAP0_INT_CLR_W<27> {
        CAP0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to clear the interrupt triggered by capture on channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_int_clr(&mut self) -> CAP1_INT_CLR_W<28> {
        CAP1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to clear the interrupt triggered by capture on channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_int_clr(&mut self) -> CAP2_INT_CLR_W<29> {
        CAP2_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
