#[doc = "Register `EVT_EN` reader"]
pub type R = crate::R<EVT_EN_SPEC>;
#[doc = "Register `EVT_EN` writer"]
pub type W = crate::W<EVT_EN_SPEC>;
#[doc = "Field `TIMER0_STOP` reader - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_STOP_R = crate::BitReader;
#[doc = "Field `TIMER0_STOP` writer - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_STOP` reader - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_STOP_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP` writer - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_STOP` reader - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_STOP_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP` writer - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEZ` reader - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` writer - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEZ` reader - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` writer - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEZ` reader - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` writer - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEP` reader - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_TEP_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP` writer - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER0_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEP` reader - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_TEP_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP` writer - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER1_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEP` reader - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_TEP_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP` writer - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type TIMER2_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEA` reader - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP0_TEA_R = crate::BitReader;
#[doc = "Field `OP0_TEA` writer - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP0_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEA` reader - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP1_TEA_R = crate::BitReader;
#[doc = "Field `OP1_TEA` writer - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP1_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEA` reader - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP2_TEA_R = crate::BitReader;
#[doc = "Field `OP2_TEA` writer - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP2_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEB` reader - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP0_TEB_R = crate::BitReader;
#[doc = "Field `OP0_TEB` writer - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP0_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEB` reader - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP1_TEB_R = crate::BitReader;
#[doc = "Field `OP1_TEB` writer - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP1_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEB` reader - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP2_TEB_R = crate::BitReader;
#[doc = "Field `OP2_TEB` writer - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type OP2_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0` reader - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0_R = crate::BitReader;
#[doc = "Field `F0` writer - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1` reader - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1_R = crate::BitReader;
#[doc = "Field `F1` writer - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2` reader - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2_R = crate::BitReader;
#[doc = "Field `F2` writer - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0_CLR` reader - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0_CLR_R = crate::BitReader;
#[doc = "Field `F0_CLR` writer - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1_CLR` reader - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1_CLR_R = crate::BitReader;
#[doc = "Field `F1_CLR` writer - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2_CLR` reader - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2_CLR_R = crate::BitReader;
#[doc = "Field `F2_CLR` writer - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_CBC` reader - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_CBC` writer - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_CBC` reader - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_CBC_R = crate::BitReader;
#[doc = "Field `TZ1_CBC` writer - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_CBC` reader - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_CBC_R = crate::BitReader;
#[doc = "Field `TZ2_CBC` writer - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_OST` reader - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_OST_R = crate::BitReader;
#[doc = "Field `TZ0_OST` writer - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_OST` reader - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_OST_R = crate::BitReader;
#[doc = "Field `TZ1_OST` writer - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_OST` reader - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_OST_R = crate::BitReader;
#[doc = "Field `TZ2_OST` writer - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type TZ2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP0_R = crate::BitReader;
#[doc = "Field `CAP0` writer - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1` reader - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP1_R = crate::BitReader;
#[doc = "Field `CAP1` writer - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2` reader - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP2_R = crate::BitReader;
#[doc = "Field `CAP2` writer - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type CAP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_stop(&self) -> TIMER0_STOP_R {
        TIMER0_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_stop(&self) -> TIMER1_STOP_R {
        TIMER1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_stop(&self) -> TIMER2_STOP_R {
        TIMER2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tez(&self) -> TIMER0_TEZ_R {
        TIMER0_TEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tez(&self) -> TIMER1_TEZ_R {
        TIMER1_TEZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tez(&self) -> TIMER2_TEZ_R {
        TIMER2_TEZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tep(&self) -> TIMER0_TEP_R {
        TIMER0_TEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tep(&self) -> TIMER1_TEP_R {
        TIMER1_TEP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tep(&self) -> TIMER2_TEP_R {
        TIMER2_TEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tea(&self) -> OP0_TEA_R {
        OP0_TEA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tea(&self) -> OP1_TEA_R {
        OP1_TEA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tea(&self) -> OP2_TEA_R {
        OP2_TEA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_teb(&self) -> OP0_TEB_R {
        OP0_TEB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_teb(&self) -> OP1_TEB_R {
        OP1_TEB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_teb(&self) -> OP2_TEB_R {
        OP2_TEB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0(&self) -> F0_R {
        F0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1(&self) -> F1_R {
        F1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2(&self) -> F2_R {
        F2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_clr(&self) -> F0_CLR_R {
        F0_CLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_clr(&self) -> F1_CLR_R {
        F1_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_clr(&self) -> F2_CLR_R {
        F2_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_cbc(&self) -> TZ0_CBC_R {
        TZ0_CBC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_cbc(&self) -> TZ1_CBC_R {
        TZ1_CBC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_cbc(&self) -> TZ2_CBC_R {
        TZ2_CBC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_ost(&self) -> TZ0_OST_R {
        TZ0_OST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_ost(&self) -> TZ1_OST_R {
        TZ1_OST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_ost(&self) -> TZ2_OST_R {
        TZ2_OST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap1(&self) -> CAP1_R {
        CAP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap2(&self) -> CAP2_R {
        CAP2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_EN")
            .field("timer0_stop", &self.timer0_stop())
            .field("timer1_stop", &self.timer1_stop())
            .field("timer2_stop", &self.timer2_stop())
            .field("timer0_tez", &self.timer0_tez())
            .field("timer1_tez", &self.timer1_tez())
            .field("timer2_tez", &self.timer2_tez())
            .field("timer0_tep", &self.timer0_tep())
            .field("timer1_tep", &self.timer1_tep())
            .field("timer2_tep", &self.timer2_tep())
            .field("op0_tea", &self.op0_tea())
            .field("op1_tea", &self.op1_tea())
            .field("op2_tea", &self.op2_tea())
            .field("op0_teb", &self.op0_teb())
            .field("op1_teb", &self.op1_teb())
            .field("op2_teb", &self.op2_teb())
            .field("f0", &self.f0())
            .field("f1", &self.f1())
            .field("f2", &self.f2())
            .field("f0_clr", &self.f0_clr())
            .field("f1_clr", &self.f1_clr())
            .field("f2_clr", &self.f2_clr())
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
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_stop(&mut self) -> TIMER0_STOP_W<'_, EVT_EN_SPEC> {
        TIMER0_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_stop(&mut self) -> TIMER1_STOP_W<'_, EVT_EN_SPEC> {
        TIMER1_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_stop(&mut self) -> TIMER2_STOP_W<'_, EVT_EN_SPEC> {
        TIMER2_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tez(&mut self) -> TIMER0_TEZ_W<'_, EVT_EN_SPEC> {
        TIMER0_TEZ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tez(&mut self) -> TIMER1_TEZ_W<'_, EVT_EN_SPEC> {
        TIMER1_TEZ_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tez(&mut self) -> TIMER2_TEZ_W<'_, EVT_EN_SPEC> {
        TIMER2_TEZ_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tep(&mut self) -> TIMER0_TEP_W<'_, EVT_EN_SPEC> {
        TIMER0_TEP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tep(&mut self) -> TIMER1_TEP_W<'_, EVT_EN_SPEC> {
        TIMER1_TEP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tep(&mut self) -> TIMER2_TEP_W<'_, EVT_EN_SPEC> {
        TIMER2_TEP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tea(&mut self) -> OP0_TEA_W<'_, EVT_EN_SPEC> {
        OP0_TEA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tea(&mut self) -> OP1_TEA_W<'_, EVT_EN_SPEC> {
        OP1_TEA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tea(&mut self) -> OP2_TEA_W<'_, EVT_EN_SPEC> {
        OP2_TEA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_teb(&mut self) -> OP0_TEB_W<'_, EVT_EN_SPEC> {
        OP0_TEB_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_teb(&mut self) -> OP1_TEB_W<'_, EVT_EN_SPEC> {
        OP1_TEB_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_teb(&mut self) -> OP2_TEB_W<'_, EVT_EN_SPEC> {
        OP2_TEB_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0(&mut self) -> F0_W<'_, EVT_EN_SPEC> {
        F0_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1(&mut self) -> F1_W<'_, EVT_EN_SPEC> {
        F1_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2(&mut self) -> F2_W<'_, EVT_EN_SPEC> {
        F2_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_clr(&mut self) -> F0_CLR_W<'_, EVT_EN_SPEC> {
        F0_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_clr(&mut self) -> F1_CLR_W<'_, EVT_EN_SPEC> {
        F1_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_clr(&mut self) -> F2_CLR_W<'_, EVT_EN_SPEC> {
        F2_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_cbc(&mut self) -> TZ0_CBC_W<'_, EVT_EN_SPEC> {
        TZ0_CBC_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_cbc(&mut self) -> TZ1_CBC_W<'_, EVT_EN_SPEC> {
        TZ1_CBC_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_cbc(&mut self) -> TZ2_CBC_W<'_, EVT_EN_SPEC> {
        TZ2_CBC_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_ost(&mut self) -> TZ0_OST_W<'_, EVT_EN_SPEC> {
        TZ0_OST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_ost(&mut self) -> TZ1_OST_W<'_, EVT_EN_SPEC> {
        TZ1_OST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_ost(&mut self) -> TZ2_OST_W<'_, EVT_EN_SPEC> {
        TZ2_OST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap0(&mut self) -> CAP0_W<'_, EVT_EN_SPEC> {
        CAP0_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap1(&mut self) -> CAP1_W<'_, EVT_EN_SPEC> {
        CAP1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap2(&mut self) -> CAP2_W<'_, EVT_EN_SPEC> {
        CAP2_W::new(self, 29)
    }
}
#[doc = "Event enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
