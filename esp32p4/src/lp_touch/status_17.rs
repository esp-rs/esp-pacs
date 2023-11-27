#[doc = "Register `STATUS_17` reader"]
pub type R = crate::R<STATUS_17_SPEC>;
#[doc = "Field `DCAP_LPF` reader - Reserved"]
pub type DCAP_LPF_R = crate::FieldReader;
#[doc = "Field `DRES_LPF` reader - need_des"]
pub type DRES_LPF_R = crate::FieldReader;
#[doc = "Field `DRV_LS` reader - need_des"]
pub type DRV_LS_R = crate::FieldReader;
#[doc = "Field `DRV_HS` reader - need_des"]
pub type DRV_HS_R = crate::FieldReader;
#[doc = "Field `DBIAS` reader - need_des"]
pub type DBIAS_R = crate::FieldReader;
#[doc = "Field `RTC_FREQ_SCAN_CNT` reader - need_des"]
pub type RTC_FREQ_SCAN_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn dcap_lpf(&self) -> DCAP_LPF_R {
        DCAP_LPF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn dres_lpf(&self) -> DRES_LPF_R {
        DRES_LPF_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn drv_ls(&self) -> DRV_LS_R {
        DRV_LS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn drv_hs(&self) -> DRV_HS_R {
        DRV_HS_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn dbias(&self) -> DBIAS_R {
        DBIAS_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn rtc_freq_scan_cnt(&self) -> RTC_FREQ_SCAN_CNT_R {
        RTC_FREQ_SCAN_CNT_R::new(((self.bits >> 23) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_17")
            .field("dcap_lpf", &format_args!("{}", self.dcap_lpf().bits()))
            .field("dres_lpf", &format_args!("{}", self.dres_lpf().bits()))
            .field("drv_ls", &format_args!("{}", self.drv_ls().bits()))
            .field("drv_hs", &format_args!("{}", self.drv_hs().bits()))
            .field("dbias", &format_args!("{}", self.dbias().bits()))
            .field(
                "rtc_freq_scan_cnt",
                &format_args!("{}", self.rtc_freq_scan_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_17_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_17_SPEC;
impl crate::RegisterSpec for STATUS_17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_17::R`](R) reader structure"]
impl crate::Readable for STATUS_17_SPEC {}
#[doc = "`reset()` method sets STATUS_17 to value 0"]
impl crate::Resettable for STATUS_17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
