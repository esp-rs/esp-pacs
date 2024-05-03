#[doc = "Register `TASK_EN` reader"]
pub type R = crate::R<TASK_EN_SPEC>;
#[doc = "Register `TASK_EN` writer"]
pub type W = crate::W<TASK_EN_SPEC>;
#[doc = "Field `CMPR0_A_UP` reader - Configures whether or not to enable PWM generator0 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR0_A_UP_R = crate::BitReader;
#[doc = "Field `CMPR0_A_UP` writer - Configures whether or not to enable PWM generator0 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR0_A_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR1_A_UP` reader - Configures whether or not to enable PWM generator1 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR1_A_UP_R = crate::BitReader;
#[doc = "Field `CMPR1_A_UP` writer - Configures whether or not to enable PWM generator1 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR1_A_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR2_A_UP` reader - Configures whether or not to enable PWM generator2 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR2_A_UP_R = crate::BitReader;
#[doc = "Field `CMPR2_A_UP` writer - Configures whether or not to enable PWM generator2 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR2_A_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR0_B_UP` reader - Configures whether or not to enable PWM generator0 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR0_B_UP_R = crate::BitReader;
#[doc = "Field `CMPR0_B_UP` writer - Configures whether or not to enable PWM generator0 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR0_B_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR1_B_UP` reader - Configures whether or not to enable PWM generator1 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR1_B_UP_R = crate::BitReader;
#[doc = "Field `CMPR1_B_UP` writer - Configures whether or not to enable PWM generator1 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR1_B_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR2_B_UP` reader - Configures whether or not to enable PWM generator2 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR2_B_UP_R = crate::BitReader;
#[doc = "Field `CMPR2_B_UP` writer - Configures whether or not to enable PWM generator2 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
pub type CMPR2_B_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_STOP` reader - Configures whether or not to enable all PWM generate stop task receive.\\\\0: Disable\\\\1: Enable"]
pub type GEN_STOP_R = crate::BitReader;
#[doc = "Field `GEN_STOP` writer - Configures whether or not to enable all PWM generate stop task receive.\\\\0: Disable\\\\1: Enable"]
pub type GEN_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_SYNC` reader - Configures whether or not to enable timer0 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_SYNC_R = crate::BitReader;
#[doc = "Field `TIMER0_SYNC` writer - Configures whether or not to enable timer0 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_SYNC` reader - Configures whether or not to enable timer1 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_SYNC_R = crate::BitReader;
#[doc = "Field `TIMER1_SYNC` writer - Configures whether or not to enable timer1 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_SYNC` reader - Configures whether or not to enable timer2 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_SYNC_R = crate::BitReader;
#[doc = "Field `TIMER2_SYNC` writer - Configures whether or not to enable timer2 sync task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_PERIOD_UP` reader - Configures whether or not to enable timer0 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_PERIOD_UP_R = crate::BitReader;
#[doc = "Field `TIMER0_PERIOD_UP` writer - Configures whether or not to enable timer0 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_PERIOD_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_PERIOD_UP` reader - Configures whether or not to enable timer1 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_PERIOD_UP_R = crate::BitReader;
#[doc = "Field `TIMER1_PERIOD_UP` writer - Configures whether or not to enable timer1 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_PERIOD_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_PERIOD_UP` reader - Configures whether or not to enable timer2 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_PERIOD_UP_R = crate::BitReader;
#[doc = "Field `TIMER2_PERIOD_UP` writer - Configures whether or not to enable timer2 period update task receive.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_PERIOD_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_OST` reader - Configures whether or not to enable one shot trip0 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_OST_R = crate::BitReader;
#[doc = "Field `TZ0_OST` writer - Configures whether or not to enable one shot trip0 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_OST` reader - Configures whether or not to enable one shot trip1 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_OST_R = crate::BitReader;
#[doc = "Field `TZ1_OST` writer - Configures whether or not to enable one shot trip1 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_OST` reader - Configures whether or not to enable one shot trip2 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_OST_R = crate::BitReader;
#[doc = "Field `TZ2_OST` writer - Configures whether or not to enable one shot trip2 task receive.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR0_OST` reader - Configures whether or not to enable one shot trip0 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR0_OST_R = crate::BitReader;
#[doc = "Field `CLR0_OST` writer - Configures whether or not to enable one shot trip0 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR1_OST` reader - Configures whether or not to enable one shot trip1 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR1_OST_R = crate::BitReader;
#[doc = "Field `CLR1_OST` writer - Configures whether or not to enable one shot trip1 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR2_OST` reader - Configures whether or not to enable one shot trip2 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR2_OST_R = crate::BitReader;
#[doc = "Field `CLR2_OST` writer - Configures whether or not to enable one shot trip2 clear task receive.\\\\0: Disable\\\\1: Enable"]
pub type CLR2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - Configures whether or not to enable capture0 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP0_R = crate::BitReader;
#[doc = "Field `CAP0` writer - Configures whether or not to enable capture0 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1` reader - Configures whether or not to enable capture1 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP1_R = crate::BitReader;
#[doc = "Field `CAP1` writer - Configures whether or not to enable capture1 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2` reader - Configures whether or not to enable capture2 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP2_R = crate::BitReader;
#[doc = "Field `CAP2` writer - Configures whether or not to enable capture2 task receive.\\\\0: Disable\\\\1: Enable"]
pub type CAP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr0_a_up(&self) -> CMPR0_A_UP_R {
        CMPR0_A_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr1_a_up(&self) -> CMPR1_A_UP_R {
        CMPR1_A_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr2_a_up(&self) -> CMPR2_A_UP_R {
        CMPR2_A_UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr0_b_up(&self) -> CMPR0_B_UP_R {
        CMPR0_B_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr1_b_up(&self) -> CMPR1_B_UP_R {
        CMPR1_B_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cmpr2_b_up(&self) -> CMPR2_B_UP_R {
        CMPR2_B_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable all PWM generate stop task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn gen_stop(&self) -> GEN_STOP_R {
        GEN_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer0 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_sync(&self) -> TIMER0_SYNC_R {
        TIMER0_SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer1 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_sync(&self) -> TIMER1_SYNC_R {
        TIMER1_SYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable timer2 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_sync(&self) -> TIMER2_SYNC_R {
        TIMER2_SYNC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable timer0 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_period_up(&self) -> TIMER0_PERIOD_UP_R {
        TIMER0_PERIOD_UP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable timer1 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_period_up(&self) -> TIMER1_PERIOD_UP_R {
        TIMER1_PERIOD_UP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable timer2 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_period_up(&self) -> TIMER2_PERIOD_UP_R {
        TIMER2_PERIOD_UP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable one shot trip0 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_ost(&self) -> TZ0_OST_R {
        TZ0_OST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable one shot trip1 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_ost(&self) -> TZ1_OST_R {
        TZ1_OST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable one shot trip2 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_ost(&self) -> TZ2_OST_R {
        TZ2_OST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to enable one shot trip0 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn clr0_ost(&self) -> CLR0_OST_R {
        CLR0_OST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable one shot trip1 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn clr1_ost(&self) -> CLR1_OST_R {
        CLR1_OST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to enable one shot trip2 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn clr2_ost(&self) -> CLR2_OST_R {
        CLR2_OST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to enable capture0 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable capture1 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap1(&self) -> CAP1_R {
        CAP1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to enable capture2 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap2(&self) -> CAP2_R {
        CAP2_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_EN")
            .field("cmpr0_a_up", &self.cmpr0_a_up().bit())
            .field("cmpr1_a_up", &self.cmpr1_a_up().bit())
            .field("cmpr2_a_up", &self.cmpr2_a_up().bit())
            .field("cmpr0_b_up", &self.cmpr0_b_up().bit())
            .field("cmpr1_b_up", &self.cmpr1_b_up().bit())
            .field("cmpr2_b_up", &self.cmpr2_b_up().bit())
            .field("gen_stop", &self.gen_stop().bit())
            .field("timer0_sync", &self.timer0_sync().bit())
            .field("timer1_sync", &self.timer1_sync().bit())
            .field("timer2_sync", &self.timer2_sync().bit())
            .field("timer0_period_up", &self.timer0_period_up().bit())
            .field("timer1_period_up", &self.timer1_period_up().bit())
            .field("timer2_period_up", &self.timer2_period_up().bit())
            .field("tz0_ost", &self.tz0_ost().bit())
            .field("tz1_ost", &self.tz1_ost().bit())
            .field("tz2_ost", &self.tz2_ost().bit())
            .field("clr0_ost", &self.clr0_ost().bit())
            .field("clr1_ost", &self.clr1_ost().bit())
            .field("clr2_ost", &self.clr2_ost().bit())
            .field("cap0", &self.cap0().bit())
            .field("cap1", &self.cap1().bit())
            .field("cap2", &self.cap2().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_a_up(&mut self) -> CMPR0_A_UP_W<TASK_EN_SPEC> {
        CMPR0_A_UP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_a_up(&mut self) -> CMPR1_A_UP_W<TASK_EN_SPEC> {
        CMPR1_A_UP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer stamp A's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_a_up(&mut self) -> CMPR2_A_UP_W<TASK_EN_SPEC> {
        CMPR2_A_UP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b_up(&mut self) -> CMPR0_B_UP_W<TASK_EN_SPEC> {
        CMPR0_B_UP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_b_up(&mut self) -> CMPR1_B_UP_W<TASK_EN_SPEC> {
        CMPR1_B_UP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer stamp B's shadow register update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_b_up(&mut self) -> CMPR2_B_UP_W<TASK_EN_SPEC> {
        CMPR2_B_UP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable all PWM generate stop task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen_stop(&mut self) -> GEN_STOP_W<TASK_EN_SPEC> {
        GEN_STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer0 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_sync(&mut self) -> TIMER0_SYNC_W<TASK_EN_SPEC> {
        TIMER0_SYNC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer1 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_sync(&mut self) -> TIMER1_SYNC_W<TASK_EN_SPEC> {
        TIMER1_SYNC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable timer2 sync task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_sync(&mut self) -> TIMER2_SYNC_W<TASK_EN_SPEC> {
        TIMER2_SYNC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable timer0 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_period_up(&mut self) -> TIMER0_PERIOD_UP_W<TASK_EN_SPEC> {
        TIMER0_PERIOD_UP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable timer1 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_period_up(&mut self) -> TIMER1_PERIOD_UP_W<TASK_EN_SPEC> {
        TIMER1_PERIOD_UP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable timer2 period update task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_period_up(&mut self) -> TIMER2_PERIOD_UP_W<TASK_EN_SPEC> {
        TIMER2_PERIOD_UP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable one shot trip0 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_ost(&mut self) -> TZ0_OST_W<TASK_EN_SPEC> {
        TZ0_OST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable one shot trip1 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz1_ost(&mut self) -> TZ1_OST_W<TASK_EN_SPEC> {
        TZ1_OST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable one shot trip2 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_ost(&mut self) -> TZ2_OST_W<TASK_EN_SPEC> {
        TZ2_OST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable one shot trip0 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr0_ost(&mut self) -> CLR0_OST_W<TASK_EN_SPEC> {
        CLR0_OST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable one shot trip1 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr1_ost(&mut self) -> CLR1_OST_W<TASK_EN_SPEC> {
        CLR1_OST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable one shot trip2 clear task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr2_ost(&mut self) -> CLR2_OST_W<TASK_EN_SPEC> {
        CLR2_OST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable capture0 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap0(&mut self) -> CAP0_W<TASK_EN_SPEC> {
        CAP0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable capture1 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap1(&mut self) -> CAP1_W<TASK_EN_SPEC> {
        CAP1_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable capture2 task receive.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap2(&mut self) -> CAP2_W<TASK_EN_SPEC> {
        CAP2_W::new(self, 21)
    }
}
#[doc = "Task enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_EN_SPEC;
impl crate::RegisterSpec for TASK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_en::R`](R) reader structure"]
impl crate::Readable for TASK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_en::W`](W) writer structure"]
impl crate::Writable for TASK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASK_EN to value 0"]
impl crate::Resettable for TASK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
