#[doc = "Register `RD_MAC_SYS4` reader"]
pub type R = crate::R<RD_MAC_SYS4_SPEC>;
#[doc = "Field `SYS_DATA_PART0_1` reader - Represents the first 14-bit of zeroth part of system data."]
pub type SYS_DATA_PART0_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the first 14-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_1(&self) -> SYS_DATA_PART0_1_R {
        SYS_DATA_PART0_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS4")
            .field("sys_data_part0_1", &self.sys_data_part0_1())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS4_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys4::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS4_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS4 to value 0"]
impl crate::Resettable for RD_MAC_SYS4_SPEC {
    const RESET_VALUE: u32 = 0;
}
