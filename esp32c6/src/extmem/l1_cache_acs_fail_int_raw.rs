#[doc = "Register `L1_CACHE_ACS_FAIL_INT_RAW` reader"]
pub struct R(crate::R<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_ACS_FAIL_INT_RAW` writer"]
pub struct W(crate::W<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>;
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
impl From<crate::W<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_ICACHE0_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type L1_ICACHE0_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type L1_ICACHE0_FAIL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_FAIL_INT_RAW_SPEC, O>;
#[doc = "Field `L1_ICACHE1_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type L1_ICACHE1_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type L1_ICACHE1_FAIL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_FAIL_INT_RAW_SPEC, O>;
#[doc = "Field `L1_ICACHE2_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type L1_ICACHE2_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type L1_ICACHE2_FAIL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_FAIL_INT_RAW_SPEC, O>;
#[doc = "Field `L1_ICACHE3_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type L1_ICACHE3_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type L1_ICACHE3_FAIL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_FAIL_INT_RAW_SPEC, O>;
#[doc = "Field `L1_CACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type L1_CACHE_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `L1_CACHE_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type L1_CACHE_FAIL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_ACS_FAIL_INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_raw(&self) -> L1_ICACHE0_FAIL_INT_RAW_R {
        L1_ICACHE0_FAIL_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_raw(&self) -> L1_ICACHE1_FAIL_INT_RAW_R {
        L1_ICACHE1_FAIL_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_fail_int_raw(&self) -> L1_ICACHE2_FAIL_INT_RAW_R {
        L1_ICACHE2_FAIL_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    pub fn l1_icache3_fail_int_raw(&self) -> L1_ICACHE3_FAIL_INT_RAW_R {
        L1_ICACHE3_FAIL_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    pub fn l1_cache_fail_int_raw(&self) -> L1_CACHE_FAIL_INT_RAW_R {
        L1_CACHE_FAIL_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_FAIL_INT_RAW")
            .field(
                "l1_icache0_fail_int_raw",
                &format_args!("{}", self.l1_icache0_fail_int_raw().bit()),
            )
            .field(
                "l1_icache1_fail_int_raw",
                &format_args!("{}", self.l1_icache1_fail_int_raw().bit()),
            )
            .field(
                "l1_icache2_fail_int_raw",
                &format_args!("{}", self.l1_icache2_fail_int_raw().bit()),
            )
            .field(
                "l1_icache3_fail_int_raw",
                &format_args!("{}", self.l1_icache3_fail_int_raw().bit()),
            )
            .field(
                "l1_cache_fail_int_raw",
                &format_args!("{}", self.l1_cache_fail_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_FAIL_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_fail_int_raw(&mut self) -> L1_ICACHE0_FAIL_INT_RAW_W<0> {
        L1_ICACHE0_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_fail_int_raw(&mut self) -> L1_ICACHE1_FAIL_INT_RAW_W<1> {
        L1_ICACHE1_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache2_fail_int_raw(&mut self) -> L1_ICACHE2_FAIL_INT_RAW_W<2> {
        L1_ICACHE2_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache3_fail_int_raw(&mut self) -> L1_ICACHE3_FAIL_INT_RAW_W<3> {
        L1_ICACHE3_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_fail_int_raw(&mut self) -> L1_CACHE_FAIL_INT_RAW_W<4> {
        L1_CACHE_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Access Fail Interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_acs_fail_int_raw](index.html) module"]
pub struct L1_CACHE_ACS_FAIL_INT_RAW_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_acs_fail_int_raw::R](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_acs_fail_int_raw::W](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_RAW to value 0"]
impl crate::Resettable for L1_CACHE_ACS_FAIL_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
