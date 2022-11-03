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
#[doc = "Field `TIMER0_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 0 stops."]
pub type TIMER0_STOP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 1 stops."]
pub type TIMER1_STOP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_STOP_INT_RAW` reader - The raw status bit for the interrupt triggered when the timer 2 stops."]
pub type TIMER2_STOP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 0 TEZ event."]
pub type TIMER0_TEZ_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 1 TEZ event."]
pub type TIMER1_TEZ_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_TEZ_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 2 TEZ event."]
pub type TIMER2_TEZ_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 0 TEP event."]
pub type TIMER0_TEP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 1 TEP event."]
pub type TIMER1_TEP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_TEP_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM timer 2 TEP event."]
pub type TIMER2_TEP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f0 starts."]
pub type FAULT0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f1 starts."]
pub type FAULT1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT2_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f2 starts."]
pub type FAULT2_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f0 ends."]
pub type FAULT0_CLR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f1 ends."]
pub type FAULT1_CLR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `FAULT2_CLR_INT_RAW` reader - The raw status bit for the interrupt triggered when event_f2 ends."]
pub type FAULT2_CLR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR0_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 0 TEA event"]
pub type CMPR0_TEA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR1_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 1 TEA event"]
pub type CMPR1_TEA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR2_TEA_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 2 TEA event"]
pub type CMPR2_TEA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR0_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 0 TEB event"]
pub type CMPR0_TEB_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR1_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 1 TEB event"]
pub type CMPR1_TEB_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CMPR2_TEB_INT_RAW` reader - The raw status bit for the interrupt triggered by a PWM operator 2 TEB event"]
pub type CMPR2_TEB_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ0_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type TZ0_CBC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type TZ1_CBC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_CBC_INT_RAW` reader - The raw status bit for the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type TZ2_CBC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ0_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM0."]
pub type TZ0_OST_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM1."]
pub type TZ1_OST_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_OST_INT_RAW` reader - The raw status bit for the interrupt triggered by a one-shot mode action on PWM2."]
pub type TZ2_OST_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CAP0_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 0."]
pub type CAP0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CAP1_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 1."]
pub type CAP1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CAP2_INT_RAW` reader - The raw status bit for the interrupt triggered by capture on channel 2."]
pub type CAP2_INT_RAW_R = crate::BitReader<bool>;
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
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
