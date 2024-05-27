#[doc = "Register `SAR_TOUCH_STATUS16` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS16_SPEC>;
#[doc = "Field `TOUCH_APPROACH_PAD2_CNT` reader - Count status of proximity pad 2."]
pub type TOUCH_APPROACH_PAD2_CNT_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD1_CNT` reader - Count status of proximity pad 1."]
pub type TOUCH_APPROACH_PAD1_CNT_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD0_CNT` reader - Count status of proximity pad 0."]
pub type TOUCH_APPROACH_PAD0_CNT_R = crate::FieldReader;
#[doc = "Field `TOUCH_SLP_APPROACH_CNT` reader - Count status of sleep pad in proximity mode."]
pub type TOUCH_SLP_APPROACH_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Count status of proximity pad 2."]
    #[inline(always)]
    pub fn touch_approach_pad2_cnt(&self) -> TOUCH_APPROACH_PAD2_CNT_R {
        TOUCH_APPROACH_PAD2_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Count status of proximity pad 1."]
    #[inline(always)]
    pub fn touch_approach_pad1_cnt(&self) -> TOUCH_APPROACH_PAD1_CNT_R {
        TOUCH_APPROACH_PAD1_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count status of proximity pad 0."]
    #[inline(always)]
    pub fn touch_approach_pad0_cnt(&self) -> TOUCH_APPROACH_PAD0_CNT_R {
        TOUCH_APPROACH_PAD0_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Count status of sleep pad in proximity mode."]
    #[inline(always)]
    pub fn touch_slp_approach_cnt(&self) -> TOUCH_SLP_APPROACH_CNT_R {
        TOUCH_SLP_APPROACH_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS16")
            .field("touch_approach_pad2_cnt", &self.touch_approach_pad2_cnt())
            .field("touch_approach_pad1_cnt", &self.touch_approach_pad1_cnt())
            .field("touch_approach_pad0_cnt", &self.touch_approach_pad0_cnt())
            .field("touch_slp_approach_cnt", &self.touch_slp_approach_cnt())
            .finish()
    }
}
#[doc = "Touch approach count status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS16_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status16::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS16_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS16 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS16_SPEC {
    const RESET_VALUE: u32 = 0;
}
