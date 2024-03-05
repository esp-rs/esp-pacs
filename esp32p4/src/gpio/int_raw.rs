#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `COMP0_NEG_INT_RAW` reader - analog comparator pos edge interrupt raw"]
pub type COMP0_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP0_NEG_INT_RAW` writer - analog comparator pos edge interrupt raw"]
pub type COMP0_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_POS_INT_RAW` reader - analog comparator neg edge interrupt raw"]
pub type COMP0_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP0_POS_INT_RAW` writer - analog comparator neg edge interrupt raw"]
pub type COMP0_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0_ALL_INT_RAW` reader - analog comparator neg or pos edge interrupt raw"]
pub type COMP0_ALL_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP0_ALL_INT_RAW` writer - analog comparator neg or pos edge interrupt raw"]
pub type COMP0_ALL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_NEG_INT_RAW` reader - analog comparator pos edge interrupt raw"]
pub type COMP1_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP1_NEG_INT_RAW` writer - analog comparator pos edge interrupt raw"]
pub type COMP1_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_POS_INT_RAW` reader - analog comparator neg edge interrupt raw"]
pub type COMP1_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP1_POS_INT_RAW` writer - analog comparator neg edge interrupt raw"]
pub type COMP1_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_ALL_INT_RAW` reader - analog comparator neg or pos edge interrupt raw"]
pub type COMP1_ALL_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMP1_ALL_INT_RAW` writer - analog comparator neg or pos edge interrupt raw"]
pub type COMP1_ALL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTOK_INT_RAW` reader - pad bistok interrupt raw"]
pub type BISTOK_INT_RAW_R = crate::BitReader;
#[doc = "Field `BISTOK_INT_RAW` writer - pad bistok interrupt raw"]
pub type BISTOK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTFAIL_INT_RAW` reader - pad bistfail interrupt raw"]
pub type BISTFAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `BISTFAIL_INT_RAW` writer - pad bistfail interrupt raw"]
pub type BISTFAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_neg_int_raw(&self) -> COMP0_NEG_INT_RAW_R {
        COMP0_NEG_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_pos_int_raw(&self) -> COMP0_POS_INT_RAW_R {
        COMP0_POS_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp0_all_int_raw(&self) -> COMP0_ALL_INT_RAW_R {
        COMP0_ALL_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_neg_int_raw(&self) -> COMP1_NEG_INT_RAW_R {
        COMP1_NEG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_pos_int_raw(&self) -> COMP1_POS_INT_RAW_R {
        COMP1_POS_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn comp1_all_int_raw(&self) -> COMP1_ALL_INT_RAW_R {
        COMP1_ALL_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt raw"]
    #[inline(always)]
    pub fn bistok_int_raw(&self) -> BISTOK_INT_RAW_R {
        BISTOK_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt raw"]
    #[inline(always)]
    pub fn bistfail_int_raw(&self) -> BISTFAIL_INT_RAW_R {
        BISTFAIL_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "comp0_neg_int_raw",
                &format_args!("{}", self.comp0_neg_int_raw().bit()),
            )
            .field(
                "comp0_pos_int_raw",
                &format_args!("{}", self.comp0_pos_int_raw().bit()),
            )
            .field(
                "comp0_all_int_raw",
                &format_args!("{}", self.comp0_all_int_raw().bit()),
            )
            .field(
                "comp1_neg_int_raw",
                &format_args!("{}", self.comp1_neg_int_raw().bit()),
            )
            .field(
                "comp1_pos_int_raw",
                &format_args!("{}", self.comp1_pos_int_raw().bit()),
            )
            .field(
                "comp1_all_int_raw",
                &format_args!("{}", self.comp1_all_int_raw().bit()),
            )
            .field(
                "bistok_int_raw",
                &format_args!("{}", self.bistok_int_raw().bit()),
            )
            .field(
                "bistfail_int_raw",
                &format_args!("{}", self.bistfail_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_neg_int_raw(&mut self) -> COMP0_NEG_INT_RAW_W<INT_RAW_SPEC> {
        COMP0_NEG_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_pos_int_raw(&mut self) -> COMP0_POS_INT_RAW_W<INT_RAW_SPEC> {
        COMP0_POS_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp0_all_int_raw(&mut self) -> COMP0_ALL_INT_RAW_W<INT_RAW_SPEC> {
        COMP0_ALL_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_neg_int_raw(&mut self) -> COMP1_NEG_INT_RAW_W<INT_RAW_SPEC> {
        COMP1_NEG_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_pos_int_raw(&mut self) -> COMP1_POS_INT_RAW_W<INT_RAW_SPEC> {
        COMP1_POS_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_all_int_raw(&mut self) -> COMP1_ALL_INT_RAW_W<INT_RAW_SPEC> {
        COMP1_ALL_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn bistok_int_raw(&mut self) -> BISTOK_INT_RAW_W<INT_RAW_SPEC> {
        BISTOK_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn bistfail_int_raw(&mut self) -> BISTFAIL_INT_RAW_W<INT_RAW_SPEC> {
        BISTFAIL_INT_RAW_W::new(self, 7)
    }
}
#[doc = "analog comparator interrupt raw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
