#[doc = "Register `L1_CACHE_ACS_CNT_INT_CLR` reader"]
pub struct R(crate::R<L1_CACHE_ACS_CNT_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_ACS_CNT_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_ACS_CNT_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_ACS_CNT_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_ACS_CNT_INT_CLR` writer"]
pub struct W(crate::W<L1_CACHE_ACS_CNT_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_ACS_CNT_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<L1_CACHE_ACS_CNT_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_ACS_CNT_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_IBUS0_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_IBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L1_IBUS2_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L1_IBUS3_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_BUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_BUS0_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_CNT_INT_CLR_SPEC, O>;
#[doc = "Field `L1_BUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_BUS1_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_CNT_INT_CLR_SPEC, O>;
#[doc = "Field `L1_DBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L1_DBUS2_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_DBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L1_DBUS3_OVF_INT_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_clr(&self) -> L1_IBUS0_OVF_INT_CLR_R {
        L1_IBUS0_OVF_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_clr(&self) -> L1_IBUS1_OVF_INT_CLR_R {
        L1_IBUS1_OVF_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_clr(&self) -> L1_IBUS2_OVF_INT_CLR_R {
        L1_IBUS2_OVF_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_clr(&self) -> L1_IBUS3_OVF_INT_CLR_R {
        L1_IBUS3_OVF_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_clr(&self) -> L1_DBUS2_OVF_INT_CLR_R {
        L1_DBUS2_OVF_INT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_clr(&self) -> L1_DBUS3_OVF_INT_CLR_R {
        L1_DBUS3_OVF_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_CNT_INT_CLR")
            .field(
                "l1_ibus0_ovf_int_clr",
                &format_args!("{}", self.l1_ibus0_ovf_int_clr().bit()),
            )
            .field(
                "l1_ibus1_ovf_int_clr",
                &format_args!("{}", self.l1_ibus1_ovf_int_clr().bit()),
            )
            .field(
                "l1_ibus2_ovf_int_clr",
                &format_args!("{}", self.l1_ibus2_ovf_int_clr().bit()),
            )
            .field(
                "l1_ibus3_ovf_int_clr",
                &format_args!("{}", self.l1_ibus3_ovf_int_clr().bit()),
            )
            .field(
                "l1_dbus2_ovf_int_clr",
                &format_args!("{}", self.l1_dbus2_ovf_int_clr().bit()),
            )
            .field(
                "l1_dbus3_ovf_int_clr",
                &format_args!("{}", self.l1_dbus3_ovf_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus0_ovf_int_clr(&mut self) -> L1_BUS0_OVF_INT_CLR_W<4> {
        L1_BUS0_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus1_ovf_int_clr(&mut self) -> L1_BUS1_OVF_INT_CLR_W<5> {
        L1_BUS1_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_acs_cnt_int_clr](index.html) module"]
pub struct L1_CACHE_ACS_CNT_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_acs_cnt_int_clr::R](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_acs_cnt_int_clr::W](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
