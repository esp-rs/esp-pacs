#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TIMER0_STOP_INT_CLR` writer - "]
pub type TIMER0_STOP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_STOP_INT_CLR` writer - "]
pub type TIMER1_STOP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_STOP_INT_CLR` writer - "]
pub type TIMER2_STOP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEZ_INT_CLR` writer - "]
pub type TIMER0_TEZ_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEZ_INT_CLR` writer - "]
pub type TIMER1_TEZ_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEZ_INT_CLR` writer - "]
pub type TIMER2_TEZ_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEP_INT_CLR` writer - "]
pub type TIMER0_TEP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEP_INT_CLR` writer - "]
pub type TIMER1_TEP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEP_INT_CLR` writer - "]
pub type TIMER2_TEP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0_INT_CLR` writer - "]
pub type FAULT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1_INT_CLR` writer - "]
pub type FAULT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2_INT_CLR` writer - "]
pub type FAULT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0_CLR_INT_CLR` writer - "]
pub type FAULT0_CLR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1_CLR_INT_CLR` writer - "]
pub type FAULT1_CLR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2_CLR_INT_CLR` writer - "]
pub type FAULT2_CLR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEA_INT_CLR` writer - "]
pub type OP0_TEA_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEA_INT_CLR` writer - "]
pub type OP1_TEA_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEA_INT_CLR` writer - "]
pub type OP2_TEA_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEB_INT_CLR` writer - "]
pub type OP0_TEB_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEB_INT_CLR` writer - "]
pub type OP1_TEB_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEB_INT_CLR` writer - "]
pub type OP2_TEB_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH0_CBC_INT_CLR` writer - "]
pub type FH0_CBC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH1_CBC_INT_CLR` writer - "]
pub type FH1_CBC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_CBC_INT_CLR` writer - "]
pub type FH2_CBC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH0_OST_INT_CLR` writer - "]
pub type FH0_OST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH1_OST_INT_CLR` writer - "]
pub type FH1_OST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_OST_INT_CLR` writer - "]
pub type FH2_OST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0_INT_CLR` writer - "]
pub type CAP0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1_INT_CLR` writer - "]
pub type CAP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2_INT_CLR` writer - "]
pub type CAP2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn timer0_stop_int_clr(&mut self) -> TIMER0_STOP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER0_STOP_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_stop_int_clr(&mut self) -> TIMER1_STOP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER1_STOP_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_stop_int_clr(&mut self) -> TIMER2_STOP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER2_STOP_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tez_int_clr(&mut self) -> TIMER0_TEZ_INT_CLR_W<INT_CLR_SPEC> {
        TIMER0_TEZ_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tez_int_clr(&mut self) -> TIMER1_TEZ_INT_CLR_W<INT_CLR_SPEC> {
        TIMER1_TEZ_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tez_int_clr(&mut self) -> TIMER2_TEZ_INT_CLR_W<INT_CLR_SPEC> {
        TIMER2_TEZ_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_tep_int_clr(&mut self) -> TIMER0_TEP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER0_TEP_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_tep_int_clr(&mut self) -> TIMER1_TEP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER1_TEP_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_tep_int_clr(&mut self) -> TIMER2_TEP_INT_CLR_W<INT_CLR_SPEC> {
        TIMER2_TEP_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_int_clr(&mut self) -> FAULT0_INT_CLR_W<INT_CLR_SPEC> {
        FAULT0_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_int_clr(&mut self) -> FAULT1_INT_CLR_W<INT_CLR_SPEC> {
        FAULT1_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_int_clr(&mut self) -> FAULT2_INT_CLR_W<INT_CLR_SPEC> {
        FAULT2_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn fault0_clr_int_clr(&mut self) -> FAULT0_CLR_INT_CLR_W<INT_CLR_SPEC> {
        FAULT0_CLR_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn fault1_clr_int_clr(&mut self) -> FAULT1_CLR_INT_CLR_W<INT_CLR_SPEC> {
        FAULT1_CLR_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn fault2_clr_int_clr(&mut self) -> FAULT2_CLR_INT_CLR_W<INT_CLR_SPEC> {
        FAULT2_CLR_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn op0_tea_int_clr(&mut self) -> OP0_TEA_INT_CLR_W<INT_CLR_SPEC> {
        OP0_TEA_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn op1_tea_int_clr(&mut self) -> OP1_TEA_INT_CLR_W<INT_CLR_SPEC> {
        OP1_TEA_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn op2_tea_int_clr(&mut self) -> OP2_TEA_INT_CLR_W<INT_CLR_SPEC> {
        OP2_TEA_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn op0_teb_int_clr(&mut self) -> OP0_TEB_INT_CLR_W<INT_CLR_SPEC> {
        OP0_TEB_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn op1_teb_int_clr(&mut self) -> OP1_TEB_INT_CLR_W<INT_CLR_SPEC> {
        OP1_TEB_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn op2_teb_int_clr(&mut self) -> OP2_TEB_INT_CLR_W<INT_CLR_SPEC> {
        OP2_TEB_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_cbc_int_clr(&mut self) -> FH0_CBC_INT_CLR_W<INT_CLR_SPEC> {
        FH0_CBC_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_cbc_int_clr(&mut self) -> FH1_CBC_INT_CLR_W<INT_CLR_SPEC> {
        FH1_CBC_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_cbc_int_clr(&mut self) -> FH2_CBC_INT_CLR_W<INT_CLR_SPEC> {
        FH2_CBC_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_ost_int_clr(&mut self) -> FH0_OST_INT_CLR_W<INT_CLR_SPEC> {
        FH0_OST_INT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_ost_int_clr(&mut self) -> FH1_OST_INT_CLR_W<INT_CLR_SPEC> {
        FH1_OST_INT_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_ost_int_clr(&mut self) -> FH2_OST_INT_CLR_W<INT_CLR_SPEC> {
        FH2_OST_INT_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_int_clr(&mut self) -> CAP0_INT_CLR_W<INT_CLR_SPEC> {
        CAP0_INT_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cap1_int_clr(&mut self) -> CAP1_INT_CLR_W<INT_CLR_SPEC> {
        CAP1_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_int_clr(&mut self) -> CAP2_INT_CLR_W<INT_CLR_SPEC> {
        CAP2_INT_CLR_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
