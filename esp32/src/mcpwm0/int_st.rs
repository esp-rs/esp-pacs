#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TIMER0_STOP` reader - "]
pub type TIMER0_STOP_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP` reader - "]
pub type TIMER1_STOP_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP` reader - "]
pub type TIMER2_STOP_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` reader - "]
pub type TIMER0_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` reader - "]
pub type TIMER1_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` reader - "]
pub type TIMER2_TEZ_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP` reader - "]
pub type TIMER0_TEP_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP` reader - "]
pub type TIMER1_TEP_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP` reader - "]
pub type TIMER2_TEP_R = crate::BitReader;
#[doc = "Field `FAULT0` reader - "]
pub type FAULT0_R = crate::BitReader;
#[doc = "Field `FAULT1` reader - "]
pub type FAULT1_R = crate::BitReader;
#[doc = "Field `FAULT2` reader - "]
pub type FAULT2_R = crate::BitReader;
#[doc = "Field `FAULT0_CLR` reader - "]
pub type FAULT0_CLR_R = crate::BitReader;
#[doc = "Field `FAULT1_CLR` reader - "]
pub type FAULT1_CLR_R = crate::BitReader;
#[doc = "Field `FAULT2_CLR` reader - "]
pub type FAULT2_CLR_R = crate::BitReader;
#[doc = "Field `OP0_TEA` reader - "]
pub type OP0_TEA_R = crate::BitReader;
#[doc = "Field `OP1_TEA` reader - "]
pub type OP1_TEA_R = crate::BitReader;
#[doc = "Field `OP2_TEA` reader - "]
pub type OP2_TEA_R = crate::BitReader;
#[doc = "Field `OP0_TEB` reader - "]
pub type OP0_TEB_R = crate::BitReader;
#[doc = "Field `OP1_TEB` reader - "]
pub type OP1_TEB_R = crate::BitReader;
#[doc = "Field `OP2_TEB` reader - "]
pub type OP2_TEB_R = crate::BitReader;
#[doc = "Field `FH0_CBC` reader - "]
pub type FH0_CBC_R = crate::BitReader;
#[doc = "Field `FH1_CBC` reader - "]
pub type FH1_CBC_R = crate::BitReader;
#[doc = "Field `FH2_CBC` reader - "]
pub type FH2_CBC_R = crate::BitReader;
#[doc = "Field `FH0_OST` reader - "]
pub type FH0_OST_R = crate::BitReader;
#[doc = "Field `FH1_OST` reader - "]
pub type FH1_OST_R = crate::BitReader;
#[doc = "Field `FH2_OST` reader - "]
pub type FH2_OST_R = crate::BitReader;
#[doc = "Field `CAP0` reader - "]
pub type CAP0_R = crate::BitReader;
#[doc = "Field `CAP1` reader - "]
pub type CAP1_R = crate::BitReader;
#[doc = "Field `CAP2` reader - "]
pub type CAP2_R = crate::BitReader;
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
            .field("op0_tea", &self.op0_tea())
            .field("op1_tea", &self.op1_tea())
            .field("op2_tea", &self.op2_tea())
            .field("op0_teb", &self.op0_teb())
            .field("op1_teb", &self.op1_teb())
            .field("op2_teb", &self.op2_teb())
            .field("fh0_cbc", &self.fh0_cbc())
            .field("fh1_cbc", &self.fh1_cbc())
            .field("fh2_cbc", &self.fh2_cbc())
            .field("fh0_ost", &self.fh0_ost())
            .field("fh1_ost", &self.fh1_ost())
            .field("fh2_ost", &self.fh2_ost())
            .field("cap0", &self.cap0())
            .field("cap1", &self.cap1())
            .field("cap2", &self.cap2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
