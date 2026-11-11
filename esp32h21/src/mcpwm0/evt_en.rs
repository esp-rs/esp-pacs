#[doc = "Register `EVT_EN` reader"]
pub type R = crate::R<EVT_EN_SPEC>;
#[doc = "Register `EVT_EN` writer"]
pub type W = crate::W<EVT_EN_SPEC>;
#[doc = "Field `TIMER_STOP_EN(0-2)` reader - set this bit high to enable timer%s stop event generate"]
pub type TIMER_STOP_EN_R = crate::BitReader;
#[doc = "Field `TIMER_STOP_EN(0-2)` writer - set this bit high to enable timer%s stop event generate"]
pub type TIMER_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_TEZ_EN(0-2)` reader - set this bit high to enable timer%s equal zero event generate"]
pub type TIMER_TEZ_EN_R = crate::BitReader;
#[doc = "Field `TIMER_TEZ_EN(0-2)` writer - set this bit high to enable timer%s equal zero event generate"]
pub type TIMER_TEZ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_TEP_EN(0-2)` reader - set this bit high to enable timer%s equal period event generate"]
pub type TIMER_TEP_EN_R = crate::BitReader;
#[doc = "Field `TIMER_TEP_EN(0-2)` writer - set this bit high to enable timer%s equal period event generate"]
pub type TIMER_TEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP_TEA_EN(0-2)` reader - set this bit high to enable PWM generator%s timer equal a event generate"]
pub type OP_TEA_EN_R = crate::BitReader;
#[doc = "Field `OP_TEA_EN(0-2)` writer - set this bit high to enable PWM generator%s timer equal a event generate"]
pub type OP_TEA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP_TEB_EN(0-2)` reader - set this bit high to enable PWM generator%s timer equal b event generate"]
pub type OP_TEB_EN_R = crate::BitReader;
#[doc = "Field `OP_TEB_EN(0-2)` writer - set this bit high to enable PWM generator%s timer equal b event generate"]
pub type OP_TEB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_EN(0-2)` reader - set this bit high to enable fault%s event generate"]
pub type F_EN_R = crate::BitReader;
#[doc = "Field `F_EN(0-2)` writer - set this bit high to enable fault%s event generate"]
pub type F_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CLR_EN(0-2)` reader - set this bit high to enable fault%s clear event generate"]
pub type F_CLR_EN_R = crate::BitReader;
#[doc = "Field `F_CLR_EN(0-2)` writer - set this bit high to enable fault%s clear event generate"]
pub type F_CLR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_CBC_EN(0-2)` reader - set this bit high to enable cycle by cycle trip%s event generate"]
pub type TZ_CBC_EN_R = crate::BitReader;
#[doc = "Field `TZ_CBC_EN(0-2)` writer - set this bit high to enable cycle by cycle trip%s event generate"]
pub type TZ_CBC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_OST_EN(0-2)` reader - set this bit high to enable one shot trip%s event generate"]
pub type TZ_OST_EN_R = crate::BitReader;
#[doc = "Field `TZ_OST_EN(0-2)` writer - set this bit high to enable one shot trip%s event generate"]
pub type TZ_OST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_EN(0-2)` reader - set this bit high to enable capture%s event generate"]
pub type CAP_EN_R = crate::BitReader;
#[doc = "Field `CAP_EN(0-2)` writer - set this bit high to enable capture%s event generate"]
pub type CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "set this bit high to enable timer(0-2) stop event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_STOP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_stop_en(&self, n: u8) -> TIMER_STOP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_STOP_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable timer(0-2) stop event generate"]
    #[inline(always)]
    pub fn timer_stop_en_iter(&self) -> impl Iterator<Item = TIMER_STOP_EN_R> + '_ {
        (0..3).map(move |n| TIMER_STOP_EN_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - set this bit high to enable timer0 stop event generate"]
    #[inline(always)]
    pub fn timer0_stop_en(&self) -> TIMER_STOP_EN_R {
        TIMER_STOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit high to enable timer1 stop event generate"]
    #[inline(always)]
    pub fn timer1_stop_en(&self) -> TIMER_STOP_EN_R {
        TIMER_STOP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set this bit high to enable timer2 stop event generate"]
    #[inline(always)]
    pub fn timer2_stop_en(&self) -> TIMER_STOP_EN_R {
        TIMER_STOP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "set this bit high to enable timer(0-2) equal zero event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEZ_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_tez_en(&self, n: u8) -> TIMER_TEZ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEZ_EN_R::new(((self.bits >> (n + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable timer(0-2) equal zero event generate"]
    #[inline(always)]
    pub fn timer_tez_en_iter(&self) -> impl Iterator<Item = TIMER_TEZ_EN_R> + '_ {
        (0..3).map(move |n| TIMER_TEZ_EN_R::new(((self.bits >> (n + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - set this bit high to enable timer0 equal zero event generate"]
    #[inline(always)]
    pub fn timer0_tez_en(&self) -> TIMER_TEZ_EN_R {
        TIMER_TEZ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit high to enable timer1 equal zero event generate"]
    #[inline(always)]
    pub fn timer1_tez_en(&self) -> TIMER_TEZ_EN_R {
        TIMER_TEZ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set this bit high to enable timer2 equal zero event generate"]
    #[inline(always)]
    pub fn timer2_tez_en(&self) -> TIMER_TEZ_EN_R {
        TIMER_TEZ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "set this bit high to enable timer(0-2) equal period event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_tep_en(&self, n: u8) -> TIMER_TEP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEP_EN_R::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable timer(0-2) equal period event generate"]
    #[inline(always)]
    pub fn timer_tep_en_iter(&self) -> impl Iterator<Item = TIMER_TEP_EN_R> + '_ {
        (0..3).map(move |n| TIMER_TEP_EN_R::new(((self.bits >> (n + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - set this bit high to enable timer0 equal period event generate"]
    #[inline(always)]
    pub fn timer0_tep_en(&self) -> TIMER_TEP_EN_R {
        TIMER_TEP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit high to enable timer1 equal period event generate"]
    #[inline(always)]
    pub fn timer1_tep_en(&self) -> TIMER_TEP_EN_R {
        TIMER_TEP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - set this bit high to enable timer2 equal period event generate"]
    #[inline(always)]
    pub fn timer2_tep_en(&self) -> TIMER_TEP_EN_R {
        TIMER_TEP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal a event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_TEA_EN` field.</div>"]
    #[inline(always)]
    pub fn op_tea_en(&self, n: u8) -> OP_TEA_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_TEA_EN_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal a event generate"]
    #[inline(always)]
    pub fn op_tea_en_iter(&self) -> impl Iterator<Item = OP_TEA_EN_R> + '_ {
        (0..3).map(move |n| OP_TEA_EN_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - set this bit high to enable PWM generator0 timer equal a event generate"]
    #[inline(always)]
    pub fn op0_tea_en(&self) -> OP_TEA_EN_R {
        OP_TEA_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit high to enable PWM generator1 timer equal a event generate"]
    #[inline(always)]
    pub fn op1_tea_en(&self) -> OP_TEA_EN_R {
        OP_TEA_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - set this bit high to enable PWM generator2 timer equal a event generate"]
    #[inline(always)]
    pub fn op2_tea_en(&self) -> OP_TEA_EN_R {
        OP_TEA_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal b event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_TEB_EN` field.</div>"]
    #[inline(always)]
    pub fn op_teb_en(&self, n: u8) -> OP_TEB_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_TEB_EN_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal b event generate"]
    #[inline(always)]
    pub fn op_teb_en_iter(&self) -> impl Iterator<Item = OP_TEB_EN_R> + '_ {
        (0..3).map(move |n| OP_TEB_EN_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - set this bit high to enable PWM generator0 timer equal b event generate"]
    #[inline(always)]
    pub fn op0_teb_en(&self) -> OP_TEB_EN_R {
        OP_TEB_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - set this bit high to enable PWM generator1 timer equal b event generate"]
    #[inline(always)]
    pub fn op1_teb_en(&self) -> OP_TEB_EN_R {
        OP_TEB_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - set this bit high to enable PWM generator2 timer equal b event generate"]
    #[inline(always)]
    pub fn op2_teb_en(&self) -> OP_TEB_EN_R {
        OP_TEB_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "set this bit high to enable fault(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `F0_EN` field.</div>"]
    #[inline(always)]
    pub fn f_en(&self, n: u8) -> F_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        F_EN_R::new(((self.bits >> (n + 15)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable fault(0-2) event generate"]
    #[inline(always)]
    pub fn f_en_iter(&self) -> impl Iterator<Item = F_EN_R> + '_ {
        (0..3).map(move |n| F_EN_R::new(((self.bits >> (n + 15)) & 1) != 0))
    }
    #[doc = "Bit 15 - set this bit high to enable fault0 event generate"]
    #[inline(always)]
    pub fn f0_en(&self) -> F_EN_R {
        F_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - set this bit high to enable fault1 event generate"]
    #[inline(always)]
    pub fn f1_en(&self) -> F_EN_R {
        F_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - set this bit high to enable fault2 event generate"]
    #[inline(always)]
    pub fn f2_en(&self) -> F_EN_R {
        F_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "set this bit high to enable fault(0-2) clear event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `F0_CLR_EN` field.</div>"]
    #[inline(always)]
    pub fn f_clr_en(&self, n: u8) -> F_CLR_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        F_CLR_EN_R::new(((self.bits >> (n + 18)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable fault(0-2) clear event generate"]
    #[inline(always)]
    pub fn f_clr_en_iter(&self) -> impl Iterator<Item = F_CLR_EN_R> + '_ {
        (0..3).map(move |n| F_CLR_EN_R::new(((self.bits >> (n + 18)) & 1) != 0))
    }
    #[doc = "Bit 18 - set this bit high to enable fault0 clear event generate"]
    #[inline(always)]
    pub fn f0_clr_en(&self) -> F_CLR_EN_R {
        F_CLR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - set this bit high to enable fault1 clear event generate"]
    #[inline(always)]
    pub fn f1_clr_en(&self) -> F_CLR_EN_R {
        F_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - set this bit high to enable fault2 clear event generate"]
    #[inline(always)]
    pub fn f2_clr_en(&self) -> F_CLR_EN_R {
        F_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "set this bit high to enable cycle by cycle trip(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_CBC_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_cbc_en(&self, n: u8) -> TZ_CBC_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_CBC_EN_R::new(((self.bits >> (n + 21)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable cycle by cycle trip(0-2) event generate"]
    #[inline(always)]
    pub fn tz_cbc_en_iter(&self) -> impl Iterator<Item = TZ_CBC_EN_R> + '_ {
        (0..3).map(move |n| TZ_CBC_EN_R::new(((self.bits >> (n + 21)) & 1) != 0))
    }
    #[doc = "Bit 21 - set this bit high to enable cycle by cycle trip0 event generate"]
    #[inline(always)]
    pub fn tz0_cbc_en(&self) -> TZ_CBC_EN_R {
        TZ_CBC_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - set this bit high to enable cycle by cycle trip1 event generate"]
    #[inline(always)]
    pub fn tz1_cbc_en(&self) -> TZ_CBC_EN_R {
        TZ_CBC_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - set this bit high to enable cycle by cycle trip2 event generate"]
    #[inline(always)]
    pub fn tz2_cbc_en(&self) -> TZ_CBC_EN_R {
        TZ_CBC_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_ost_en(&self, n: u8) -> TZ_OST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_EN_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable one shot trip(0-2) event generate"]
    #[inline(always)]
    pub fn tz_ost_en_iter(&self) -> impl Iterator<Item = TZ_OST_EN_R> + '_ {
        (0..3).map(move |n| TZ_OST_EN_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - set this bit high to enable one shot trip0 event generate"]
    #[inline(always)]
    pub fn tz0_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - set this bit high to enable one shot trip1 event generate"]
    #[inline(always)]
    pub fn tz1_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - set this bit high to enable one shot trip2 event generate"]
    #[inline(always)]
    pub fn tz2_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "set this bit high to enable capture(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0_EN` field.</div>"]
    #[inline(always)]
    pub fn cap_en(&self, n: u8) -> CAP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_EN_R::new(((self.bits >> (n + 27)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable capture(0-2) event generate"]
    #[inline(always)]
    pub fn cap_en_iter(&self) -> impl Iterator<Item = CAP_EN_R> + '_ {
        (0..3).map(move |n| CAP_EN_R::new(((self.bits >> (n + 27)) & 1) != 0))
    }
    #[doc = "Bit 27 - set this bit high to enable capture0 event generate"]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - set this bit high to enable capture1 event generate"]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - set this bit high to enable capture2 event generate"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_EN")
            .field("timer0_stop_en", &self.timer0_stop_en())
            .field("timer1_stop_en", &self.timer1_stop_en())
            .field("timer2_stop_en", &self.timer2_stop_en())
            .field("timer0_tez_en", &self.timer0_tez_en())
            .field("timer1_tez_en", &self.timer1_tez_en())
            .field("timer2_tez_en", &self.timer2_tez_en())
            .field("timer0_tep_en", &self.timer0_tep_en())
            .field("timer1_tep_en", &self.timer1_tep_en())
            .field("timer2_tep_en", &self.timer2_tep_en())
            .field("op0_tea_en", &self.op0_tea_en())
            .field("op1_tea_en", &self.op1_tea_en())
            .field("op2_tea_en", &self.op2_tea_en())
            .field("op0_teb_en", &self.op0_teb_en())
            .field("op1_teb_en", &self.op1_teb_en())
            .field("op2_teb_en", &self.op2_teb_en())
            .field("f0_en", &self.f0_en())
            .field("f1_en", &self.f1_en())
            .field("f2_en", &self.f2_en())
            .field("f0_clr_en", &self.f0_clr_en())
            .field("f1_clr_en", &self.f1_clr_en())
            .field("f2_clr_en", &self.f2_clr_en())
            .field("tz0_cbc_en", &self.tz0_cbc_en())
            .field("tz1_cbc_en", &self.tz1_cbc_en())
            .field("tz2_cbc_en", &self.tz2_cbc_en())
            .field("tz0_ost_en", &self.tz0_ost_en())
            .field("tz1_ost_en", &self.tz1_ost_en())
            .field("tz2_ost_en", &self.tz2_ost_en())
            .field("cap0_en", &self.cap0_en())
            .field("cap1_en", &self.cap1_en())
            .field("cap2_en", &self.cap2_en())
            .finish()
    }
}
impl W {
    #[doc = "set this bit high to enable timer(0-2) stop event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_STOP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_stop_en(&mut self, n: u8) -> TIMER_STOP_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_STOP_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - set this bit high to enable timer0 stop event generate"]
    #[inline(always)]
    pub fn timer0_stop_en(&mut self) -> TIMER_STOP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_STOP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - set this bit high to enable timer1 stop event generate"]
    #[inline(always)]
    pub fn timer1_stop_en(&mut self) -> TIMER_STOP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_STOP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - set this bit high to enable timer2 stop event generate"]
    #[inline(always)]
    pub fn timer2_stop_en(&mut self) -> TIMER_STOP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_STOP_EN_W::new(self, 2)
    }
    #[doc = "set this bit high to enable timer(0-2) equal zero event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEZ_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_tez_en(&mut self, n: u8) -> TIMER_TEZ_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEZ_EN_W::new(self, n + 3)
    }
    #[doc = "Bit 3 - set this bit high to enable timer0 equal zero event generate"]
    #[inline(always)]
    pub fn timer0_tez_en(&mut self) -> TIMER_TEZ_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEZ_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - set this bit high to enable timer1 equal zero event generate"]
    #[inline(always)]
    pub fn timer1_tez_en(&mut self) -> TIMER_TEZ_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEZ_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - set this bit high to enable timer2 equal zero event generate"]
    #[inline(always)]
    pub fn timer2_tez_en(&mut self) -> TIMER_TEZ_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEZ_EN_W::new(self, 5)
    }
    #[doc = "set this bit high to enable timer(0-2) equal period event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_tep_en(&mut self, n: u8) -> TIMER_TEP_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEP_EN_W::new(self, n + 6)
    }
    #[doc = "Bit 6 - set this bit high to enable timer0 equal period event generate"]
    #[inline(always)]
    pub fn timer0_tep_en(&mut self) -> TIMER_TEP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - set this bit high to enable timer1 equal period event generate"]
    #[inline(always)]
    pub fn timer1_tep_en(&mut self) -> TIMER_TEP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEP_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - set this bit high to enable timer2 equal period event generate"]
    #[inline(always)]
    pub fn timer2_tep_en(&mut self) -> TIMER_TEP_EN_W<'_, EVT_EN_SPEC> {
        TIMER_TEP_EN_W::new(self, 8)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal a event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_TEA_EN` field.</div>"]
    #[inline(always)]
    pub fn op_tea_en(&mut self, n: u8) -> OP_TEA_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_TEA_EN_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - set this bit high to enable PWM generator0 timer equal a event generate"]
    #[inline(always)]
    pub fn op0_tea_en(&mut self) -> OP_TEA_EN_W<'_, EVT_EN_SPEC> {
        OP_TEA_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - set this bit high to enable PWM generator1 timer equal a event generate"]
    #[inline(always)]
    pub fn op1_tea_en(&mut self) -> OP_TEA_EN_W<'_, EVT_EN_SPEC> {
        OP_TEA_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - set this bit high to enable PWM generator2 timer equal a event generate"]
    #[inline(always)]
    pub fn op2_tea_en(&mut self) -> OP_TEA_EN_W<'_, EVT_EN_SPEC> {
        OP_TEA_EN_W::new(self, 11)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer equal b event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_TEB_EN` field.</div>"]
    #[inline(always)]
    pub fn op_teb_en(&mut self, n: u8) -> OP_TEB_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_TEB_EN_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - set this bit high to enable PWM generator0 timer equal b event generate"]
    #[inline(always)]
    pub fn op0_teb_en(&mut self) -> OP_TEB_EN_W<'_, EVT_EN_SPEC> {
        OP_TEB_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - set this bit high to enable PWM generator1 timer equal b event generate"]
    #[inline(always)]
    pub fn op1_teb_en(&mut self) -> OP_TEB_EN_W<'_, EVT_EN_SPEC> {
        OP_TEB_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - set this bit high to enable PWM generator2 timer equal b event generate"]
    #[inline(always)]
    pub fn op2_teb_en(&mut self) -> OP_TEB_EN_W<'_, EVT_EN_SPEC> {
        OP_TEB_EN_W::new(self, 14)
    }
    #[doc = "set this bit high to enable fault(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `F0_EN` field.</div>"]
    #[inline(always)]
    pub fn f_en(&mut self, n: u8) -> F_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        F_EN_W::new(self, n + 15)
    }
    #[doc = "Bit 15 - set this bit high to enable fault0 event generate"]
    #[inline(always)]
    pub fn f0_en(&mut self) -> F_EN_W<'_, EVT_EN_SPEC> {
        F_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - set this bit high to enable fault1 event generate"]
    #[inline(always)]
    pub fn f1_en(&mut self) -> F_EN_W<'_, EVT_EN_SPEC> {
        F_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - set this bit high to enable fault2 event generate"]
    #[inline(always)]
    pub fn f2_en(&mut self) -> F_EN_W<'_, EVT_EN_SPEC> {
        F_EN_W::new(self, 17)
    }
    #[doc = "set this bit high to enable fault(0-2) clear event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `F0_CLR_EN` field.</div>"]
    #[inline(always)]
    pub fn f_clr_en(&mut self, n: u8) -> F_CLR_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        F_CLR_EN_W::new(self, n + 18)
    }
    #[doc = "Bit 18 - set this bit high to enable fault0 clear event generate"]
    #[inline(always)]
    pub fn f0_clr_en(&mut self) -> F_CLR_EN_W<'_, EVT_EN_SPEC> {
        F_CLR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - set this bit high to enable fault1 clear event generate"]
    #[inline(always)]
    pub fn f1_clr_en(&mut self) -> F_CLR_EN_W<'_, EVT_EN_SPEC> {
        F_CLR_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - set this bit high to enable fault2 clear event generate"]
    #[inline(always)]
    pub fn f2_clr_en(&mut self) -> F_CLR_EN_W<'_, EVT_EN_SPEC> {
        F_CLR_EN_W::new(self, 20)
    }
    #[doc = "set this bit high to enable cycle by cycle trip(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_CBC_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_cbc_en(&mut self, n: u8) -> TZ_CBC_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_CBC_EN_W::new(self, n + 21)
    }
    #[doc = "Bit 21 - set this bit high to enable cycle by cycle trip0 event generate"]
    #[inline(always)]
    pub fn tz0_cbc_en(&mut self) -> TZ_CBC_EN_W<'_, EVT_EN_SPEC> {
        TZ_CBC_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - set this bit high to enable cycle by cycle trip1 event generate"]
    #[inline(always)]
    pub fn tz1_cbc_en(&mut self) -> TZ_CBC_EN_W<'_, EVT_EN_SPEC> {
        TZ_CBC_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - set this bit high to enable cycle by cycle trip2 event generate"]
    #[inline(always)]
    pub fn tz2_cbc_en(&mut self) -> TZ_CBC_EN_W<'_, EVT_EN_SPEC> {
        TZ_CBC_EN_W::new(self, 23)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_ost_en(&mut self, n: u8) -> TZ_OST_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_EN_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - set this bit high to enable one shot trip0 event generate"]
    #[inline(always)]
    pub fn tz0_ost_en(&mut self) -> TZ_OST_EN_W<'_, EVT_EN_SPEC> {
        TZ_OST_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - set this bit high to enable one shot trip1 event generate"]
    #[inline(always)]
    pub fn tz1_ost_en(&mut self) -> TZ_OST_EN_W<'_, EVT_EN_SPEC> {
        TZ_OST_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - set this bit high to enable one shot trip2 event generate"]
    #[inline(always)]
    pub fn tz2_ost_en(&mut self) -> TZ_OST_EN_W<'_, EVT_EN_SPEC> {
        TZ_OST_EN_W::new(self, 26)
    }
    #[doc = "set this bit high to enable capture(0-2) event generate"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0_EN` field.</div>"]
    #[inline(always)]
    pub fn cap_en(&mut self, n: u8) -> CAP_EN_W<'_, EVT_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_EN_W::new(self, n + 27)
    }
    #[doc = "Bit 27 - set this bit high to enable capture0 event generate"]
    #[inline(always)]
    pub fn cap0_en(&mut self) -> CAP_EN_W<'_, EVT_EN_SPEC> {
        CAP_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - set this bit high to enable capture1 event generate"]
    #[inline(always)]
    pub fn cap1_en(&mut self) -> CAP_EN_W<'_, EVT_EN_SPEC> {
        CAP_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - set this bit high to enable capture2 event generate"]
    #[inline(always)]
    pub fn cap2_en(&mut self) -> CAP_EN_W<'_, EVT_EN_SPEC> {
        CAP_EN_W::new(self, 29)
    }
}
#[doc = "MCPWM event enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_EN_SPEC;
impl crate::RegisterSpec for EVT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_en::R`](R) reader structure"]
impl crate::Readable for EVT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_en::W`](W) writer structure"]
impl crate::Writable for EVT_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_EN to value 0"]
impl crate::Resettable for EVT_EN_SPEC {}
