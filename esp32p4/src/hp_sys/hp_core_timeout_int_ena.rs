#[doc = "Register `HP_CORE_TIMEOUT_INT_ENA` reader"]
pub type R = crate::R<HP_CORE_TIMEOUT_INT_ENA_SPEC>;
#[doc = "Register `HP_CORE_TIMEOUT_INT_ENA` writer"]
pub type W = crate::W<HP_CORE_TIMEOUT_INT_ENA_SPEC>;
#[doc = "Field `HP_CORE0_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ahb_timeout int"]
pub type HP_CORE0_AHB_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE0_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ahb_timeout int"]
pub type HP_CORE0_AHB_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ahb_timeout int"]
pub type HP_CORE1_AHB_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE1_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ahb_timeout int"]
pub type HP_CORE1_AHB_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ibus_timeout int"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ibus_timeout int"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ibus_timeout int"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ibus_timeout int"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_dbus_timeout int"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_dbus_timeout int"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_dbus_timeout int"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_dbus_timeout int"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable hp_core0_ahb_timeout int"]
    #[inline(always)]
    pub fn hp_core0_ahb_timeout_int_ena(&self) -> HP_CORE0_AHB_TIMEOUT_INT_ENA_R {
        HP_CORE0_AHB_TIMEOUT_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable hp_core1_ahb_timeout int"]
    #[inline(always)]
    pub fn hp_core1_ahb_timeout_int_ena(&self) -> HP_CORE1_AHB_TIMEOUT_INT_ENA_R {
        HP_CORE1_AHB_TIMEOUT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable hp_core0_ibus_timeout int"]
    #[inline(always)]
    pub fn hp_core0_ibus_timeout_int_ena(&self) -> HP_CORE0_IBUS_TIMEOUT_INT_ENA_R {
        HP_CORE0_IBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable hp_core1_ibus_timeout int"]
    #[inline(always)]
    pub fn hp_core1_ibus_timeout_int_ena(&self) -> HP_CORE1_IBUS_TIMEOUT_INT_ENA_R {
        HP_CORE1_IBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable hp_core0_dbus_timeout int"]
    #[inline(always)]
    pub fn hp_core0_dbus_timeout_int_ena(&self) -> HP_CORE0_DBUS_TIMEOUT_INT_ENA_R {
        HP_CORE0_DBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable hp_core1_dbus_timeout int"]
    #[inline(always)]
    pub fn hp_core1_dbus_timeout_int_ena(&self) -> HP_CORE1_DBUS_TIMEOUT_INT_ENA_R {
        HP_CORE1_DBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_TIMEOUT_INT_ENA")
            .field(
                "hp_core0_ahb_timeout_int_ena",
                &format_args!("{}", self.hp_core0_ahb_timeout_int_ena().bit()),
            )
            .field(
                "hp_core1_ahb_timeout_int_ena",
                &format_args!("{}", self.hp_core1_ahb_timeout_int_ena().bit()),
            )
            .field(
                "hp_core0_ibus_timeout_int_ena",
                &format_args!("{}", self.hp_core0_ibus_timeout_int_ena().bit()),
            )
            .field(
                "hp_core1_ibus_timeout_int_ena",
                &format_args!("{}", self.hp_core1_ibus_timeout_int_ena().bit()),
            )
            .field(
                "hp_core0_dbus_timeout_int_ena",
                &format_args!("{}", self.hp_core0_dbus_timeout_int_ena().bit()),
            )
            .field(
                "hp_core1_dbus_timeout_int_ena",
                &format_args!("{}", self.hp_core1_dbus_timeout_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable hp_core0_ahb_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_ahb_timeout_int_ena(
        &mut self,
    ) -> HP_CORE0_AHB_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE0_AHB_TIMEOUT_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable hp_core1_ahb_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_ahb_timeout_int_ena(
        &mut self,
    ) -> HP_CORE1_AHB_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE1_AHB_TIMEOUT_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable hp_core0_ibus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_ibus_timeout_int_ena(
        &mut self,
    ) -> HP_CORE0_IBUS_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE0_IBUS_TIMEOUT_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable hp_core1_ibus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_ibus_timeout_int_ena(
        &mut self,
    ) -> HP_CORE1_IBUS_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE1_IBUS_TIMEOUT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable hp_core0_dbus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core0_dbus_timeout_int_ena(
        &mut self,
    ) -> HP_CORE0_DBUS_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE0_DBUS_TIMEOUT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable hp_core1_dbus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_core1_dbus_timeout_int_ena(
        &mut self,
    ) -> HP_CORE1_DBUS_TIMEOUT_INT_ENA_W<HP_CORE_TIMEOUT_INT_ENA_SPEC> {
        HP_CORE1_DBUS_TIMEOUT_INT_ENA_W::new(self, 5)
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
#[doc = "masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_timeout_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_TIMEOUT_INT_ENA_SPEC;
impl crate::RegisterSpec for HP_CORE_TIMEOUT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_timeout_int_ena::R`](R) reader structure"]
impl crate::Readable for HP_CORE_TIMEOUT_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_core_timeout_int_ena::W`](W) writer structure"]
impl crate::Writable for HP_CORE_TIMEOUT_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CORE_TIMEOUT_INT_ENA to value 0"]
impl crate::Resettable for HP_CORE_TIMEOUT_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
