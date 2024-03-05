#[doc = "Register `RTCCALICFG2` reader"]
pub type R = crate::R<RTCCALICFG2_SPEC>;
#[doc = "Register `RTCCALICFG2` writer"]
pub type W = crate::W<RTCCALICFG2_SPEC>;
#[doc = "Field `RTC_CALI_TIMEOUT` reader - timeoutindicator"]
pub type RTC_CALI_TIMEOUT_R = crate::BitReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` reader - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
pub type RTC_CALI_TIMEOUT_RST_CNT_R = crate::FieldReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` writer - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
pub type RTC_CALI_TIMEOUT_RST_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` reader - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
pub type RTC_CALI_TIMEOUT_THRES_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` writer - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
pub type RTC_CALI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - timeoutindicator"]
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> RTC_CALI_TIMEOUT_R {
        RTC_CALI_TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> RTC_CALI_TIMEOUT_RST_CNT_R {
        RTC_CALI_TIMEOUT_RST_CNT_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:31 - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> RTC_CALI_TIMEOUT_THRES_R {
        RTC_CALI_TIMEOUT_THRES_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG2")
            .field(
                "rtc_cali_timeout",
                &format_args!("{}", self.rtc_cali_timeout().bit()),
            )
            .field(
                "rtc_cali_timeout_rst_cnt",
                &format_args!("{}", self.rtc_cali_timeout_rst_cnt().bits()),
            )
            .field(
                "rtc_cali_timeout_thres",
                &format_args!("{}", self.rtc_cali_timeout_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTCCALICFG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 3:6 - reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_timeout_rst_cnt(&mut self) -> RTC_CALI_TIMEOUT_RST_CNT_W<RTCCALICFG2_SPEC> {
        RTC_CALI_TIMEOUT_RST_CNT_W::new(self, 3)
    }
    #[doc = "Bits 7:31 - reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_timeout_thres(&mut self) -> RTC_CALI_TIMEOUT_THRES_W<RTCCALICFG2_SPEC> {
        RTC_CALI_TIMEOUT_THRES_W::new(self, 7)
    }
}
#[doc = "TIMG_RTCCALICFG2_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG2_SPEC;
impl crate::RegisterSpec for RTCCALICFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg2::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccalicfg2::W`](W) writer structure"]
impl crate::Writable for RTCCALICFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCCALICFG2 to value 0xffff_ff98"]
impl crate::Resettable for RTCCALICFG2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ff98;
}
