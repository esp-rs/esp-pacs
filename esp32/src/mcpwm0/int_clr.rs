#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TIMER0_STOP` writer - "]
pub type TIMER0_STOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER1_STOP` writer - "]
pub type TIMER1_STOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER2_STOP` writer - "]
pub type TIMER2_STOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER0_TEZ` writer - "]
pub type TIMER0_TEZ_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER1_TEZ` writer - "]
pub type TIMER1_TEZ_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER2_TEZ` writer - "]
pub type TIMER2_TEZ_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER0_TEP` writer - "]
pub type TIMER0_TEP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER1_TEP` writer - "]
pub type TIMER1_TEP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER2_TEP` writer - "]
pub type TIMER2_TEP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT0` writer - "]
pub type FAULT0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT1` writer - "]
pub type FAULT1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT2` writer - "]
pub type FAULT2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT0_CLR` writer - "]
pub type FAULT0_CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT1_CLR` writer - "]
pub type FAULT1_CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FAULT2_CLR` writer - "]
pub type FAULT2_CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP0_TEA` writer - "]
pub type OP0_TEA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP1_TEA` writer - "]
pub type OP1_TEA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP2_TEA` writer - "]
pub type OP2_TEA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP0_TEB` writer - "]
pub type OP0_TEB_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP1_TEB` writer - "]
pub type OP1_TEB_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OP2_TEB` writer - "]
pub type OP2_TEB_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH0_CBC` writer - "]
pub type FH0_CBC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH1_CBC` writer - "]
pub type FH1_CBC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH2_CBC` writer - "]
pub type FH2_CBC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH0_OST` writer - "]
pub type FH0_OST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH1_OST` writer - "]
pub type FH1_OST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FH2_OST` writer - "]
pub type FH2_OST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAP0` writer - "]
pub type CAP0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAP1` writer - "]
pub type CAP1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAP2` writer - "]
pub type CAP2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_stop(&mut self) -> TIMER0_STOP_W<INT_CLR_SPEC> {
        TIMER0_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop(&mut self) -> TIMER1_STOP_W<INT_CLR_SPEC> {
        TIMER1_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop(&mut self) -> TIMER2_STOP_W<INT_CLR_SPEC> {
        TIMER2_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez(&mut self) -> TIMER0_TEZ_W<INT_CLR_SPEC> {
        TIMER0_TEZ_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez(&mut self) -> TIMER1_TEZ_W<INT_CLR_SPEC> {
        TIMER1_TEZ_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez(&mut self) -> TIMER2_TEZ_W<INT_CLR_SPEC> {
        TIMER2_TEZ_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep(&mut self) -> TIMER0_TEP_W<INT_CLR_SPEC> {
        TIMER0_TEP_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep(&mut self) -> TIMER1_TEP_W<INT_CLR_SPEC> {
        TIMER1_TEP_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep(&mut self) -> TIMER2_TEP_W<INT_CLR_SPEC> {
        TIMER2_TEP_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> FAULT0_W<INT_CLR_SPEC> {
        FAULT0_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<INT_CLR_SPEC> {
        FAULT1_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<INT_CLR_SPEC> {
        FAULT2_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr(&mut self) -> FAULT0_CLR_W<INT_CLR_SPEC> {
        FAULT0_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr(&mut self) -> FAULT1_CLR_W<INT_CLR_SPEC> {
        FAULT1_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr(&mut self) -> FAULT2_CLR_W<INT_CLR_SPEC> {
        FAULT2_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn op0_tea(&mut self) -> OP0_TEA_W<INT_CLR_SPEC> {
        OP0_TEA_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn op1_tea(&mut self) -> OP1_TEA_W<INT_CLR_SPEC> {
        OP1_TEA_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn op2_tea(&mut self) -> OP2_TEA_W<INT_CLR_SPEC> {
        OP2_TEA_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn op0_teb(&mut self) -> OP0_TEB_W<INT_CLR_SPEC> {
        OP0_TEB_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn op1_teb(&mut self) -> OP1_TEB_W<INT_CLR_SPEC> {
        OP1_TEB_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn op2_teb(&mut self) -> OP2_TEB_W<INT_CLR_SPEC> {
        OP2_TEB_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_cbc(&mut self) -> FH0_CBC_W<INT_CLR_SPEC> {
        FH0_CBC_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_cbc(&mut self) -> FH1_CBC_W<INT_CLR_SPEC> {
        FH1_CBC_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_cbc(&mut self) -> FH2_CBC_W<INT_CLR_SPEC> {
        FH2_CBC_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_ost(&mut self) -> FH0_OST_W<INT_CLR_SPEC> {
        FH0_OST_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_ost(&mut self) -> FH1_OST_W<INT_CLR_SPEC> {
        FH1_OST_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_ost(&mut self) -> FH2_OST_W<INT_CLR_SPEC> {
        FH2_OST_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cap0(&mut self) -> CAP0_W<INT_CLR_SPEC> {
        CAP0_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cap1(&mut self) -> CAP1_W<INT_CLR_SPEC> {
        CAP1_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cap2(&mut self) -> CAP2_W<INT_CLR_SPEC> {
        CAP2_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3fff_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
