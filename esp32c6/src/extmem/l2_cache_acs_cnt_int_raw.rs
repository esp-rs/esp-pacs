#[doc = "Register `L2_CACHE_ACS_CNT_INT_RAW` reader"]
pub type R = crate::R<L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Register `L2_CACHE_ACS_CNT_INT_RAW` writer"]
pub type W = crate::W<L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Field `L2_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type L2_IBUS0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_IBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type L2_IBUS0_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type L2_IBUS1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_IBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type L2_IBUS1_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type L2_IBUS2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_IBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type L2_IBUS2_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type L2_IBUS3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_IBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type L2_IBUS3_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_DBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type L2_DBUS0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_DBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type L2_DBUS0_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_DBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type L2_DBUS1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_DBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type L2_DBUS1_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type L2_DBUS2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_DBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type L2_DBUS2_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L2_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type L2_DBUS3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `L2_DBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type L2_DBUS3_OVF_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_raw(&self) -> L2_IBUS0_OVF_INT_RAW_R {
        L2_IBUS0_OVF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_raw(&self) -> L2_IBUS1_OVF_INT_RAW_R {
        L2_IBUS1_OVF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_raw(&self) -> L2_IBUS2_OVF_INT_RAW_R {
        L2_IBUS2_OVF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_raw(&self) -> L2_IBUS3_OVF_INT_RAW_R {
        L2_IBUS3_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_raw(&self) -> L2_DBUS0_OVF_INT_RAW_R {
        L2_DBUS0_OVF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_raw(&self) -> L2_DBUS1_OVF_INT_RAW_R {
        L2_DBUS1_OVF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_raw(&self) -> L2_DBUS2_OVF_INT_RAW_R {
        L2_DBUS2_OVF_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_raw(&self) -> L2_DBUS3_OVF_INT_RAW_R {
        L2_DBUS3_OVF_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_CNT_INT_RAW")
            .field(
                "l2_ibus0_ovf_int_raw",
                &format_args!("{}", self.l2_ibus0_ovf_int_raw().bit()),
            )
            .field(
                "l2_ibus1_ovf_int_raw",
                &format_args!("{}", self.l2_ibus1_ovf_int_raw().bit()),
            )
            .field(
                "l2_ibus2_ovf_int_raw",
                &format_args!("{}", self.l2_ibus2_ovf_int_raw().bit()),
            )
            .field(
                "l2_ibus3_ovf_int_raw",
                &format_args!("{}", self.l2_ibus3_ovf_int_raw().bit()),
            )
            .field(
                "l2_dbus0_ovf_int_raw",
                &format_args!("{}", self.l2_dbus0_ovf_int_raw().bit()),
            )
            .field(
                "l2_dbus1_ovf_int_raw",
                &format_args!("{}", self.l2_dbus1_ovf_int_raw().bit()),
            )
            .field(
                "l2_dbus2_ovf_int_raw",
                &format_args!("{}", self.l2_dbus2_ovf_int_raw().bit()),
            )
            .field(
                "l2_dbus3_ovf_int_raw",
                &format_args!("{}", self.l2_dbus3_ovf_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus0_ovf_int_raw(
        &mut self,
    ) -> L2_IBUS0_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 8> {
        L2_IBUS0_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus1_ovf_int_raw(
        &mut self,
    ) -> L2_IBUS1_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 9> {
        L2_IBUS1_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus2_ovf_int_raw(
        &mut self,
    ) -> L2_IBUS2_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 10> {
        L2_IBUS2_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus3_ovf_int_raw(
        &mut self,
    ) -> L2_IBUS3_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 11> {
        L2_IBUS3_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus0_ovf_int_raw(
        &mut self,
    ) -> L2_DBUS0_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 12> {
        L2_DBUS0_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus1_ovf_int_raw(
        &mut self,
    ) -> L2_DBUS1_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 13> {
        L2_DBUS1_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus2_ovf_int_raw(
        &mut self,
    ) -> L2_DBUS2_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 14> {
        L2_DBUS2_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus3_ovf_int_raw(
        &mut self,
    ) -> L2_DBUS3_OVF_INT_RAW_W<L2_CACHE_ACS_CNT_INT_RAW_SPEC, 15> {
        L2_DBUS3_OVF_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_cnt_int_raw::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_CNT_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_cnt_int_raw::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
