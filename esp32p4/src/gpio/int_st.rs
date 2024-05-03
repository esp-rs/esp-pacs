#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `COMP0_NEG` reader - analog comparator pos edge interrupt status"]
pub type COMP0_NEG_R = crate::BitReader;
#[doc = "Field `COMP0_POS` reader - analog comparator neg edge interrupt status"]
pub type COMP0_POS_R = crate::BitReader;
#[doc = "Field `COMP0_ALL` reader - analog comparator neg or pos edge interrupt status"]
pub type COMP0_ALL_R = crate::BitReader;
#[doc = "Field `COMP1_NEG` reader - analog comparator pos edge interrupt status"]
pub type COMP1_NEG_R = crate::BitReader;
#[doc = "Field `COMP1_POS` reader - analog comparator neg edge interrupt status"]
pub type COMP1_POS_R = crate::BitReader;
#[doc = "Field `COMP1_ALL` reader - analog comparator neg or pos edge interrupt status"]
pub type COMP1_ALL_R = crate::BitReader;
#[doc = "Field `BISTOK` reader - pad bistok interrupt status"]
pub type BISTOK_R = crate::BitReader;
#[doc = "Field `BISTFAIL` reader - pad bistfail interrupt status"]
pub type BISTFAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn comp0_neg(&self) -> COMP0_NEG_R {
        COMP0_NEG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn comp0_pos(&self) -> COMP0_POS_R {
        COMP0_POS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn comp0_all(&self) -> COMP0_ALL_R {
        COMP0_ALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn comp1_neg(&self) -> COMP1_NEG_R {
        COMP1_NEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn comp1_pos(&self) -> COMP1_POS_R {
        COMP1_POS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn comp1_all(&self) -> COMP1_ALL_R {
        COMP1_ALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt status"]
    #[inline(always)]
    pub fn bistok(&self) -> BISTOK_R {
        BISTOK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt status"]
    #[inline(always)]
    pub fn bistfail(&self) -> BISTFAIL_R {
        BISTFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("comp0_neg", &self.comp0_neg().bit())
            .field("comp0_pos", &self.comp0_pos().bit())
            .field("comp0_all", &self.comp0_all().bit())
            .field("comp1_neg", &self.comp1_neg().bit())
            .field("comp1_pos", &self.comp1_pos().bit())
            .field("comp1_all", &self.comp1_all().bit())
            .field("bistok", &self.bistok().bit())
            .field("bistfail", &self.bistfail().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "analog comparator interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
