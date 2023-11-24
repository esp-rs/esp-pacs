#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_INT_RAW` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_INT_RAW` writer - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW` writer - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UPVOLTAGE_INT_RAW` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_INT_RAW_R = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE_INT_RAW` writer - need_des"]
pub type VDDBAT_UPVOLTAGE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_INT_RAW` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_INT_RAW_R = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_INT_RAW` writer - need_des"]
pub type VDDBAT_UNDERVOLTAGE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_INT_RAW` reader - need_des"]
pub type BOD_MODE0_INT_RAW_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INT_RAW` writer - need_des"]
pub type BOD_MODE0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_int_raw(&self) -> VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_R {
        VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_int_raw(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_int_raw(&self) -> VDDBAT_UPVOLTAGE_INT_RAW_R {
        VDDBAT_UPVOLTAGE_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_int_raw(&self) -> VDDBAT_UNDERVOLTAGE_INT_RAW_R {
        VDDBAT_UNDERVOLTAGE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_int_raw(&self) -> BOD_MODE0_INT_RAW_R {
        BOD_MODE0_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "vddbat_charge_upvoltage_int_raw",
                &format_args!("{}", self.vddbat_charge_upvoltage_int_raw().bit()),
            )
            .field(
                "vddbat_charge_undervoltage_int_raw",
                &format_args!("{}", self.vddbat_charge_undervoltage_int_raw().bit()),
            )
            .field(
                "vddbat_upvoltage_int_raw",
                &format_args!("{}", self.vddbat_upvoltage_int_raw().bit()),
            )
            .field(
                "vddbat_undervoltage_int_raw",
                &format_args!("{}", self.vddbat_undervoltage_int_raw().bit()),
            )
            .field(
                "bod_mode0_int_raw",
                &format_args!("{}", self.bod_mode0_int_raw().bit()),
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
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage_int_raw(
        &mut self,
    ) -> VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_W<INT_RAW_SPEC> {
        VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage_int_raw(
        &mut self,
    ) -> VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_W<INT_RAW_SPEC> {
        VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_upvoltage_int_raw(&mut self) -> VDDBAT_UPVOLTAGE_INT_RAW_W<INT_RAW_SPEC> {
        VDDBAT_UPVOLTAGE_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_undervoltage_int_raw(&mut self) -> VDDBAT_UNDERVOLTAGE_INT_RAW_W<INT_RAW_SPEC> {
        VDDBAT_UNDERVOLTAGE_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_int_raw(&mut self) -> BOD_MODE0_INT_RAW_W<INT_RAW_SPEC> {
        BOD_MODE0_INT_RAW_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
