#[doc = "Register `HP_CORE_TIMEOUT_INT_RAW` reader"]
pub type R = crate::R<HP_CORE_TIMEOUT_INT_RAW_SPEC>;
#[doc = "Register `HP_CORE_TIMEOUT_INT_RAW` writer"]
pub type W = crate::W<HP_CORE_TIMEOUT_INT_RAW_SPEC>;
#[doc = "Field `HP_CORE0_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 ahb timeout"]
pub type HP_CORE0_AHB_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE0_AHB_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 ahb timeout"]
pub type HP_CORE0_AHB_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 ahb timeout"]
pub type HP_CORE1_AHB_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE1_AHB_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 ahb timeout"]
pub type HP_CORE1_AHB_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 ibus timeout"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 ibus timeout"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 ibus timeout"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 ibus timeout"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core0 dbus timeout"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core0 dbus timeout"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of hp core1 dbus timeout"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_RAW` writer - the raw interrupt status of hp core1 dbus timeout"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn hp_core0_ahb_timeout_int_raw(&self) -> HP_CORE0_AHB_TIMEOUT_INT_RAW_R {
        HP_CORE0_AHB_TIMEOUT_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn hp_core1_ahb_timeout_int_raw(&self) -> HP_CORE1_AHB_TIMEOUT_INT_RAW_R {
        HP_CORE1_AHB_TIMEOUT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn hp_core0_ibus_timeout_int_raw(&self) -> HP_CORE0_IBUS_TIMEOUT_INT_RAW_R {
        HP_CORE0_IBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn hp_core1_ibus_timeout_int_raw(&self) -> HP_CORE1_IBUS_TIMEOUT_INT_RAW_R {
        HP_CORE1_IBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn hp_core0_dbus_timeout_int_raw(&self) -> HP_CORE0_DBUS_TIMEOUT_INT_RAW_R {
        HP_CORE0_DBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn hp_core1_dbus_timeout_int_raw(&self) -> HP_CORE1_DBUS_TIMEOUT_INT_RAW_R {
        HP_CORE1_DBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_TIMEOUT_INT_RAW")
            .field(
                "hp_core0_ahb_timeout_int_raw",
                &format_args!("{}", self.hp_core0_ahb_timeout_int_raw().bit()),
            )
            .field(
                "hp_core1_ahb_timeout_int_raw",
                &format_args!("{}", self.hp_core1_ahb_timeout_int_raw().bit()),
            )
            .field(
                "hp_core0_ibus_timeout_int_raw",
                &format_args!("{}", self.hp_core0_ibus_timeout_int_raw().bit()),
            )
            .field(
                "hp_core1_ibus_timeout_int_raw",
                &format_args!("{}", self.hp_core1_ibus_timeout_int_raw().bit()),
            )
            .field(
                "hp_core0_dbus_timeout_int_raw",
                &format_args!("{}", self.hp_core0_dbus_timeout_int_raw().bit()),
            )
            .field(
                "hp_core1_dbus_timeout_int_raw",
                &format_args!("{}", self.hp_core1_dbus_timeout_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - the raw interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_ahb_timeout_int_raw(
        &mut self,
    ) -> HP_CORE0_AHB_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE0_AHB_TIMEOUT_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_ahb_timeout_int_raw(
        &mut self,
    ) -> HP_CORE1_AHB_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE1_AHB_TIMEOUT_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - the raw interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_ibus_timeout_int_raw(
        &mut self,
    ) -> HP_CORE0_IBUS_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE0_IBUS_TIMEOUT_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - the raw interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_ibus_timeout_int_raw(
        &mut self,
    ) -> HP_CORE1_IBUS_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE1_IBUS_TIMEOUT_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - the raw interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_dbus_timeout_int_raw(
        &mut self,
    ) -> HP_CORE0_DBUS_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE0_DBUS_TIMEOUT_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - the raw interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_dbus_timeout_int_raw(
        &mut self,
    ) -> HP_CORE1_DBUS_TIMEOUT_INT_RAW_W<HP_CORE_TIMEOUT_INT_RAW_SPEC> {
        HP_CORE1_DBUS_TIMEOUT_INT_RAW_W::new(self, 5)
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
#[doc = "Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_timeout_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_TIMEOUT_INT_RAW_SPEC;
impl crate::RegisterSpec for HP_CORE_TIMEOUT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_timeout_int_raw::R`](R) reader structure"]
impl crate::Readable for HP_CORE_TIMEOUT_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_core_timeout_int_raw::W`](W) writer structure"]
impl crate::Writable for HP_CORE_TIMEOUT_INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CORE_TIMEOUT_INT_RAW to value 0"]
impl crate::Resettable for HP_CORE_TIMEOUT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
