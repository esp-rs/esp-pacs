///Register `CORE0_ACS_CACHE_INT_ENA` reader
pub type R = crate::R<CORE0_ACS_CACHE_INT_ENA_SPEC>;
///Register `CORE0_ACS_CACHE_INT_ENA` writer
pub type W = crate::W<CORE0_ACS_CACHE_INT_ENA_SPEC>;
///Field `CORE0_IBUS_ACS_MSK_IC` reader - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access.
pub type CORE0_IBUS_ACS_MSK_IC_R = crate::BitReader;
///Field `CORE0_IBUS_ACS_MSK_IC` writer - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access.
pub type CORE0_IBUS_ACS_MSK_IC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_IBUS_WR_IC` reader - The bit is used to enable interrupt by ibus trying to write icache
pub type CORE0_IBUS_WR_IC_R = crate::BitReader;
///Field `CORE0_IBUS_WR_IC` writer - The bit is used to enable interrupt by ibus trying to write icache
pub type CORE0_IBUS_WR_IC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_IBUS_REJECT` reader - The bit is used to enable interrupt by authentication fail.
pub type CORE0_IBUS_REJECT_R = crate::BitReader;
///Field `CORE0_IBUS_REJECT` writer - The bit is used to enable interrupt by authentication fail.
pub type CORE0_IBUS_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_DBUS_ACS_MSK_DC` reader - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access.
pub type CORE0_DBUS_ACS_MSK_DC_R = crate::BitReader;
///Field `CORE0_DBUS_ACS_MSK_DC` writer - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access.
pub type CORE0_DBUS_ACS_MSK_DC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_DBUS_REJECT` reader - The bit is used to enable interrupt by authentication fail.
pub type CORE0_DBUS_REJECT_R = crate::BitReader;
///Field `CORE0_DBUS_REJECT` writer - The bit is used to enable interrupt by authentication fail.
pub type CORE0_DBUS_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access.
    #[inline(always)]
    pub fn core0_ibus_acs_msk_ic(&self) -> CORE0_IBUS_ACS_MSK_IC_R {
        CORE0_IBUS_ACS_MSK_IC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable interrupt by ibus trying to write icache
    #[inline(always)]
    pub fn core0_ibus_wr_ic(&self) -> CORE0_IBUS_WR_IC_R {
        CORE0_IBUS_WR_IC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The bit is used to enable interrupt by authentication fail.
    #[inline(always)]
    pub fn core0_ibus_reject(&self) -> CORE0_IBUS_REJECT_R {
        CORE0_IBUS_REJECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access.
    #[inline(always)]
    pub fn core0_dbus_acs_msk_dc(&self) -> CORE0_DBUS_ACS_MSK_DC_R {
        CORE0_DBUS_ACS_MSK_DC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The bit is used to enable interrupt by authentication fail.
    #[inline(always)]
    pub fn core0_dbus_reject(&self) -> CORE0_DBUS_REJECT_R {
        CORE0_DBUS_REJECT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_ACS_CACHE_INT_ENA")
            .field("core0_ibus_acs_msk_ic", &self.core0_ibus_acs_msk_ic())
            .field("core0_ibus_wr_ic", &self.core0_ibus_wr_ic())
            .field("core0_ibus_reject", &self.core0_ibus_reject())
            .field("core0_dbus_acs_msk_dc", &self.core0_dbus_acs_msk_dc())
            .field("core0_dbus_reject", &self.core0_dbus_reject())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_acs_msk_ic(
        &mut self,
    ) -> CORE0_IBUS_ACS_MSK_IC_W<CORE0_ACS_CACHE_INT_ENA_SPEC> {
        CORE0_IBUS_ACS_MSK_IC_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to enable interrupt by ibus trying to write icache
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_wr_ic(&mut self) -> CORE0_IBUS_WR_IC_W<CORE0_ACS_CACHE_INT_ENA_SPEC> {
        CORE0_IBUS_WR_IC_W::new(self, 1)
    }
    ///Bit 2 - The bit is used to enable interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_reject(&mut self) -> CORE0_IBUS_REJECT_W<CORE0_ACS_CACHE_INT_ENA_SPEC> {
        CORE0_IBUS_REJECT_W::new(self, 2)
    }
    ///Bit 3 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_acs_msk_dc(
        &mut self,
    ) -> CORE0_DBUS_ACS_MSK_DC_W<CORE0_ACS_CACHE_INT_ENA_SPEC> {
        CORE0_DBUS_ACS_MSK_DC_W::new(self, 3)
    }
    ///Bit 4 - The bit is used to enable interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_reject(&mut self) -> CORE0_DBUS_REJECT_W<CORE0_ACS_CACHE_INT_ENA_SPEC> {
        CORE0_DBUS_REJECT_W::new(self, 4)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_acs_cache_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_ACS_CACHE_INT_ENA_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core0_acs_cache_int_ena::R`](R) reader structure
impl crate::Readable for CORE0_ACS_CACHE_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`core0_acs_cache_int_ena::W`](W) writer structure
impl crate::Writable for CORE0_ACS_CACHE_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE0_ACS_CACHE_INT_ENA to value 0
impl crate::Resettable for CORE0_ACS_CACHE_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
