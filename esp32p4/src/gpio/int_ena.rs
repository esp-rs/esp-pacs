#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `COMP0_NEG_INT_ENA` reader - analog comparator pos edge interrupt enable"]
pub type COMP0_NEG_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP0_NEG_INT_ENA` writer - analog comparator pos edge interrupt enable"]
pub type COMP0_NEG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_POS_INT_ENA` reader - analog comparator neg edge interrupt enable"]
pub type COMP0_POS_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP0_POS_INT_ENA` writer - analog comparator neg edge interrupt enable"]
pub type COMP0_POS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_ALL_INT_ENA` reader - analog comparator neg or pos edge interrupt enable"]
pub type COMP0_ALL_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP0_ALL_INT_ENA` writer - analog comparator neg or pos edge interrupt enable"]
pub type COMP0_ALL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_NEG_INT_ENA` reader - analog comparator pos edge interrupt enable"]
pub type COMP1_NEG_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP1_NEG_INT_ENA` writer - analog comparator pos edge interrupt enable"]
pub type COMP1_NEG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POS_INT_ENA` reader - analog comparator neg edge interrupt enable"]
pub type COMP1_POS_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP1_POS_INT_ENA` writer - analog comparator neg edge interrupt enable"]
pub type COMP1_POS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_ALL_INT_ENA` reader - analog comparator neg or pos edge interrupt enable"]
pub type COMP1_ALL_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP1_ALL_INT_ENA` writer - analog comparator neg or pos edge interrupt enable"]
pub type COMP1_ALL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTOK_INT_ENA` reader - pad bistok interrupt enable"]
pub type BISTOK_INT_ENA_R = crate::BitReader;
#[doc = "Field `BISTOK_INT_ENA` writer - pad bistok interrupt enable"]
pub type BISTOK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTFAIL_INT_ENA` reader - pad bistfail interrupt enable"]
pub type BISTFAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `BISTFAIL_INT_ENA` writer - pad bistfail interrupt enable"]
pub type BISTFAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_neg_int_ena(&self) -> COMP0_NEG_INT_ENA_R {
        COMP0_NEG_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_pos_int_ena(&self) -> COMP0_POS_INT_ENA_R {
        COMP0_POS_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp0_all_int_ena(&self) -> COMP0_ALL_INT_ENA_R {
        COMP0_ALL_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_neg_int_ena(&self) -> COMP1_NEG_INT_ENA_R {
        COMP1_NEG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_pos_int_ena(&self) -> COMP1_POS_INT_ENA_R {
        COMP1_POS_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp1_all_int_ena(&self) -> COMP1_ALL_INT_ENA_R {
        COMP1_ALL_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    pub fn bistok_int_ena(&self) -> BISTOK_INT_ENA_R {
        BISTOK_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    pub fn bistfail_int_ena(&self) -> BISTFAIL_INT_ENA_R {
        BISTFAIL_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "comp0_neg_int_ena",
                &format_args!("{}", self.comp0_neg_int_ena().bit()),
            )
            .field(
                "comp0_pos_int_ena",
                &format_args!("{}", self.comp0_pos_int_ena().bit()),
            )
            .field(
                "comp0_all_int_ena",
                &format_args!("{}", self.comp0_all_int_ena().bit()),
            )
            .field(
                "comp1_neg_int_ena",
                &format_args!("{}", self.comp1_neg_int_ena().bit()),
            )
            .field(
                "comp1_pos_int_ena",
                &format_args!("{}", self.comp1_pos_int_ena().bit()),
            )
            .field(
                "comp1_all_int_ena",
                &format_args!("{}", self.comp1_all_int_ena().bit()),
            )
            .field(
                "bistok_int_ena",
                &format_args!("{}", self.bistok_int_ena().bit()),
            )
            .field(
                "bistfail_int_ena",
                &format_args!("{}", self.bistfail_int_ena().bit()),
            )
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
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_neg_int_ena(&mut self) -> COMP0_NEG_INT_ENA_W<INT_ENA_SPEC> {
        COMP0_NEG_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_pos_int_ena(&mut self) -> COMP0_POS_INT_ENA_W<INT_ENA_SPEC> {
        COMP0_POS_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_all_int_ena(&mut self) -> COMP0_ALL_INT_ENA_W<INT_ENA_SPEC> {
        COMP0_ALL_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_neg_int_ena(&mut self) -> COMP1_NEG_INT_ENA_W<INT_ENA_SPEC> {
        COMP1_NEG_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_pos_int_ena(&mut self) -> COMP1_POS_INT_ENA_W<INT_ENA_SPEC> {
        COMP1_POS_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_all_int_ena(&mut self) -> COMP1_ALL_INT_ENA_W<INT_ENA_SPEC> {
        COMP1_ALL_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bistok_int_ena(&mut self) -> BISTOK_INT_ENA_W<INT_ENA_SPEC> {
        BISTOK_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bistfail_int_ena(&mut self) -> BISTFAIL_INT_ENA_W<INT_ENA_SPEC> {
        BISTFAIL_INT_ENA_W::new(self, 7)
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
#[doc = "analog comparator interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0xff"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
