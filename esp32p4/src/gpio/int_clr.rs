#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `COMP0_NEG_INT_CLR` writer - analog comparator pos edge interrupt clear"]
pub type COMP0_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_POS_INT_CLR` writer - analog comparator neg edge interrupt clear"]
pub type COMP0_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_ALL_INT_CLR` writer - analog comparator neg or pos edge interrupt clear"]
pub type COMP0_ALL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_NEG_INT_CLR` writer - analog comparator pos edge interrupt clear"]
pub type COMP1_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POS_INT_CLR` writer - analog comparator neg edge interrupt clear"]
pub type COMP1_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_ALL_INT_CLR` writer - analog comparator neg or pos edge interrupt clear"]
pub type COMP1_ALL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTOK_INT_CLR` writer - pad bistok interrupt enable"]
pub type BISTOK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTFAIL_INT_CLR` writer - pad bistfail interrupt enable"]
pub type BISTFAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_neg_int_clr(&mut self) -> COMP0_NEG_INT_CLR_W<INT_CLR_SPEC> {
        COMP0_NEG_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_pos_int_clr(&mut self) -> COMP0_POS_INT_CLR_W<INT_CLR_SPEC> {
        COMP0_POS_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_all_int_clr(&mut self) -> COMP0_ALL_INT_CLR_W<INT_CLR_SPEC> {
        COMP0_ALL_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_neg_int_clr(&mut self) -> COMP1_NEG_INT_CLR_W<INT_CLR_SPEC> {
        COMP1_NEG_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_pos_int_clr(&mut self) -> COMP1_POS_INT_CLR_W<INT_CLR_SPEC> {
        COMP1_POS_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_all_int_clr(&mut self) -> COMP1_ALL_INT_CLR_W<INT_CLR_SPEC> {
        COMP1_ALL_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bistok_int_clr(&mut self) -> BISTOK_INT_CLR_W<INT_CLR_SPEC> {
        BISTOK_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bistfail_int_clr(&mut self) -> BISTFAIL_INT_CLR_W<INT_CLR_SPEC> {
        BISTFAIL_INT_CLR_W::new(self, 7)
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
#[doc = "analog comparator interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
