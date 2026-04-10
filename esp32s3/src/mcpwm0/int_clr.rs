#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TIMER_STOP(0-2)` writer - The interrupt clear bit for the timer%s stop event."]
pub type TIMER_STOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER_TEZ(0-2)` writer - The interrupt clear bit for the timer%s zero event."]
pub type TIMER_TEZ_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER_TEP(0-2)` writer - The interrupt clear bit for the timer%s period event."]
pub type TIMER_TEP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT(0-2)` writer - The interrupt clear bit for the fault%s event."]
pub type FAULT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT_CLR(0-2)` writer - The interrupt clear bit for the fault%s clear event."]
pub type FAULT_CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMPR_TEA(0-2)` writer - The interrupt clear bit for the operator%s compare A event."]
pub type CMPR_TEA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMPR_TEB(0-2)` writer - The interrupt clear bit for the operator%s compare B event."]
pub type CMPR_TEB_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TZ_CBC(0-2)` writer - The interrupt clear bit for the fault handler%s cycle-by-cycle mode action."]
pub type TZ_CBC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TZ_OST(0-2)` writer - The interrupt clear bit for the fault handler%s one-shot mode action."]
pub type TZ_OST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAP(0-2)` writer - The interrupt clear bit for the capture%s event."]
pub type CAP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "The interrupt clear bit for the timer(0-2) stop event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_STOP` field.</div>"]
    #[inline(always)]
    pub fn timer_stop(&mut self, n: u8) -> TIMER_STOP_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_STOP_W::new(self, n)
    }
    #[doc = "Bit 0 - The interrupt clear bit for the timer0 stop event."]
    #[inline(always)]
    pub fn timer0_stop(&mut self) -> TIMER_STOP_W<'_, INT_CLR_SPEC> {
        TIMER_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt clear bit for the timer1 stop event."]
    #[inline(always)]
    pub fn timer1_stop(&mut self) -> TIMER_STOP_W<'_, INT_CLR_SPEC> {
        TIMER_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt clear bit for the timer2 stop event."]
    #[inline(always)]
    pub fn timer2_stop(&mut self) -> TIMER_STOP_W<'_, INT_CLR_SPEC> {
        TIMER_STOP_W::new(self, 2)
    }
    #[doc = "The interrupt clear bit for the timer(0-2) zero event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEZ` field.</div>"]
    #[inline(always)]
    pub fn timer_tez(&mut self, n: u8) -> TIMER_TEZ_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEZ_W::new(self, n + 3)
    }
    #[doc = "Bit 3 - The interrupt clear bit for the timer0 zero event."]
    #[inline(always)]
    pub fn timer0_tez(&mut self) -> TIMER_TEZ_W<'_, INT_CLR_SPEC> {
        TIMER_TEZ_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt clear bit for the timer1 zero event."]
    #[inline(always)]
    pub fn timer1_tez(&mut self) -> TIMER_TEZ_W<'_, INT_CLR_SPEC> {
        TIMER_TEZ_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt clear bit for the timer2 zero event."]
    #[inline(always)]
    pub fn timer2_tez(&mut self) -> TIMER_TEZ_W<'_, INT_CLR_SPEC> {
        TIMER_TEZ_W::new(self, 5)
    }
    #[doc = "The interrupt clear bit for the timer(0-2) period event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_TEP` field.</div>"]
    #[inline(always)]
    pub fn timer_tep(&mut self, n: u8) -> TIMER_TEP_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_TEP_W::new(self, n + 6)
    }
    #[doc = "Bit 6 - The interrupt clear bit for the timer0 period event."]
    #[inline(always)]
    pub fn timer0_tep(&mut self) -> TIMER_TEP_W<'_, INT_CLR_SPEC> {
        TIMER_TEP_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt clear bit for the timer1 period event."]
    #[inline(always)]
    pub fn timer1_tep(&mut self) -> TIMER_TEP_W<'_, INT_CLR_SPEC> {
        TIMER_TEP_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt clear bit for the timer2 period event."]
    #[inline(always)]
    pub fn timer2_tep(&mut self) -> TIMER_TEP_W<'_, INT_CLR_SPEC> {
        TIMER_TEP_W::new(self, 8)
    }
    #[doc = "The interrupt clear bit for the fault(0-2) event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FAULT0` field.</div>"]
    #[inline(always)]
    pub fn fault(&mut self, n: u8) -> FAULT_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        FAULT_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - The interrupt clear bit for the fault0 event."]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT_W<'_, INT_CLR_SPEC> {
        FAULT_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt clear bit for the fault1 event."]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT_W<'_, INT_CLR_SPEC> {
        FAULT_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt clear bit for the fault2 event."]
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT_W<'_, INT_CLR_SPEC> {
        FAULT_W::new(self, 11)
    }
    #[doc = "The interrupt clear bit for the fault(0-2) clear event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FAULT0_CLR` field.</div>"]
    #[inline(always)]
    pub fn fault_clr(&mut self, n: u8) -> FAULT_CLR_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        FAULT_CLR_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - The interrupt clear bit for the fault0 clear event."]
    #[inline(always)]
    pub fn fault0_clr(&mut self) -> FAULT_CLR_W<'_, INT_CLR_SPEC> {
        FAULT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt clear bit for the fault1 clear event."]
    #[inline(always)]
    pub fn fault1_clr(&mut self) -> FAULT_CLR_W<'_, INT_CLR_SPEC> {
        FAULT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt clear bit for the fault2 clear event."]
    #[inline(always)]
    pub fn fault2_clr(&mut self) -> FAULT_CLR_W<'_, INT_CLR_SPEC> {
        FAULT_CLR_W::new(self, 14)
    }
    #[doc = "The interrupt clear bit for the operator(0-2) compare A event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_TEA` field.</div>"]
    #[inline(always)]
    pub fn cmpr_tea(&mut self, n: u8) -> CMPR_TEA_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_TEA_W::new(self, n + 15)
    }
    #[doc = "Bit 15 - The interrupt clear bit for the operator0 compare A event."]
    #[inline(always)]
    pub fn cmpr0_tea(&mut self) -> CMPR_TEA_W<'_, INT_CLR_SPEC> {
        CMPR_TEA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt clear bit for the operator1 compare A event."]
    #[inline(always)]
    pub fn cmpr1_tea(&mut self) -> CMPR_TEA_W<'_, INT_CLR_SPEC> {
        CMPR_TEA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt clear bit for the operator2 compare A event."]
    #[inline(always)]
    pub fn cmpr2_tea(&mut self) -> CMPR_TEA_W<'_, INT_CLR_SPEC> {
        CMPR_TEA_W::new(self, 17)
    }
    #[doc = "The interrupt clear bit for the operator(0-2) compare B event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMPR0_TEB` field.</div>"]
    #[inline(always)]
    pub fn cmpr_teb(&mut self, n: u8) -> CMPR_TEB_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CMPR_TEB_W::new(self, n + 18)
    }
    #[doc = "Bit 18 - The interrupt clear bit for the operator0 compare B event."]
    #[inline(always)]
    pub fn cmpr0_teb(&mut self) -> CMPR_TEB_W<'_, INT_CLR_SPEC> {
        CMPR_TEB_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt clear bit for the operator1 compare B event."]
    #[inline(always)]
    pub fn cmpr1_teb(&mut self) -> CMPR_TEB_W<'_, INT_CLR_SPEC> {
        CMPR_TEB_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt clear bit for the operator2 compare B event."]
    #[inline(always)]
    pub fn cmpr2_teb(&mut self) -> CMPR_TEB_W<'_, INT_CLR_SPEC> {
        CMPR_TEB_W::new(self, 20)
    }
    #[doc = "The interrupt clear bit for the fault handler(0-2) cycle-by-cycle mode action."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_CBC` field.</div>"]
    #[inline(always)]
    pub fn tz_cbc(&mut self, n: u8) -> TZ_CBC_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_CBC_W::new(self, n + 21)
    }
    #[doc = "Bit 21 - The interrupt clear bit for the fault handler0 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz0_cbc(&mut self) -> TZ_CBC_W<'_, INT_CLR_SPEC> {
        TZ_CBC_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt clear bit for the fault handler1 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz1_cbc(&mut self) -> TZ_CBC_W<'_, INT_CLR_SPEC> {
        TZ_CBC_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt clear bit for the fault handler2 cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz2_cbc(&mut self) -> TZ_CBC_W<'_, INT_CLR_SPEC> {
        TZ_CBC_W::new(self, 23)
    }
    #[doc = "The interrupt clear bit for the fault handler(0-2) one-shot mode action."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TZ0_OST` field.</div>"]
    #[inline(always)]
    pub fn tz_ost(&mut self, n: u8) -> TZ_OST_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TZ_OST_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - The interrupt clear bit for the fault handler0 one-shot mode action."]
    #[inline(always)]
    pub fn tz0_ost(&mut self) -> TZ_OST_W<'_, INT_CLR_SPEC> {
        TZ_OST_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt clear bit for the fault handler1 one-shot mode action."]
    #[inline(always)]
    pub fn tz1_ost(&mut self) -> TZ_OST_W<'_, INT_CLR_SPEC> {
        TZ_OST_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt clear bit for the fault handler2 one-shot mode action."]
    #[inline(always)]
    pub fn tz2_ost(&mut self) -> TZ_OST_W<'_, INT_CLR_SPEC> {
        TZ_OST_W::new(self, 26)
    }
    #[doc = "The interrupt clear bit for the capture(0-2) event."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0` field.</div>"]
    #[inline(always)]
    pub fn cap(&mut self, n: u8) -> CAP_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_W::new(self, n + 27)
    }
    #[doc = "Bit 27 - The interrupt clear bit for the capture0 event."]
    #[inline(always)]
    pub fn cap0(&mut self) -> CAP_W<'_, INT_CLR_SPEC> {
        CAP_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt clear bit for the capture1 event."]
    #[inline(always)]
    pub fn cap1(&mut self) -> CAP_W<'_, INT_CLR_SPEC> {
        CAP_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt clear bit for the capture2 event."]
    #[inline(always)]
    pub fn cap2(&mut self) -> CAP_W<'_, INT_CLR_SPEC> {
        CAP_W::new(self, 29)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3fff_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
