#[doc = "Register `L1_ICACHE0_ACS_FAIL_ID_ATTR` reader"]
pub struct R(crate::R<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE0_FAIL_ID` reader - The register records the ID of fail-access when cache0 accesses L1-ICache."]
pub type L1_ICACHE0_FAIL_ID_R = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE0_FAIL_ATTR` reader - The register records the attribution of fail-access when cache0 accesses L1-ICache."]
pub type L1_ICACHE0_FAIL_ATTR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when cache0 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache0_fail_id(&self) -> L1_ICACHE0_FAIL_ID_R {
        L1_ICACHE0_FAIL_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when cache0 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache0_fail_attr(&self) -> L1_ICACHE0_FAIL_ATTR_R {
        L1_ICACHE0_FAIL_ATTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_ACS_FAIL_ID_ATTR")
            .field(
                "l1_icache0_fail_id",
                &format_args!("{}", self.l1_icache0_fail_id().bits()),
            )
            .field(
                "l1_icache0_fail_attr",
                &format_args!("{}", self.l1_icache0_fail_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1-ICache0 Access Fail ID/attribution information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache0_acs_fail_id_attr](index.html) module"]
pub struct L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache0_acs_fail_id_attr::R](R) reader structure"]
impl crate::Readable for L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE0_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
