#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE` writer - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE` writer - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UPVOLTAGE` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_R = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE` writer - need_des"]
pub type VDDBAT_UPVOLTAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_R = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE` writer - need_des"]
pub type VDDBAT_UNDERVOLTAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0` reader - need_des"]
pub type BOD_MODE0_R = crate::BitReader;
#[doc = "Field `BOD_MODE0` writer - need_des"]
pub type BOD_MODE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage(&self) -> VDDBAT_CHARGE_UPVOLTAGE_R {
        VDDBAT_CHARGE_UPVOLTAGE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage(&self) -> VDDBAT_UPVOLTAGE_R {
        VDDBAT_UPVOLTAGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage(&self) -> VDDBAT_UNDERVOLTAGE_R {
        VDDBAT_UNDERVOLTAGE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&self) -> BOD_MODE0_R {
        BOD_MODE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "vddbat_charge_upvoltage",
                &format_args!("{}", self.vddbat_charge_upvoltage().bit()),
            )
            .field(
                "vddbat_charge_undervoltage",
                &format_args!("{}", self.vddbat_charge_undervoltage().bit()),
            )
            .field(
                "vddbat_upvoltage",
                &format_args!("{}", self.vddbat_upvoltage().bit()),
            )
            .field(
                "vddbat_undervoltage",
                &format_args!("{}", self.vddbat_undervoltage().bit()),
            )
            .field("bod_mode0", &format_args!("{}", self.bod_mode0().bit()))
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
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage(&mut self) -> VDDBAT_CHARGE_UPVOLTAGE_W<INT_ENA_SPEC> {
        VDDBAT_CHARGE_UPVOLTAGE_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage(&mut self) -> VDDBAT_CHARGE_UNDERVOLTAGE_W<INT_ENA_SPEC> {
        VDDBAT_CHARGE_UNDERVOLTAGE_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_upvoltage(&mut self) -> VDDBAT_UPVOLTAGE_W<INT_ENA_SPEC> {
        VDDBAT_UPVOLTAGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_undervoltage(&mut self) -> VDDBAT_UNDERVOLTAGE_W<INT_ENA_SPEC> {
        VDDBAT_UNDERVOLTAGE_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0(&mut self) -> BOD_MODE0_W<INT_ENA_SPEC> {
        BOD_MODE0_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
