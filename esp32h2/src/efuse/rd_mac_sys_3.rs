#[doc = "Register `RD_MAC_SYS_3` reader"]
pub type R = crate::R<RD_MAC_SYS_3_SPEC>;
#[doc = "Field `DBIAS_VOL_GAP_VALUE2` reader - Stores the high 3 bits of dbias_vol_gap."]
pub type DBIAS_VOL_GAP_VALUE2_R = crate::FieldReader;
#[doc = "Field `DBIAS_VOL_GAP_SIGN` reader - Stores the sign bit of dbias_vol_gap."]
pub type DBIAS_VOL_GAP_SIGN_R = crate::BitReader;
#[doc = "Field `MAC_RESERVED_2` reader - Reserved."]
pub type MAC_RESERVED_2_R = crate::FieldReader<u16>;
#[doc = "Field `WAFER_VERSION_MINOR` reader - Stores the wafer version minor."]
pub type WAFER_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MAJOR` reader - Stores the wafer version major."]
pub type WAFER_VERSION_MAJOR_R = crate::FieldReader;
#[doc = "Field `DISABLE_WAFER_VERSION_MAJOR` reader - Disables check of wafer version major."]
pub type DISABLE_WAFER_VERSION_MAJOR_R = crate::BitReader;
#[doc = "Field `FLASH_CAP` reader - Stores the flash cap."]
pub type FLASH_CAP_R = crate::FieldReader;
#[doc = "Field `FLASH_TEMP` reader - Stores the flash temp."]
pub type FLASH_TEMP_R = crate::FieldReader;
#[doc = "Field `FLASH_VENDOR` reader - Stores the flash vendor."]
pub type FLASH_VENDOR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Stores the high 3 bits of dbias_vol_gap."]
    #[inline(always)]
    pub fn dbias_vol_gap_value2(&self) -> DBIAS_VOL_GAP_VALUE2_R {
        DBIAS_VOL_GAP_VALUE2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Stores the sign bit of dbias_vol_gap."]
    #[inline(always)]
    pub fn dbias_vol_gap_sign(&self) -> DBIAS_VOL_GAP_SIGN_R {
        DBIAS_VOL_GAP_SIGN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:17 - Reserved."]
    #[inline(always)]
    pub fn mac_reserved_2(&self) -> MAC_RESERVED_2_R {
        MAC_RESERVED_2_R::new(((self.bits >> 4) & 0x3fff) as u16)
    }
    #[doc = "Bits 18:20 - Stores the wafer version minor."]
    #[inline(always)]
    pub fn wafer_version_minor(&self) -> WAFER_VERSION_MINOR_R {
        WAFER_VERSION_MINOR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Stores the wafer version major."]
    #[inline(always)]
    pub fn wafer_version_major(&self) -> WAFER_VERSION_MAJOR_R {
        WAFER_VERSION_MAJOR_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Disables check of wafer version major."]
    #[inline(always)]
    pub fn disable_wafer_version_major(&self) -> DISABLE_WAFER_VERSION_MAJOR_R {
        DISABLE_WAFER_VERSION_MAJOR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Stores the flash cap."]
    #[inline(always)]
    pub fn flash_cap(&self) -> FLASH_CAP_R {
        FLASH_CAP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - Stores the flash temp."]
    #[inline(always)]
    pub fn flash_temp(&self) -> FLASH_TEMP_R {
        FLASH_TEMP_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - Stores the flash vendor."]
    #[inline(always)]
    pub fn flash_vendor(&self) -> FLASH_VENDOR_R {
        FLASH_VENDOR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS_3")
            .field(
                "dbias_vol_gap_value2",
                &format_args!("{}", self.dbias_vol_gap_value2().bits()),
            )
            .field(
                "dbias_vol_gap_sign",
                &format_args!("{}", self.dbias_vol_gap_sign().bit()),
            )
            .field(
                "mac_reserved_2",
                &format_args!("{}", self.mac_reserved_2().bits()),
            )
            .field(
                "wafer_version_minor",
                &format_args!("{}", self.wafer_version_minor().bits()),
            )
            .field(
                "wafer_version_major",
                &format_args!("{}", self.wafer_version_major().bits()),
            )
            .field(
                "disable_wafer_version_major",
                &format_args!("{}", self.disable_wafer_version_major().bit()),
            )
            .field("flash_cap", &format_args!("{}", self.flash_cap().bits()))
            .field("flash_temp", &format_args!("{}", self.flash_temp().bits()))
            .field(
                "flash_vendor",
                &format_args!("{}", self.flash_vendor().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_MAC_SYS_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "BLOCK1 data register $n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS_3_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys_3::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS_3_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS_3 to value 0"]
impl crate::Resettable for RD_MAC_SYS_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
