#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 0 stops."]
pub type TIMER0_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_STOP_INT_RAW` writer - The raw status bit for the interrupt triggered when the timer 0 stops."]
pub type TIMER0_STOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER1_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 1 stops."]
pub type TIMER1_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP_INT_RAW` writer - The raw status bit for the interrupt triggered when the timer 1 stops."]
pub type TIMER1_STOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER2_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 2 stops."]
pub type TIMER2_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP_INT_RAW` writer - The raw status bit for the interrupt triggered when the timer 2 stops."]
pub type TIMER2_STOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER0_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
pub type TIMER0_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
pub type TIMER0_TEZ_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER1_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
pub type TIMER1_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
pub type TIMER1_TEZ_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER2_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
pub type TIMER2_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
pub type TIMER2_TEZ_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER0_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 0 TEP event."]
pub type TIMER0_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 0 TEP event."]
pub type TIMER0_TEP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER1_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 1 TEP event."]
pub type TIMER1_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 1 TEP event."]
pub type TIMER1_TEP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TIMER2_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 2 TEP event."]
pub type TIMER2_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM timer 2 TEP event."]
pub type TIMER2_TEP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT0_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f0 starts."]
pub type FAULT0_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT0_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f0 starts."]
pub type FAULT0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT1_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f1 starts."]
pub type FAULT1_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT1_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f1 starts."]
pub type FAULT1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT2_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f2 starts."]
pub type FAULT2_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT2_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f2 starts."]
pub type FAULT2_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT0_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f0 ends."]
pub type FAULT0_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT0_CLR_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f0 ends."]
pub type FAULT0_CLR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT1_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f1 ends."]
pub type FAULT1_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT1_CLR_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f1 ends."]
pub type FAULT1_CLR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `FAULT2_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f2 ends."]
pub type FAULT2_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT2_CLR_INT_RAW` writer - The raw status bit for the interrupt triggered when event_f2 ends."]
pub type FAULT2_CLR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR0_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 0 TEA event"]
pub type CMPR0_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR0_TEA_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 0 TEA event"]
pub type CMPR0_TEA_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR1_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 1 TEA event"]
pub type CMPR1_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR1_TEA_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 1 TEA event"]
pub type CMPR1_TEA_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR2_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 2 TEA event"]
pub type CMPR2_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR2_TEA_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 2 TEA event"]
pub type CMPR2_TEA_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR0_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 0 TEB event"]
pub type CMPR0_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR0_TEB_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 0 TEB event"]
pub type CMPR0_TEB_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR1_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 1 TEB event"]
pub type CMPR1_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR1_TEB_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 1 TEB event"]
pub type CMPR1_TEB_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CMPR2_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 2 TEB event"]
pub type CMPR2_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMPR2_TEB_INT_RAW` writer - The raw status bit for the interrupt triggered by a PWM operator 2 TEB event"]
pub type CMPR2_TEB_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ0_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type TZ0_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ0_CBC_INT_RAW` writer - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type TZ0_CBC_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ1_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type TZ1_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ1_CBC_INT_RAW` writer - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type TZ1_CBC_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ2_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type TZ2_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ2_CBC_INT_RAW` writer - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type TZ2_CBC_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ0_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM0."]
pub type TZ0_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ0_OST_INT_RAW` writer - The raw status bit for the interrupt triggered by a one-shot mode action on PWM0."]
pub type TZ0_OST_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ1_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM1."]
pub type TZ1_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ1_OST_INT_RAW` writer - The raw status bit for the interrupt triggered by a one-shot mode action on PWM1."]
pub type TZ1_OST_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TZ2_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM2."]
pub type TZ2_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `TZ2_OST_INT_RAW` writer - The raw status bit for the interrupt triggered by a one-shot mode action on PWM2."]
pub type TZ2_OST_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CAP0_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 0."]
pub type CAP0_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP0_INT_RAW` writer - The raw status bit for the interrupt triggered by capture on channel 0."]
pub type CAP0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CAP1_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 1."]
pub type CAP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP1_INT_RAW` writer - The raw status bit for the interrupt triggered by capture on channel 1."]
pub type CAP1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CAP2_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 2."]
pub type CAP2_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP2_INT_RAW` writer - The raw status bit for the interrupt triggered by capture on channel 2."]
pub type CAP2_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The raw status bit for the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop_int_raw(&self) -> TIMER0_STOP_INT_RAW_R {
        TIMER0_STOP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw status bit for the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop_int_raw(&self) -> TIMER1_STOP_INT_RAW_R {
        TIMER1_STOP_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw status bit for the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop_int_raw(&self) -> TIMER2_STOP_INT_RAW_R {
        TIMER2_STOP_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez_int_raw(&self) -> TIMER0_TEZ_INT_RAW_R {
        TIMER0_TEZ_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez_int_raw(&self) -> TIMER1_TEZ_INT_RAW_R {
        TIMER1_TEZ_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez_int_raw(&self) -> TIMER2_TEZ_INT_RAW_R {
        TIMER2_TEZ_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw status bit for the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep_int_raw(&self) -> TIMER0_TEP_INT_RAW_R {
        TIMER0_TEP_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw status bit for the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep_int_raw(&self) -> TIMER1_TEP_INT_RAW_R {
        TIMER1_TEP_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw status bit for the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep_int_raw(&self) -> TIMER2_TEP_INT_RAW_R {
        TIMER2_TEP_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw status bit for the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0_int_raw(&self) -> FAULT0_INT_RAW_R {
        FAULT0_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw status bit for the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1_int_raw(&self) -> FAULT1_INT_RAW_R {
        FAULT1_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw status bit for the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2_int_raw(&self) -> FAULT2_INT_RAW_R {
        FAULT2_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw status bit for the interrupt triggered when event_f0 ends."]
    #[inline(always)]
    pub fn fault0_clr_int_raw(&self) -> FAULT0_CLR_INT_RAW_R {
        FAULT0_CLR_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw status bit for the interrupt triggered when event_f1 ends."]
    #[inline(always)]
    pub fn fault1_clr_int_raw(&self) -> FAULT1_CLR_INT_RAW_R {
        FAULT1_CLR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw status bit for the interrupt triggered when event_f2 ends."]
    #[inline(always)]
    pub fn fault2_clr_int_raw(&self) -> FAULT2_CLR_INT_RAW_R {
        FAULT2_CLR_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw status bit for the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea_int_raw(&self) -> CMPR0_TEA_INT_RAW_R {
        CMPR0_TEA_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw status bit for the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea_int_raw(&self) -> CMPR1_TEA_INT_RAW_R {
        CMPR1_TEA_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw status bit for the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea_int_raw(&self) -> CMPR2_TEA_INT_RAW_R {
        CMPR2_TEA_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw status bit for the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb_int_raw(&self) -> CMPR0_TEB_INT_RAW_R {
        CMPR0_TEB_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw status bit for the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb_int_raw(&self) -> CMPR1_TEB_INT_RAW_R {
        CMPR1_TEB_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw status bit for the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb_int_raw(&self) -> CMPR2_TEB_INT_RAW_R {
        CMPR2_TEB_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc_int_raw(&self) -> TZ0_CBC_INT_RAW_R {
        TZ0_CBC_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc_int_raw(&self) -> TZ1_CBC_INT_RAW_R {
        TZ1_CBC_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc_int_raw(&self) -> TZ2_CBC_INT_RAW_R {
        TZ2_CBC_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost_int_raw(&self) -> TZ0_OST_INT_RAW_R {
        TZ0_OST_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost_int_raw(&self) -> TZ1_OST_INT_RAW_R {
        TZ1_OST_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost_int_raw(&self) -> TZ2_OST_INT_RAW_R {
        TZ2_OST_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The raw status bit for the interrupt triggered by capture on channel 0."]
    #[inline(always)]
    pub fn cap0_int_raw(&self) -> CAP0_INT_RAW_R {
        CAP0_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The raw status bit for the interrupt triggered by capture on channel 1."]
    #[inline(always)]
    pub fn cap1_int_raw(&self) -> CAP1_INT_RAW_R {
        CAP1_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The raw status bit for the interrupt triggered by capture on channel 2."]
    #[inline(always)]
    pub fn cap2_int_raw(&self) -> CAP2_INT_RAW_R {
        CAP2_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "timer0_stop_int_raw",
                &format_args!("{}", self.timer0_stop_int_raw().bit()),
            )
            .field(
                "timer1_stop_int_raw",
                &format_args!("{}", self.timer1_stop_int_raw().bit()),
            )
            .field(
                "timer2_stop_int_raw",
                &format_args!("{}", self.timer2_stop_int_raw().bit()),
            )
            .field(
                "timer0_tez_int_raw",
                &format_args!("{}", self.timer0_tez_int_raw().bit()),
            )
            .field(
                "timer1_tez_int_raw",
                &format_args!("{}", self.timer1_tez_int_raw().bit()),
            )
            .field(
                "timer2_tez_int_raw",
                &format_args!("{}", self.timer2_tez_int_raw().bit()),
            )
            .field(
                "timer0_tep_int_raw",
                &format_args!("{}", self.timer0_tep_int_raw().bit()),
            )
            .field(
                "timer1_tep_int_raw",
                &format_args!("{}", self.timer1_tep_int_raw().bit()),
            )
            .field(
                "timer2_tep_int_raw",
                &format_args!("{}", self.timer2_tep_int_raw().bit()),
            )
            .field(
                "fault0_int_raw",
                &format_args!("{}", self.fault0_int_raw().bit()),
            )
            .field(
                "fault1_int_raw",
                &format_args!("{}", self.fault1_int_raw().bit()),
            )
            .field(
                "fault2_int_raw",
                &format_args!("{}", self.fault2_int_raw().bit()),
            )
            .field(
                "fault0_clr_int_raw",
                &format_args!("{}", self.fault0_clr_int_raw().bit()),
            )
            .field(
                "fault1_clr_int_raw",
                &format_args!("{}", self.fault1_clr_int_raw().bit()),
            )
            .field(
                "fault2_clr_int_raw",
                &format_args!("{}", self.fault2_clr_int_raw().bit()),
            )
            .field(
                "cmpr0_tea_int_raw",
                &format_args!("{}", self.cmpr0_tea_int_raw().bit()),
            )
            .field(
                "cmpr1_tea_int_raw",
                &format_args!("{}", self.cmpr1_tea_int_raw().bit()),
            )
            .field(
                "cmpr2_tea_int_raw",
                &format_args!("{}", self.cmpr2_tea_int_raw().bit()),
            )
            .field(
                "cmpr0_teb_int_raw",
                &format_args!("{}", self.cmpr0_teb_int_raw().bit()),
            )
            .field(
                "cmpr1_teb_int_raw",
                &format_args!("{}", self.cmpr1_teb_int_raw().bit()),
            )
            .field(
                "cmpr2_teb_int_raw",
                &format_args!("{}", self.cmpr2_teb_int_raw().bit()),
            )
            .field(
                "tz0_cbc_int_raw",
                &format_args!("{}", self.tz0_cbc_int_raw().bit()),
            )
            .field(
                "tz1_cbc_int_raw",
                &format_args!("{}", self.tz1_cbc_int_raw().bit()),
            )
            .field(
                "tz2_cbc_int_raw",
                &format_args!("{}", self.tz2_cbc_int_raw().bit()),
            )
            .field(
                "tz0_ost_int_raw",
                &format_args!("{}", self.tz0_ost_int_raw().bit()),
            )
            .field(
                "tz1_ost_int_raw",
                &format_args!("{}", self.tz1_ost_int_raw().bit()),
            )
            .field(
                "tz2_ost_int_raw",
                &format_args!("{}", self.tz2_ost_int_raw().bit()),
            )
            .field(
                "cap0_int_raw",
                &format_args!("{}", self.cap0_int_raw().bit()),
            )
            .field(
                "cap1_int_raw",
                &format_args!("{}", self.cap1_int_raw().bit()),
            )
            .field(
                "cap2_int_raw",
                &format_args!("{}", self.cap2_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw status bit for the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_stop_int_raw(&mut self) -> TIMER0_STOP_INT_RAW_W<0> {
        TIMER0_STOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The raw status bit for the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop_int_raw(&mut self) -> TIMER1_STOP_INT_RAW_W<1> {
        TIMER1_STOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The raw status bit for the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop_int_raw(&mut self) -> TIMER2_STOP_INT_RAW_W<2> {
        TIMER2_STOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The raw status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez_int_raw(&mut self) -> TIMER0_TEZ_INT_RAW_W<3> {
        TIMER0_TEZ_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The raw status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez_int_raw(&mut self) -> TIMER1_TEZ_INT_RAW_W<4> {
        TIMER1_TEZ_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - The raw status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez_int_raw(&mut self) -> TIMER2_TEZ_INT_RAW_W<5> {
        TIMER2_TEZ_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - The raw status bit for the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep_int_raw(&mut self) -> TIMER0_TEP_INT_RAW_W<6> {
        TIMER0_TEP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - The raw status bit for the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep_int_raw(&mut self) -> TIMER1_TEP_INT_RAW_W<7> {
        TIMER1_TEP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - The raw status bit for the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep_int_raw(&mut self) -> TIMER2_TEP_INT_RAW_W<8> {
        TIMER2_TEP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - The raw status bit for the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault0_int_raw(&mut self) -> FAULT0_INT_RAW_W<9> {
        FAULT0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The raw status bit for the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault1_int_raw(&mut self) -> FAULT1_INT_RAW_W<10> {
        FAULT1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - The raw status bit for the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    #[must_use]
    pub fn fault2_int_raw(&mut self) -> FAULT2_INT_RAW_W<11> {
        FAULT2_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The raw status bit for the interrupt triggered when event_f0 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr_int_raw(&mut self) -> FAULT0_CLR_INT_RAW_W<12> {
        FAULT0_CLR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - The raw status bit for the interrupt triggered when event_f1 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr_int_raw(&mut self) -> FAULT1_CLR_INT_RAW_W<13> {
        FAULT1_CLR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 14 - The raw status bit for the interrupt triggered when event_f2 ends."]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr_int_raw(&mut self) -> FAULT2_CLR_INT_RAW_W<14> {
        FAULT2_CLR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - The raw status bit for the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_tea_int_raw(&mut self) -> CMPR0_TEA_INT_RAW_W<15> {
        CMPR0_TEA_INT_RAW_W::new(self)
    }
    #[doc = "Bit 16 - The raw status bit for the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_tea_int_raw(&mut self) -> CMPR1_TEA_INT_RAW_W<16> {
        CMPR1_TEA_INT_RAW_W::new(self)
    }
    #[doc = "Bit 17 - The raw status bit for the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_tea_int_raw(&mut self) -> CMPR2_TEA_INT_RAW_W<17> {
        CMPR2_TEA_INT_RAW_W::new(self)
    }
    #[doc = "Bit 18 - The raw status bit for the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_teb_int_raw(&mut self) -> CMPR0_TEB_INT_RAW_W<18> {
        CMPR0_TEB_INT_RAW_W::new(self)
    }
    #[doc = "Bit 19 - The raw status bit for the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_teb_int_raw(&mut self) -> CMPR1_TEB_INT_RAW_W<19> {
        CMPR1_TEB_INT_RAW_W::new(self)
    }
    #[doc = "Bit 20 - The raw status bit for the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_teb_int_raw(&mut self) -> CMPR2_TEB_INT_RAW_W<20> {
        CMPR2_TEB_INT_RAW_W::new(self)
    }
    #[doc = "Bit 21 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn tz0_cbc_int_raw(&mut self) -> TZ0_CBC_INT_RAW_W<21> {
        TZ0_CBC_INT_RAW_W::new(self)
    }
    #[doc = "Bit 22 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn tz1_cbc_int_raw(&mut self) -> TZ1_CBC_INT_RAW_W<22> {
        TZ1_CBC_INT_RAW_W::new(self)
    }
    #[doc = "Bit 23 - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    #[must_use]
    pub fn tz2_cbc_int_raw(&mut self) -> TZ2_CBC_INT_RAW_W<23> {
        TZ2_CBC_INT_RAW_W::new(self)
    }
    #[doc = "Bit 24 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn tz0_ost_int_raw(&mut self) -> TZ0_OST_INT_RAW_W<24> {
        TZ0_OST_INT_RAW_W::new(self)
    }
    #[doc = "Bit 25 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn tz1_ost_int_raw(&mut self) -> TZ1_OST_INT_RAW_W<25> {
        TZ1_OST_INT_RAW_W::new(self)
    }
    #[doc = "Bit 26 - The raw status bit for the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    #[must_use]
    pub fn tz2_ost_int_raw(&mut self) -> TZ2_OST_INT_RAW_W<26> {
        TZ2_OST_INT_RAW_W::new(self)
    }
    #[doc = "Bit 27 - The raw status bit for the interrupt triggered by capture on channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn cap0_int_raw(&mut self) -> CAP0_INT_RAW_W<27> {
        CAP0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - The raw status bit for the interrupt triggered by capture on channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_int_raw(&mut self) -> CAP1_INT_RAW_W<28> {
        CAP1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - The raw status bit for the interrupt triggered by capture on channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_int_raw(&mut self) -> CAP2_INT_RAW_W<29> {
        CAP2_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
