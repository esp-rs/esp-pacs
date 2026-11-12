#[doc = "Register `TASK_EN` reader"]
pub type R = crate::R<TASK_EN_SPEC>;
#[doc = "Register `TASK_EN` writer"]
pub type W = crate::W<TASK_EN_SPEC>;
#[doc = "Field `CMPR_A_UP_EN(0-2)` reader - set this bit high to enable PWM generator%s timer stamp A's shadow register update task receive"]
pub type CMPR_A_UP_EN_R = crate::BitReader;
#[doc = "Field `CMPR_A_UP_EN(0-2)` writer - set this bit high to enable PWM generator%s timer stamp A's shadow register update task receive"]
pub type CMPR_A_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR_B_UP_EN(0-2)` reader - set this bit high to enable PWM generator%s timer stamp B's shadow register update task receive"]
pub type CMPR_B_UP_EN_R = crate::BitReader;
#[doc = "Field `CMPR_B_UP_EN(0-2)` writer - set this bit high to enable PWM generator%s timer stamp B's shadow register update task receive"]
pub type CMPR_B_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_STOP_EN` reader - set this bit high to enable all PWM generate stop task receive"]
pub type GEN_STOP_EN_R = crate::BitReader;
#[doc = "Field `GEN_STOP_EN` writer - set this bit high to enable all PWM generate stop task receive"]
pub type GEN_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_SYNC_EN(0-2)` reader - set this bit high to enable timer%s sync task receive"]
pub type TIMER_SYNC_EN_R = crate::BitReader;
#[doc = "Field `TIMER_SYNC_EN(0-2)` writer - set this bit high to enable timer%s sync task receive"]
pub type TIMER_SYNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_PERIOD_UP_EN(0-2)` reader - set this bit high to enable timer%s period update task receive"]
pub type TIMER_PERIOD_UP_EN_R = crate::BitReader;
#[doc = "Field `TIMER_PERIOD_UP_EN(0-2)` writer - set this bit high to enable timer%s period update task receive"]
pub type TIMER_PERIOD_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_OST_EN(0-2)` reader - set this bit high to enable one shot trip%s task receive"]
pub type TZ_OST_EN_R = crate::BitReader;
#[doc = "Field `TZ_OST_EN(0-2)` writer - set this bit high to enable one shot trip%s task receive"]
pub type TZ_OST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OST_EN(0-2)` reader - set this bit high to enable one shot trip%s clear task receive"]
pub type CLR_OST_EN_R = crate::BitReader;
#[doc = "Field `CLR_OST_EN(0-2)` writer - set this bit high to enable one shot trip%s clear task receive"]
pub type CLR_OST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_EN(0-2)` reader - set this bit high to enable capture%s task receive"]
pub type CAP_EN_R = crate::BitReader;
#[doc = "Field `CAP_EN(0-2)` writer - set this bit high to enable capture%s task receive"]
pub type CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp A's shadow register update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_A_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn cmpr_a_up_en(&self, n: u8) -> CMPR_A_UP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_A_UP_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr_a_up_en_iter(&self) -> impl Iterator<Item = CMPR_A_UP_EN_R> + '_ {
        (0..3).map(move |n| CMPR_A_UP_EN_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr0_a_up_en(&self) -> CMPR_A_UP_EN_R {
        CMPR_A_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr1_a_up_en(&self) -> CMPR_A_UP_EN_R {
        CMPR_A_UP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr2_a_up_en(&self) -> CMPR_A_UP_EN_R {
        CMPR_A_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp B's shadow register update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_B_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn cmpr_b_up_en(&self, n: u8) -> CMPR_B_UP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_B_UP_EN_R::new(((self.bits >> (n + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr_b_up_en_iter(&self) -> impl Iterator<Item = CMPR_B_UP_EN_R> + '_ {
        (0..3).map(move |n| CMPR_B_UP_EN_R::new(((self.bits >> (n + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr0_b_up_en(&self) -> CMPR_B_UP_EN_R {
        CMPR_B_UP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr1_b_up_en(&self) -> CMPR_B_UP_EN_R {
        CMPR_B_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr2_b_up_en(&self) -> CMPR_B_UP_EN_R {
        CMPR_B_UP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set this bit high to enable all PWM generate stop task receive"]
    #[inline(always)]
    pub fn gen_stop_en(&self) -> GEN_STOP_EN_R {
        GEN_STOP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "set this bit high to enable timer(0-2) sync task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_SYNC_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_sync_en(&self, n: u8) -> TIMER_SYNC_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_SYNC_EN_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable timer(0-2) sync task receive"]
    #[inline(always)]
    pub fn timer_sync_en_iter(&self) -> impl Iterator<Item = TIMER_SYNC_EN_R> + '_ {
        (0..3).map(move |n| TIMER_SYNC_EN_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - set this bit high to enable timer0 sync task receive"]
    #[inline(always)]
    pub fn timer0_sync_en(&self) -> TIMER_SYNC_EN_R {
        TIMER_SYNC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - set this bit high to enable timer1 sync task receive"]
    #[inline(always)]
    pub fn timer1_sync_en(&self) -> TIMER_SYNC_EN_R {
        TIMER_SYNC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set this bit high to enable timer2 sync task receive"]
    #[inline(always)]
    pub fn timer2_sync_en(&self) -> TIMER_SYNC_EN_R {
        TIMER_SYNC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "set this bit high to enable timer(0-2) period update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_PERIOD_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_period_up_en(&self, n: u8) -> TIMER_PERIOD_UP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_PERIOD_UP_EN_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable timer(0-2) period update task receive"]
    #[inline(always)]
    pub fn timer_period_up_en_iter(&self) -> impl Iterator<Item = TIMER_PERIOD_UP_EN_R> + '_ {
        (0..3).map(move |n| TIMER_PERIOD_UP_EN_R::new(((self.bits >> (n + 10)) & 1) != 0))
    }
    #[doc = "Bit 10 - set this bit high to enable timer0 period update task receive"]
    #[inline(always)]
    pub fn timer0_period_up_en(&self) -> TIMER_PERIOD_UP_EN_R {
        TIMER_PERIOD_UP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - set this bit high to enable timer1 period update task receive"]
    #[inline(always)]
    pub fn timer1_period_up_en(&self) -> TIMER_PERIOD_UP_EN_R {
        TIMER_PERIOD_UP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - set this bit high to enable timer2 period update task receive"]
    #[inline(always)]
    pub fn timer2_period_up_en(&self) -> TIMER_PERIOD_UP_EN_R {
        TIMER_PERIOD_UP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_ost_en(&self, n: u8) -> TZ_OST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_EN_R::new(((self.bits >> (n + 13)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable one shot trip(0-2) task receive"]
    #[inline(always)]
    pub fn tz_ost_en_iter(&self) -> impl Iterator<Item = TZ_OST_EN_R> + '_ {
        (0..3).map(move |n| TZ_OST_EN_R::new(((self.bits >> (n + 13)) & 1) != 0))
    }
    #[doc = "Bit 13 - set this bit high to enable one shot trip0 task receive"]
    #[inline(always)]
    pub fn tz0_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - set this bit high to enable one shot trip1 task receive"]
    #[inline(always)]
    pub fn tz1_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - set this bit high to enable one shot trip2 task receive"]
    #[inline(always)]
    pub fn tz2_ost_en(&self) -> TZ_OST_EN_R {
        TZ_OST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) clear task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CLR0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn clr_ost_en(&self, n: u8) -> CLR_OST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLR_OST_EN_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable one shot trip(0-2) clear task receive"]
    #[inline(always)]
    pub fn clr_ost_en_iter(&self) -> impl Iterator<Item = CLR_OST_EN_R> + '_ {
        (0..3).map(move |n| CLR_OST_EN_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - set this bit high to enable one shot trip0 clear task receive"]
    #[inline(always)]
    pub fn clr0_ost_en(&self) -> CLR_OST_EN_R {
        CLR_OST_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - set this bit high to enable one shot trip1 clear task receive"]
    #[inline(always)]
    pub fn clr1_ost_en(&self) -> CLR_OST_EN_R {
        CLR_OST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - set this bit high to enable one shot trip2 clear task receive"]
    #[inline(always)]
    pub fn clr2_ost_en(&self) -> CLR_OST_EN_R {
        CLR_OST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "set this bit high to enable capture(0-2) task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0_EN` field.</div>"]
    #[inline(always)]
    pub fn cap_en(&self, n: u8) -> CAP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_EN_R::new(((self.bits >> (n + 19)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "set this bit high to enable capture(0-2) task receive"]
    #[inline(always)]
    pub fn cap_en_iter(&self) -> impl Iterator<Item = CAP_EN_R> + '_ {
        (0..3).map(move |n| CAP_EN_R::new(((self.bits >> (n + 19)) & 1) != 0))
    }
    #[doc = "Bit 19 - set this bit high to enable capture0 task receive"]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - set this bit high to enable capture1 task receive"]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - set this bit high to enable capture2 task receive"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_EN")
            .field("cmpr0_a_up_en", &self.cmpr0_a_up_en())
            .field("cmpr1_a_up_en", &self.cmpr1_a_up_en())
            .field("cmpr2_a_up_en", &self.cmpr2_a_up_en())
            .field("cmpr0_b_up_en", &self.cmpr0_b_up_en())
            .field("cmpr1_b_up_en", &self.cmpr1_b_up_en())
            .field("cmpr2_b_up_en", &self.cmpr2_b_up_en())
            .field("gen_stop_en", &self.gen_stop_en())
            .field("timer0_sync_en", &self.timer0_sync_en())
            .field("timer1_sync_en", &self.timer1_sync_en())
            .field("timer2_sync_en", &self.timer2_sync_en())
            .field("timer0_period_up_en", &self.timer0_period_up_en())
            .field("timer1_period_up_en", &self.timer1_period_up_en())
            .field("timer2_period_up_en", &self.timer2_period_up_en())
            .field("tz0_ost_en", &self.tz0_ost_en())
            .field("tz1_ost_en", &self.tz1_ost_en())
            .field("tz2_ost_en", &self.tz2_ost_en())
            .field("clr0_ost_en", &self.clr0_ost_en())
            .field("clr1_ost_en", &self.clr1_ost_en())
            .field("clr2_ost_en", &self.clr2_ost_en())
            .field("cap0_en", &self.cap0_en())
            .field("cap1_en", &self.cap1_en())
            .field("cap2_en", &self.cap2_en())
            .finish()
    }
}
impl W {
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp A's shadow register update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_A_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn cmpr_a_up_en(&mut self, n: u8) -> CMPR_A_UP_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_A_UP_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - set this bit high to enable PWM generator0 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr0_a_up_en(&mut self) -> CMPR_A_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_A_UP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - set this bit high to enable PWM generator1 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr1_a_up_en(&mut self) -> CMPR_A_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_A_UP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - set this bit high to enable PWM generator2 timer stamp A's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr2_a_up_en(&mut self) -> CMPR_A_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_A_UP_EN_W::new(self, 2)
    }
    #[doc = "set this bit high to enable PWM generator(0-2) timer stamp B's shadow register update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_B_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn cmpr_b_up_en(&mut self, n: u8) -> CMPR_B_UP_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_B_UP_EN_W::new(self, n + 3)
    }
    #[doc = "Bit 3 - set this bit high to enable PWM generator0 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr0_b_up_en(&mut self) -> CMPR_B_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_B_UP_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - set this bit high to enable PWM generator1 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr1_b_up_en(&mut self) -> CMPR_B_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_B_UP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - set this bit high to enable PWM generator2 timer stamp B's shadow register update task receive"]
    #[inline(always)]
    pub fn cmpr2_b_up_en(&mut self) -> CMPR_B_UP_EN_W<'_, TASK_EN_SPEC> {
        CMPR_B_UP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - set this bit high to enable all PWM generate stop task receive"]
    #[inline(always)]
    pub fn gen_stop_en(&mut self) -> GEN_STOP_EN_W<'_, TASK_EN_SPEC> {
        GEN_STOP_EN_W::new(self, 6)
    }
    #[doc = "set this bit high to enable timer(0-2) sync task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_SYNC_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_sync_en(&mut self, n: u8) -> TIMER_SYNC_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_SYNC_EN_W::new(self, n + 7)
    }
    #[doc = "Bit 7 - set this bit high to enable timer0 sync task receive"]
    #[inline(always)]
    pub fn timer0_sync_en(&mut self) -> TIMER_SYNC_EN_W<'_, TASK_EN_SPEC> {
        TIMER_SYNC_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - set this bit high to enable timer1 sync task receive"]
    #[inline(always)]
    pub fn timer1_sync_en(&mut self) -> TIMER_SYNC_EN_W<'_, TASK_EN_SPEC> {
        TIMER_SYNC_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - set this bit high to enable timer2 sync task receive"]
    #[inline(always)]
    pub fn timer2_sync_en(&mut self) -> TIMER_SYNC_EN_W<'_, TASK_EN_SPEC> {
        TIMER_SYNC_EN_W::new(self, 9)
    }
    #[doc = "set this bit high to enable timer(0-2) period update task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_PERIOD_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_period_up_en(&mut self, n: u8) -> TIMER_PERIOD_UP_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_PERIOD_UP_EN_W::new(self, n + 10)
    }
    #[doc = "Bit 10 - set this bit high to enable timer0 period update task receive"]
    #[inline(always)]
    pub fn timer0_period_up_en(&mut self) -> TIMER_PERIOD_UP_EN_W<'_, TASK_EN_SPEC> {
        TIMER_PERIOD_UP_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - set this bit high to enable timer1 period update task receive"]
    #[inline(always)]
    pub fn timer1_period_up_en(&mut self) -> TIMER_PERIOD_UP_EN_W<'_, TASK_EN_SPEC> {
        TIMER_PERIOD_UP_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - set this bit high to enable timer2 period update task receive"]
    #[inline(always)]
    pub fn timer2_period_up_en(&mut self) -> TIMER_PERIOD_UP_EN_W<'_, TASK_EN_SPEC> {
        TIMER_PERIOD_UP_EN_W::new(self, 12)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn tz_ost_en(&mut self, n: u8) -> TZ_OST_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_EN_W::new(self, n + 13)
    }
    #[doc = "Bit 13 - set this bit high to enable one shot trip0 task receive"]
    #[inline(always)]
    pub fn tz0_ost_en(&mut self) -> TZ_OST_EN_W<'_, TASK_EN_SPEC> {
        TZ_OST_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - set this bit high to enable one shot trip1 task receive"]
    #[inline(always)]
    pub fn tz1_ost_en(&mut self) -> TZ_OST_EN_W<'_, TASK_EN_SPEC> {
        TZ_OST_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - set this bit high to enable one shot trip2 task receive"]
    #[inline(always)]
    pub fn tz2_ost_en(&mut self) -> TZ_OST_EN_W<'_, TASK_EN_SPEC> {
        TZ_OST_EN_W::new(self, 15)
    }
    #[doc = "set this bit high to enable one shot trip(0-2) clear task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CLR0_OST_EN` field.</div>"]
    #[inline(always)]
    pub fn clr_ost_en(&mut self, n: u8) -> CLR_OST_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CLR_OST_EN_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - set this bit high to enable one shot trip0 clear task receive"]
    #[inline(always)]
    pub fn clr0_ost_en(&mut self) -> CLR_OST_EN_W<'_, TASK_EN_SPEC> {
        CLR_OST_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - set this bit high to enable one shot trip1 clear task receive"]
    #[inline(always)]
    pub fn clr1_ost_en(&mut self) -> CLR_OST_EN_W<'_, TASK_EN_SPEC> {
        CLR_OST_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - set this bit high to enable one shot trip2 clear task receive"]
    #[inline(always)]
    pub fn clr2_ost_en(&mut self) -> CLR_OST_EN_W<'_, TASK_EN_SPEC> {
        CLR_OST_EN_W::new(self, 18)
    }
    #[doc = "set this bit high to enable capture(0-2) task receive"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0_EN` field.</div>"]
    #[inline(always)]
    pub fn cap_en(&mut self, n: u8) -> CAP_EN_W<'_, TASK_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_EN_W::new(self, n + 19)
    }
    #[doc = "Bit 19 - set this bit high to enable capture0 task receive"]
    #[inline(always)]
    pub fn cap0_en(&mut self) -> CAP_EN_W<'_, TASK_EN_SPEC> {
        CAP_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - set this bit high to enable capture1 task receive"]
    #[inline(always)]
    pub fn cap1_en(&mut self) -> CAP_EN_W<'_, TASK_EN_SPEC> {
        CAP_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - set this bit high to enable capture2 task receive"]
    #[inline(always)]
    pub fn cap2_en(&mut self) -> CAP_EN_W<'_, TASK_EN_SPEC> {
        CAP_EN_W::new(self, 21)
    }
}
#[doc = "MCPWM task enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_EN_SPEC;
impl crate::RegisterSpec for TASK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_en::R`](R) reader structure"]
impl crate::Readable for TASK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_en::W`](W) writer structure"]
impl crate::Writable for TASK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_EN to value 0"]
impl crate::Resettable for TASK_EN_SPEC {}
