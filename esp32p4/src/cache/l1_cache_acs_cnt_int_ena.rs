#[doc = "Register `L1_CACHE_ACS_CNT_INT_ENA` reader"]
pub type R = crate::R<L1_CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Register `L1_CACHE_ACS_CNT_INT_ENA` writer"]
pub type W = crate::W<L1_CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Field `L1_IBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_IBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS2_OVF_INT_ENA` reader - Reserved"]
pub type L1_IBUS2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_ENA` reader - Reserved"]
pub type L1_IBUS3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_DBUS0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_DBUS0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_DBUS1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_DBUS1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS2_OVF_INT_ENA` reader - Reserved"]
pub type L1_DBUS2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DBUS3_OVF_INT_ENA` reader - Reserved"]
pub type L1_DBUS3_OVF_INT_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_ena(&self) -> L1_IBUS0_OVF_INT_ENA_R {
        L1_IBUS0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_ena(&self) -> L1_IBUS1_OVF_INT_ENA_R {
        L1_IBUS1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_ena(&self) -> L1_IBUS2_OVF_INT_ENA_R {
        L1_IBUS2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_ena(&self) -> L1_IBUS3_OVF_INT_ENA_R {
        L1_IBUS3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_ena(&self) -> L1_DBUS0_OVF_INT_ENA_R {
        L1_DBUS0_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_ena(&self) -> L1_DBUS1_OVF_INT_ENA_R {
        L1_DBUS1_OVF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_ena(&self) -> L1_DBUS2_OVF_INT_ENA_R {
        L1_DBUS2_OVF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_ena(&self) -> L1_DBUS3_OVF_INT_ENA_R {
        L1_DBUS3_OVF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_CNT_INT_ENA")
            .field(
                "l1_ibus0_ovf_int_ena",
                &format_args!("{}", self.l1_ibus0_ovf_int_ena().bit()),
            )
            .field(
                "l1_ibus1_ovf_int_ena",
                &format_args!("{}", self.l1_ibus1_ovf_int_ena().bit()),
            )
            .field(
                "l1_ibus2_ovf_int_ena",
                &format_args!("{}", self.l1_ibus2_ovf_int_ena().bit()),
            )
            .field(
                "l1_ibus3_ovf_int_ena",
                &format_args!("{}", self.l1_ibus3_ovf_int_ena().bit()),
            )
            .field(
                "l1_dbus0_ovf_int_ena",
                &format_args!("{}", self.l1_dbus0_ovf_int_ena().bit()),
            )
            .field(
                "l1_dbus1_ovf_int_ena",
                &format_args!("{}", self.l1_dbus1_ovf_int_ena().bit()),
            )
            .field(
                "l1_dbus2_ovf_int_ena",
                &format_args!("{}", self.l1_dbus2_ovf_int_ena().bit()),
            )
            .field(
                "l1_dbus3_ovf_int_ena",
                &format_args!("{}", self.l1_dbus3_ovf_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_CNT_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus0_ovf_int_ena(
        &mut self,
    ) -> L1_IBUS0_OVF_INT_ENA_W<L1_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L1_IBUS0_OVF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus1_ovf_int_ena(
        &mut self,
    ) -> L1_IBUS1_OVF_INT_ENA_W<L1_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L1_IBUS1_OVF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus0_ovf_int_ena(
        &mut self,
    ) -> L1_DBUS0_OVF_INT_ENA_W<L1_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L1_DBUS0_OVF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus1_ovf_int_ena(
        &mut self,
    ) -> L1_DBUS1_OVF_INT_ENA_W<L1_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L1_DBUS1_OVF_INT_ENA_W::new(self, 5)
    }
}
#[doc = "Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_CNT_INT_ENA_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_cnt_int_ena::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_cnt_int_ena::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_CNT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_ENA to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
