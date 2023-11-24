#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_INT_ST` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_INT_ST_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_R = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE_INT_ST` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_INT_ST_R = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_INT_ST` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_INT_ST_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INT_ST` reader - need_des"]
pub type BOD_MODE0_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_int_st(&self) -> VDDBAT_CHARGE_UPVOLTAGE_INT_ST_R {
        VDDBAT_CHARGE_UPVOLTAGE_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_int_st(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_int_st(&self) -> VDDBAT_UPVOLTAGE_INT_ST_R {
        VDDBAT_UPVOLTAGE_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_int_st(&self) -> VDDBAT_UNDERVOLTAGE_INT_ST_R {
        VDDBAT_UNDERVOLTAGE_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_int_st(&self) -> BOD_MODE0_INT_ST_R {
        BOD_MODE0_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "vddbat_charge_upvoltage_int_st",
                &format_args!("{}", self.vddbat_charge_upvoltage_int_st().bit()),
            )
            .field(
                "vddbat_charge_undervoltage_int_st",
                &format_args!("{}", self.vddbat_charge_undervoltage_int_st().bit()),
            )
            .field(
                "vddbat_upvoltage_int_st",
                &format_args!("{}", self.vddbat_upvoltage_int_st().bit()),
            )
            .field(
                "vddbat_undervoltage_int_st",
                &format_args!("{}", self.vddbat_undervoltage_int_st().bit()),
            )
            .field(
                "bod_mode0_int_st",
                &format_args!("{}", self.bod_mode0_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
