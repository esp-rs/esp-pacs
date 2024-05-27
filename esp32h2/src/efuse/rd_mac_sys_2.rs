#[doc = "Register `RD_MAC_SYS_2` reader"]
pub type R = crate::R<RD_MAC_SYS_2_SPEC>;
#[doc = "Field `RXIQ_VERSION` reader - Stores RF Calibration data. RXIQ version."]
pub type RXIQ_VERSION_R = crate::FieldReader;
#[doc = "Field `RXIQ_0` reader - Stores RF Calibration data. RXIQ data 0."]
pub type RXIQ_0_R = crate::FieldReader;
#[doc = "Field `RXIQ_1` reader - Stores RF Calibration data. RXIQ data 1."]
pub type RXIQ_1_R = crate::FieldReader;
#[doc = "Field `ACTIVE_HP_DBIAS` reader - Stores the PMU active hp dbias."]
pub type ACTIVE_HP_DBIAS_R = crate::FieldReader;
#[doc = "Field `ACTIVE_LP_DBIAS` reader - Stores the PMU active lp dbias."]
pub type ACTIVE_LP_DBIAS_R = crate::FieldReader;
#[doc = "Field `DSLP_DBIAS` reader - Stores the PMU sleep dbias."]
pub type DSLP_DBIAS_R = crate::FieldReader;
#[doc = "Field `DBIAS_VOL_GAP_VALUE1` reader - Stores the low 1 bit of dbias_vol_gap."]
pub type DBIAS_VOL_GAP_VALUE1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Stores RF Calibration data. RXIQ version."]
    #[inline(always)]
    pub fn rxiq_version(&self) -> RXIQ_VERSION_R {
        RXIQ_VERSION_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - Stores RF Calibration data. RXIQ data 0."]
    #[inline(always)]
    pub fn rxiq_0(&self) -> RXIQ_0_R {
        RXIQ_0_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:16 - Stores RF Calibration data. RXIQ data 1."]
    #[inline(always)]
    pub fn rxiq_1(&self) -> RXIQ_1_R {
        RXIQ_1_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 17:21 - Stores the PMU active hp dbias."]
    #[inline(always)]
    pub fn active_hp_dbias(&self) -> ACTIVE_HP_DBIAS_R {
        ACTIVE_HP_DBIAS_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Stores the PMU active lp dbias."]
    #[inline(always)]
    pub fn active_lp_dbias(&self) -> ACTIVE_LP_DBIAS_R {
        ACTIVE_LP_DBIAS_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:30 - Stores the PMU sleep dbias."]
    #[inline(always)]
    pub fn dslp_dbias(&self) -> DSLP_DBIAS_R {
        DSLP_DBIAS_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Stores the low 1 bit of dbias_vol_gap."]
    #[inline(always)]
    pub fn dbias_vol_gap_value1(&self) -> DBIAS_VOL_GAP_VALUE1_R {
        DBIAS_VOL_GAP_VALUE1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS_2")
            .field("rxiq_version", &self.rxiq_version())
            .field("rxiq_0", &self.rxiq_0())
            .field("rxiq_1", &self.rxiq_1())
            .field("active_hp_dbias", &self.active_hp_dbias())
            .field("active_lp_dbias", &self.active_lp_dbias())
            .field("dslp_dbias", &self.dslp_dbias())
            .field("dbias_vol_gap_value1", &self.dbias_vol_gap_value1())
            .finish()
    }
}
#[doc = "BLOCK1 data register $n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS_2_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys_2::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS_2_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS_2 to value 0"]
impl crate::Resettable for RD_MAC_SYS_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
