#[doc = "Register `RD_MAC_SYS5` reader"]
pub type R = crate::R<RD_MAC_SYS5_SPEC>;
#[doc = "Field `PUMP_DRV` reader - Use to configure charge pump voltage gain.\\\\"]
pub type PUMP_DRV_R = crate::FieldReader;
#[doc = "Field `SYS_DATA_PART0_2` reader - Represents the second 28-bit of zeroth part of system data."]
pub type SYS_DATA_PART0_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Use to configure charge pump voltage gain.\\\\"]
    #[inline(always)]
    pub fn pump_drv(&self) -> PUMP_DRV_R {
        PUMP_DRV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Represents the second 28-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_2(&self) -> SYS_DATA_PART0_2_R {
        SYS_DATA_PART0_2_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS5")
            .field("pump_drv", &self.pump_drv())
            .field("sys_data_part0_2", &self.sys_data_part0_2())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS5_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys5::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS5_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS5 to value 0"]
impl crate::Resettable for RD_MAC_SYS5_SPEC {}
