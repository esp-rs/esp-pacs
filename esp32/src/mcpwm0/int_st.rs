#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TIMER_STOP(0-2)` reader - The interrupt status bit for the timer%s stop event."]
pub type TIMER_STOP_R = crate::BitReader;
#[doc = "Field `TIMER_TEZ(0-2)` reader - The interrupt status bit for the timer%s zero event."]
pub type TIMER_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER_TEP(0-2)` reader - The interrupt status bit for the timer%s period event."]
pub type TIMER_TEP_R = crate::BitReader;
#[doc = "Field `FAULT(0-2)` reader - The interrupt status bit for the fault%s event."]
pub type FAULT_R = crate::BitReader;
#[doc = "Field `FAULT_CLR(0-2)` reader - The interrupt status bit for the fault%s clear event."]
pub type FAULT_CLR_R = crate::BitReader;
#[doc = "Field `CMPR_TEA(0-2)` reader - The interrupt status bit for the operator%s compare A event."]
pub type CMPR_TEA_R = crate::BitReader;
#[doc = "Field `CMPR_TEB(0-2)` reader - The interrupt status bit for the operator%s compare B event."]
pub type CMPR_TEB_R = crate::BitReader;
#[doc = "Field `TZ_CBC(0-2)` reader - The interrupt status bit for the fault handler%s cycle-by-cycle mode action."]
pub type TZ_CBC_R = crate::BitReader;
#[doc = "Field `TZ_OST(0-2)` reader - The interrupt status bit for the fault handler%s one-shot mode action."]
pub type TZ_OST_R = crate::BitReader;
#[doc = "Field `CAP(0-2)` reader - The interrupt status bit for the capture%s event."]
pub type CAP_R = crate::BitReader;
impl R {
    #[doc = "The interrupt status bit for the timer(0-2) stop event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_STOP` field.</div>"]
    #[inline(always)]
    pub fn timer_stop(&self, n: u8) -> TIMER_STOP_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_STOP_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the timer(0-2) stop event."]
    #[inline(always)]
    pub fn timer_stop_iter(&self) -> impl Iterator<Item = TIMER_STOP_R> + '_ {
        (0..3).map(move |n| TIMER_STOP_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt status bit for the timer0 stop event."]
    #[inline(always)]
    pub fn timer0_stop(&self) -> TIMER_STOP_R {
        TIMER_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt status bit for the timer1 stop event."]
    #[inline(always)]
    pub fn timer1_stop(&self) -> TIMER_STOP_R {
        TIMER_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt status bit for the timer2 stop event."]
    #[inline(always)]
    pub fn timer2_stop(&self) -> TIMER_STOP_R {
        TIMER_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the timer(0-2) zero event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEZ` field.</div>"]
    #[inline(always)]
    pub fn timer_tez(&self, n: u8) -> TIMER_TEZ_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEZ_R::new(((self.bits >> (n + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the timer(0-2) zero event."]
    #[inline(always)]
    pub fn timer_tez_iter(&self) -> impl Iterator<Item = TIMER_TEZ_R> + '_ {
        (0..3).map(move |n| TIMER_TEZ_R::new(((self.bits >> (n + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - The interrupt status bit for the timer0 zero event."]
    #[inline(always)]
    pub fn timer0_tez(&self) -> TIMER_TEZ_R {
        TIMER_TEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt status bit for the timer1 zero event."]
    #[inline(always)]
    pub fn timer1_tez(&self) -> TIMER_TEZ_R {
        TIMER_TEZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt status bit for the timer2 zero event."]
    #[inline(always)]
    pub fn timer2_tez(&self) -> TIMER_TEZ_R {
        TIMER_TEZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the timer(0-2) period event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEP` field.</div>"]
    #[inline(always)]
    pub fn timer_tep(&self, n: u8) -> TIMER_TEP_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEP_R::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the timer(0-2) period event."]
    #[inline(always)]
    pub fn timer_tep_iter(&self) -> impl Iterator<Item = TIMER_TEP_R> + '_ {
        (0..3).map(move |n| TIMER_TEP_R::new(((self.bits >> (n + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - The interrupt status bit for the timer0 period event."]
    #[inline(always)]
    pub fn timer0_tep(&self) -> TIMER_TEP_R {
        TIMER_TEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt status bit for the timer1 period event."]
    #[inline(always)]
    pub fn timer1_tep(&self) -> TIMER_TEP_R {
        TIMER_TEP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt status bit for the timer2 period event."]
    #[inline(always)]
    pub fn timer2_tep(&self) -> TIMER_TEP_R {
        TIMER_TEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the fault(0-2) event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FAULT0` field.</div>"]
    #[inline(always)]
    pub fn fault(&self, n: u8) -> FAULT_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        FAULT_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the fault(0-2) event."]
    #[inline(always)]
    pub fn fault_iter(&self) -> impl Iterator<Item = FAULT_R> + '_ {
        (0..3).map(move |n| FAULT_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - The interrupt status bit for the fault0 event."]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt status bit for the fault1 event."]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt status bit for the fault2 event."]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the fault(0-2) clear event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FAULT0_CLR` field.</div>"]
    #[inline(always)]
    pub fn fault_clr(&self, n: u8) -> FAULT_CLR_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        FAULT_CLR_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the fault(0-2) clear event."]
    #[inline(always)]
    pub fn fault_clr_iter(&self) -> impl Iterator<Item = FAULT_CLR_R> + '_ {
        (0..3).map(move |n| FAULT_CLR_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - The interrupt status bit for the fault0 clear event."]
    #[inline(always)]
    pub fn fault0_clr(&self) -> FAULT_CLR_R {
        FAULT_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt status bit for the fault1 clear event."]
    #[inline(always)]
    pub fn fault1_clr(&self) -> FAULT_CLR_R {
        FAULT_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt status bit for the fault2 clear event."]
    #[inline(always)]
    pub fn fault2_clr(&self) -> FAULT_CLR_R {
        FAULT_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the operator(0-2) compare A event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_TEA` field.</div>"]
    #[inline(always)]
    pub fn cmpr_tea(&self, n: u8) -> CMPR_TEA_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_TEA_R::new(((self.bits >> (n + 15)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the operator(0-2) compare A event."]
    #[inline(always)]
    pub fn cmpr_tea_iter(&self) -> impl Iterator<Item = CMPR_TEA_R> + '_ {
        (0..3).map(move |n| CMPR_TEA_R::new(((self.bits >> (n + 15)) & 1) != 0))
    }
    #[doc = "Bit 15 - The interrupt status bit for the operator0 compare A event."]
    #[inline(always)]
    pub fn cmpr0_tea(&self) -> CMPR_TEA_R {
        CMPR_TEA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt status bit for the operator1 compare A event."]
    #[inline(always)]
    pub fn cmpr1_tea(&self) -> CMPR_TEA_R {
        CMPR_TEA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt status bit for the operator2 compare A event."]
    #[inline(always)]
    pub fn cmpr2_tea(&self) -> CMPR_TEA_R {
        CMPR_TEA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the operator(0-2) compare B event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_TEB` field.</div>"]
    #[inline(always)]
    pub fn cmpr_teb(&self, n: u8) -> CMPR_TEB_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_TEB_R::new(((self.bits >> (n + 18)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the operator(0-2) compare B event."]
    #[inline(always)]
    pub fn cmpr_teb_iter(&self) -> impl Iterator<Item = CMPR_TEB_R> + '_ {
        (0..3).map(move |n| CMPR_TEB_R::new(((self.bits >> (n + 18)) & 1) != 0))
    }
    #[doc = "Bit 18 - The interrupt status bit for the operator0 compare B event."]
    #[inline(always)]
    pub fn cmpr0_teb(&self) -> CMPR_TEB_R {
        CMPR_TEB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt status bit for the operator1 compare B event."]
    #[inline(always)]
    pub fn cmpr1_teb(&self) -> CMPR_TEB_R {
        CMPR_TEB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt status bit for the operator2 compare B event."]
    #[inline(always)]
    pub fn cmpr2_teb(&self) -> CMPR_TEB_R {
        CMPR_TEB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the fault handler(0-2) cycle-by-cycle mode action."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_CBC` field.</div>"]
    #[inline(always)]
    pub fn tz_cbc(&self, n: u8) -> TZ_CBC_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_CBC_R::new(((self.bits >> (n + 21)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the fault handler(0-2) cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz_cbc_iter(&self) -> impl Iterator<Item = TZ_CBC_R> + '_ {
        (0..3).map(move |n| TZ_CBC_R::new(((self.bits >> (n + 21)) & 1) != 0))
    }
    #[doc = "Bit 21 - The interrupt status bit for the fault handler0 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz0_cbc(&self) -> TZ_CBC_R {
        TZ_CBC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt status bit for the fault handler1 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz1_cbc(&self) -> TZ_CBC_R {
        TZ_CBC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt status bit for the fault handler2 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz2_cbc(&self) -> TZ_CBC_R {
        TZ_CBC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the fault handler(0-2) one-shot mode action."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST` field.</div>"]
    #[inline(always)]
    pub fn tz_ost(&self, n: u8) -> TZ_OST_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the fault handler(0-2) one-shot mode action."]
    #[inline(always)]
    pub fn tz_ost_iter(&self) -> impl Iterator<Item = TZ_OST_R> + '_ {
        (0..3).map(move |n| TZ_OST_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - The interrupt status bit for the fault handler0 one-shot mode action."]
    #[inline(always)]
    pub fn tz0_ost(&self) -> TZ_OST_R {
        TZ_OST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt status bit for the fault handler1 one-shot mode action."]
    #[inline(always)]
    pub fn tz1_ost(&self) -> TZ_OST_R {
        TZ_OST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt status bit for the fault handler2 one-shot mode action."]
    #[inline(always)]
    pub fn tz2_ost(&self) -> TZ_OST_R {
        TZ_OST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "The interrupt status bit for the capture(0-2) event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0` field.</div>"]
    #[inline(always)]
    pub fn cap(&self, n: u8) -> CAP_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_R::new(((self.bits >> (n + 27)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt status bit for the capture(0-2) event."]
    #[inline(always)]
    pub fn cap_iter(&self) -> impl Iterator<Item = CAP_R> + '_ {
        (0..3).map(move |n| CAP_R::new(((self.bits >> (n + 27)) & 1) != 0))
    }
    #[doc = "Bit 27 - The interrupt status bit for the capture0 event."]
    #[inline(always)]
    pub fn cap0(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt status bit for the capture1 event."]
    #[inline(always)]
    pub fn cap1(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt status bit for the capture2 event."]
    #[inline(always)]
    pub fn cap2(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("timer0_stop", &self.timer0_stop())
            .field("timer1_stop", &self.timer1_stop())
            .field("timer2_stop", &self.timer2_stop())
            .field("timer0_tez", &self.timer0_tez())
            .field("timer1_tez", &self.timer1_tez())
            .field("timer2_tez", &self.timer2_tez())
            .field("timer0_tep", &self.timer0_tep())
            .field("timer1_tep", &self.timer1_tep())
            .field("timer2_tep", &self.timer2_tep())
            .field("fault0", &self.fault0())
            .field("fault1", &self.fault1())
            .field("fault2", &self.fault2())
            .field("fault0_clr", &self.fault0_clr())
            .field("fault1_clr", &self.fault1_clr())
            .field("fault2_clr", &self.fault2_clr())
            .field("cmpr0_tea", &self.cmpr0_tea())
            .field("cmpr1_tea", &self.cmpr1_tea())
            .field("cmpr2_tea", &self.cmpr2_tea())
            .field("cmpr0_teb", &self.cmpr0_teb())
            .field("cmpr1_teb", &self.cmpr1_teb())
            .field("cmpr2_teb", &self.cmpr2_teb())
            .field("tz0_cbc", &self.tz0_cbc())
            .field("tz1_cbc", &self.tz1_cbc())
            .field("tz2_cbc", &self.tz2_cbc())
            .field("tz0_ost", &self.tz0_ost())
            .field("tz1_ost", &self.tz1_ost())
            .field("tz2_ost", &self.tz2_ost())
            .field("cap0", &self.cap0())
            .field("cap1", &self.cap1())
            .field("cap2", &self.cap2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
