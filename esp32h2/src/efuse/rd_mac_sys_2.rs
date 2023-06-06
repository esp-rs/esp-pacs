#[doc = "Register `RD_MAC_SYS_2` reader"]
pub struct R(crate::R<RD_MAC_SYS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SYS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SYS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SYS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_RESERVED_1` reader - Reserved."]
pub type MAC_RESERVED_1_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_RESERVED_0` reader - Reserved."]
pub type MAC_RESERVED_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:13 - Reserved."]
    #[inline(always)]
    pub fn mac_reserved_1(&self) -> MAC_RESERVED_1_R {
        MAC_RESERVED_1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Reserved."]
    #[inline(always)]
    pub fn mac_reserved_0(&self) -> MAC_RESERVED_0_R {
        MAC_RESERVED_0_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS_2")
            .field(
                "mac_reserved_1",
                &format_args!("{}", self.mac_reserved_1().bits()),
            )
            .field(
                "mac_reserved_0",
                &format_args!("{}", self.mac_reserved_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SYS_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK1 data register $n.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_sys_2](index.html) module"]
pub struct RD_MAC_SYS_2_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_sys_2::R](R) reader structure"]
impl crate::Readable for RD_MAC_SYS_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SYS_2 to value 0"]
impl crate::Resettable for RD_MAC_SYS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
