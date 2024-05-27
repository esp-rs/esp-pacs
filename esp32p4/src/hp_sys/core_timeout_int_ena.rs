///Register `CORE_TIMEOUT_INT_ENA` reader
pub type R = crate::R<CORE_TIMEOUT_INT_ENA_SPEC>;
///Register `CORE_TIMEOUT_INT_ENA` writer
pub type W = crate::W<CORE_TIMEOUT_INT_ENA_SPEC>;
///Field `CORE0_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ahb_timeout int
pub type CORE0_AHB_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE0_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ahb_timeout int
pub type CORE0_AHB_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE1_AHB_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ahb_timeout int
pub type CORE1_AHB_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE1_AHB_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ahb_timeout int
pub type CORE1_AHB_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_ibus_timeout int
pub type CORE0_IBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE0_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_ibus_timeout int
pub type CORE0_IBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE1_IBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_ibus_timeout int
pub type CORE1_IBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE1_IBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_ibus_timeout int
pub type CORE1_IBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core0_dbus_timeout int
pub type CORE0_DBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE0_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core0_dbus_timeout int
pub type CORE0_DBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE1_DBUS_TIMEOUT_INT_ENA` reader - Write 1 to enable hp_core1_dbus_timeout int
pub type CORE1_DBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `CORE1_DBUS_TIMEOUT_INT_ENA` writer - Write 1 to enable hp_core1_dbus_timeout int
pub type CORE1_DBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Write 1 to enable hp_core0_ahb_timeout int
    #[inline(always)]
    pub fn core0_ahb_timeout_int_ena(&self) -> CORE0_AHB_TIMEOUT_INT_ENA_R {
        CORE0_AHB_TIMEOUT_INT_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write 1 to enable hp_core1_ahb_timeout int
    #[inline(always)]
    pub fn core1_ahb_timeout_int_ena(&self) -> CORE1_AHB_TIMEOUT_INT_ENA_R {
        CORE1_AHB_TIMEOUT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write 1 to enable hp_core0_ibus_timeout int
    #[inline(always)]
    pub fn core0_ibus_timeout_int_ena(&self) -> CORE0_IBUS_TIMEOUT_INT_ENA_R {
        CORE0_IBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Write 1 to enable hp_core1_ibus_timeout int
    #[inline(always)]
    pub fn core1_ibus_timeout_int_ena(&self) -> CORE1_IBUS_TIMEOUT_INT_ENA_R {
        CORE1_IBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write 1 to enable hp_core0_dbus_timeout int
    #[inline(always)]
    pub fn core0_dbus_timeout_int_ena(&self) -> CORE0_DBUS_TIMEOUT_INT_ENA_R {
        CORE0_DBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Write 1 to enable hp_core1_dbus_timeout int
    #[inline(always)]
    pub fn core1_dbus_timeout_int_ena(&self) -> CORE1_DBUS_TIMEOUT_INT_ENA_R {
        CORE1_DBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_TIMEOUT_INT_ENA")
            .field(
                "core0_ahb_timeout_int_ena",
                &self.core0_ahb_timeout_int_ena(),
            )
            .field(
                "core1_ahb_timeout_int_ena",
                &self.core1_ahb_timeout_int_ena(),
            )
            .field(
                "core0_ibus_timeout_int_ena",
                &self.core0_ibus_timeout_int_ena(),
            )
            .field(
                "core1_ibus_timeout_int_ena",
                &self.core1_ibus_timeout_int_ena(),
            )
            .field(
                "core0_dbus_timeout_int_ena",
                &self.core0_dbus_timeout_int_ena(),
            )
            .field(
                "core1_dbus_timeout_int_ena",
                &self.core1_dbus_timeout_int_ena(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Write 1 to enable hp_core0_ahb_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core0_ahb_timeout_int_ena(
        &mut self,
    ) -> CORE0_AHB_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE0_AHB_TIMEOUT_INT_ENA_W::new(self, 0)
    }
    ///Bit 1 - Write 1 to enable hp_core1_ahb_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core1_ahb_timeout_int_ena(
        &mut self,
    ) -> CORE1_AHB_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE1_AHB_TIMEOUT_INT_ENA_W::new(self, 1)
    }
    ///Bit 2 - Write 1 to enable hp_core0_ibus_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_timeout_int_ena(
        &mut self,
    ) -> CORE0_IBUS_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE0_IBUS_TIMEOUT_INT_ENA_W::new(self, 2)
    }
    ///Bit 3 - Write 1 to enable hp_core1_ibus_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_timeout_int_ena(
        &mut self,
    ) -> CORE1_IBUS_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE1_IBUS_TIMEOUT_INT_ENA_W::new(self, 3)
    }
    ///Bit 4 - Write 1 to enable hp_core0_dbus_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_timeout_int_ena(
        &mut self,
    ) -> CORE0_DBUS_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE0_DBUS_TIMEOUT_INT_ENA_W::new(self, 4)
    }
    ///Bit 5 - Write 1 to enable hp_core1_dbus_timeout int
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_timeout_int_ena(
        &mut self,
    ) -> CORE1_DBUS_TIMEOUT_INT_ENA_W<CORE_TIMEOUT_INT_ENA_SPEC> {
        CORE1_DBUS_TIMEOUT_INT_ENA_W::new(self, 5)
    }
}
/**masked interrupt register

You can [`read`](crate::generic::Reg::read) this register and get [`core_timeout_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_timeout_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_TIMEOUT_INT_ENA_SPEC;
impl crate::RegisterSpec for CORE_TIMEOUT_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_timeout_int_ena::R`](R) reader structure
impl crate::Readable for CORE_TIMEOUT_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`core_timeout_int_ena::W`](W) writer structure
impl crate::Writable for CORE_TIMEOUT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_TIMEOUT_INT_ENA to value 0
impl crate::Resettable for CORE_TIMEOUT_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
