#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TIMER0_STOP` reader - "]
pub type TIMER0_STOP_R = crate::BitReader;
#[doc = "Field `TIMER0_STOP` writer - "]
pub type TIMER0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_STOP` reader - "]
pub type TIMER1_STOP_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP` writer - "]
pub type TIMER1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_STOP` reader - "]
pub type TIMER2_STOP_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP` writer - "]
pub type TIMER2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEZ` reader - "]
pub type TIMER0_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` writer - "]
pub type TIMER0_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEZ` reader - "]
pub type TIMER1_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` writer - "]
pub type TIMER1_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEZ` reader - "]
pub type TIMER2_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` writer - "]
pub type TIMER2_TEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEP` reader - "]
pub type TIMER0_TEP_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP` writer - "]
pub type TIMER0_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEP` reader - "]
pub type TIMER1_TEP_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP` writer - "]
pub type TIMER1_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEP` reader - "]
pub type TIMER2_TEP_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP` writer - "]
pub type TIMER2_TEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0` reader - "]
pub type FAULT0_R = crate::BitReader;
#[doc = "Field `FAULT0` writer - "]
pub type FAULT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - "]
pub type FAULT1_R = crate::BitReader;
#[doc = "Field `FAULT1` writer - "]
pub type FAULT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2` reader - "]
pub type FAULT2_R = crate::BitReader;
#[doc = "Field `FAULT2` writer - "]
pub type FAULT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0_CLR` reader - "]
pub type FAULT0_CLR_R = crate::BitReader;
#[doc = "Field `FAULT0_CLR` writer - "]
pub type FAULT0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1_CLR` reader - "]
pub type FAULT1_CLR_R = crate::BitReader;
#[doc = "Field `FAULT1_CLR` writer - "]
pub type FAULT1_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2_CLR` reader - "]
pub type FAULT2_CLR_R = crate::BitReader;
#[doc = "Field `FAULT2_CLR` writer - "]
pub type FAULT2_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEA` reader - "]
pub type OP0_TEA_R = crate::BitReader;
#[doc = "Field `OP0_TEA` writer - "]
pub type OP0_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEA` reader - "]
pub type OP1_TEA_R = crate::BitReader;
#[doc = "Field `OP1_TEA` writer - "]
pub type OP1_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEA` reader - "]
pub type OP2_TEA_R = crate::BitReader;
#[doc = "Field `OP2_TEA` writer - "]
pub type OP2_TEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEB` reader - "]
pub type OP0_TEB_R = crate::BitReader;
#[doc = "Field `OP0_TEB` writer - "]
pub type OP0_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEB` reader - "]
pub type OP1_TEB_R = crate::BitReader;
#[doc = "Field `OP1_TEB` writer - "]
pub type OP1_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEB` reader - "]
pub type OP2_TEB_R = crate::BitReader;
#[doc = "Field `OP2_TEB` writer - "]
pub type OP2_TEB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH0_CBC` reader - "]
pub type FH0_CBC_R = crate::BitReader;
#[doc = "Field `FH0_CBC` writer - "]
pub type FH0_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH1_CBC` reader - "]
pub type FH1_CBC_R = crate::BitReader;
#[doc = "Field `FH1_CBC` writer - "]
pub type FH1_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_CBC` reader - "]
pub type FH2_CBC_R = crate::BitReader;
#[doc = "Field `FH2_CBC` writer - "]
pub type FH2_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH0_OST` reader - "]
pub type FH0_OST_R = crate::BitReader;
#[doc = "Field `FH0_OST` writer - "]
pub type FH0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH1_OST` reader - "]
pub type FH1_OST_R = crate::BitReader;
#[doc = "Field `FH1_OST` writer - "]
pub type FH1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_OST` reader - "]
pub type FH2_OST_R = crate::BitReader;
#[doc = "Field `FH2_OST` writer - "]
pub type FH2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - "]
pub type CAP0_R = crate::BitReader;
#[doc = "Field `CAP0` writer - "]
pub type CAP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1` reader - "]
pub type CAP1_R = crate::BitReader;
#[doc = "Field `CAP1` writer - "]
pub type CAP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2` reader - "]
pub type CAP2_R = crate::BitReader;
#[doc = "Field `CAP2` writer - "]
pub type CAP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop(&self) -> TIMER0_STOP_R {
        TIMER0_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop(&self) -> TIMER1_STOP_R {
        TIMER1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop(&self) -> TIMER2_STOP_R {
        TIMER2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez(&self) -> TIMER0_TEZ_R {
        TIMER0_TEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez(&self) -> TIMER1_TEZ_R {
        TIMER1_TEZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez(&self) -> TIMER2_TEZ_R {
        TIMER2_TEZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep(&self) -> TIMER0_TEP_R {
        TIMER0_TEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep(&self) -> TIMER1_TEP_R {
        TIMER1_TEP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep(&self) -> TIMER2_TEP_R {
        TIMER2_TEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr(&self) -> FAULT0_CLR_R {
        FAULT0_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr(&self) -> FAULT1_CLR_R {
        FAULT1_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr(&self) -> FAULT2_CLR_R {
        FAULT2_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea(&self) -> OP0_TEA_R {
        OP0_TEA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea(&self) -> OP1_TEA_R {
        OP1_TEA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea(&self) -> OP2_TEA_R {
        OP2_TEA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb(&self) -> OP0_TEB_R {
        OP0_TEB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb(&self) -> OP1_TEB_R {
        OP1_TEB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb(&self) -> OP2_TEB_R {
        OP2_TEB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc(&self) -> FH0_CBC_R {
        FH0_CBC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc(&self) -> FH1_CBC_R {
        FH1_CBC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc(&self) -> FH2_CBC_R {
        FH2_CBC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost(&self) -> FH0_OST_R {
        FH0_OST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost(&self) -> FH1_OST_R {
        FH1_OST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost(&self) -> FH2_OST_R {
        FH2_OST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1(&self) -> CAP1_R {
        CAP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2(&self) -> CAP2_R {
        CAP2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("timer0_stop", &format_args!("{}", self.timer0_stop().bit()))
            .field("timer1_stop", &format_args!("{}", self.timer1_stop().bit()))
            .field("timer2_stop", &format_args!("{}", self.timer2_stop().bit()))
            .field("timer0_tez", &format_args!("{}", self.timer0_tez().bit()))
            .field("timer1_tez", &format_args!("{}", self.timer1_tez().bit()))
            .field("timer2_tez", &format_args!("{}", self.timer2_tez().bit()))
            .field("timer0_tep", &format_args!("{}", self.timer0_tep().bit()))
            .field("timer1_tep", &format_args!("{}", self.timer1_tep().bit()))
            .field("timer2_tep", &format_args!("{}", self.timer2_tep().bit()))
            .field("fault0", &format_args!("{}", self.fault0().bit()))
            .field("fault1", &format_args!("{}", self.fault1().bit()))
            .field("fault2", &format_args!("{}", self.fault2().bit()))
            .field("fault0_clr", &format_args!("{}", self.fault0_clr().bit()))
            .field("fault1_clr", &format_args!("{}", self.fault1_clr().bit()))
            .field("fault2_clr", &format_args!("{}", self.fault2_clr().bit()))
            .field("op0_tea", &format_args!("{}", self.op0_tea().bit()))
            .field("op1_tea", &format_args!("{}", self.op1_tea().bit()))
            .field("op2_tea", &format_args!("{}", self.op2_tea().bit()))
            .field("op0_teb", &format_args!("{}", self.op0_teb().bit()))
            .field("op1_teb", &format_args!("{}", self.op1_teb().bit()))
            .field("op2_teb", &format_args!("{}", self.op2_teb().bit()))
            .field("fh0_cbc", &format_args!("{}", self.fh0_cbc().bit()))
            .field("fh1_cbc", &format_args!("{}", self.fh1_cbc().bit()))
            .field("fh2_cbc", &format_args!("{}", self.fh2_cbc().bit()))
            .field("fh0_ost", &format_args!("{}", self.fh0_ost().bit()))
            .field("fh1_ost", &format_args!("{}", self.fh1_ost().bit()))
            .field("fh2_ost", &format_args!("{}", self.fh2_ost().bit()))
            .field("cap0", &format_args!("{}", self.cap0().bit()))
            .field("cap1", &format_args!("{}", self.cap1().bit()))
            .field("cap2", &format_args!("{}", self.cap2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_stop(&mut self) -> TIMER0_STOP_W<INT_ENA_SPEC> {
        TIMER0_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop(&mut self) -> TIMER1_STOP_W<INT_ENA_SPEC> {
        TIMER1_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop(&mut self) -> TIMER2_STOP_W<INT_ENA_SPEC> {
        TIMER2_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez(&mut self) -> TIMER0_TEZ_W<INT_ENA_SPEC> {
        TIMER0_TEZ_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez(&mut self) -> TIMER1_TEZ_W<INT_ENA_SPEC> {
        TIMER1_TEZ_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez(&mut self) -> TIMER2_TEZ_W<INT_ENA_SPEC> {
        TIMER2_TEZ_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep(&mut self) -> TIMER0_TEP_W<INT_ENA_SPEC> {
        TIMER0_TEP_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep(&mut self) -> TIMER1_TEP_W<INT_ENA_SPEC> {
        TIMER1_TEP_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep(&mut self) -> TIMER2_TEP_W<INT_ENA_SPEC> {
        TIMER2_TEP_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> FAULT0_W<INT_ENA_SPEC> {
        FAULT0_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<INT_ENA_SPEC> {
        FAULT1_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<INT_ENA_SPEC> {
        FAULT2_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr(&mut self) -> FAULT0_CLR_W<INT_ENA_SPEC> {
        FAULT0_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr(&mut self) -> FAULT1_CLR_W<INT_ENA_SPEC> {
        FAULT1_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr(&mut self) -> FAULT2_CLR_W<INT_ENA_SPEC> {
        FAULT2_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn op0_tea(&mut self) -> OP0_TEA_W<INT_ENA_SPEC> {
        OP0_TEA_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn op1_tea(&mut self) -> OP1_TEA_W<INT_ENA_SPEC> {
        OP1_TEA_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn op2_tea(&mut self) -> OP2_TEA_W<INT_ENA_SPEC> {
        OP2_TEA_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn op0_teb(&mut self) -> OP0_TEB_W<INT_ENA_SPEC> {
        OP0_TEB_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn op1_teb(&mut self) -> OP1_TEB_W<INT_ENA_SPEC> {
        OP1_TEB_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn op2_teb(&mut self) -> OP2_TEB_W<INT_ENA_SPEC> {
        OP2_TEB_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_cbc(&mut self) -> FH0_CBC_W<INT_ENA_SPEC> {
        FH0_CBC_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_cbc(&mut self) -> FH1_CBC_W<INT_ENA_SPEC> {
        FH1_CBC_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_cbc(&mut self) -> FH2_CBC_W<INT_ENA_SPEC> {
        FH2_CBC_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_ost(&mut self) -> FH0_OST_W<INT_ENA_SPEC> {
        FH0_OST_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_ost(&mut self) -> FH1_OST_W<INT_ENA_SPEC> {
        FH1_OST_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_ost(&mut self) -> FH2_OST_W<INT_ENA_SPEC> {
        FH2_OST_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cap0(&mut self) -> CAP0_W<INT_ENA_SPEC> {
        CAP0_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cap1(&mut self) -> CAP1_W<INT_ENA_SPEC> {
        CAP1_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cap2(&mut self) -> CAP2_W<INT_ENA_SPEC> {
        CAP2_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
