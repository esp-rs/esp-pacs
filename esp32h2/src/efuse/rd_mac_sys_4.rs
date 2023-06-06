#[doc = "Register `RD_MAC_SYS_4` reader"]
pub struct R(crate::R<RD_MAC_SYS_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SYS_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SYS_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SYS_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART0_1` reader - Stores the first 32 bits of the zeroth part of system data."]
pub type SYS_DATA_PART0_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the first 32 bits of the zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_1(&self) -> SYS_DATA_PART0_1_R {
        SYS_DATA_PART0_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS_4")
            .field(
                "sys_data_part0_1",
                &format_args!("{}", self.sys_data_part0_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SYS_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK1 data register $n.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_sys_4](index.html) module"]
pub struct RD_MAC_SYS_4_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_sys_4::R](R) reader structure"]
impl crate::Readable for RD_MAC_SYS_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SYS_4 to value 0"]
impl crate::Resettable for RD_MAC_SYS_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
