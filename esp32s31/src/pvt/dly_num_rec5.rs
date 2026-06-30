#[doc = "Register `DLY_NUM_REC5` reader"]
pub type R = crate::R<DLY_NUM_REC5_SPEC>;
#[doc = "Field `SITE1_DELAY_NUM_VT0_MIN_RECORD` reader - needs field desc"]
pub type SITE1_DELAY_NUM_VT0_MIN_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE1_DELAY_NUM_VT1_MIN_RECORD` reader - needs field desc"]
pub type SITE1_DELAY_NUM_VT1_MIN_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE1_DELAY_NUM_VT2_MIN_RECORD` reader - needs field desc"]
pub type SITE1_DELAY_NUM_VT2_MIN_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE1_DELAY_NUM_VT3_MIN_RECORD` reader - needs field desc"]
pub type SITE1_DELAY_NUM_VT3_MIN_RECORD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - needs field desc"]
    #[inline(always)]
    pub fn site1_delay_num_vt0_min_record(&self) -> SITE1_DELAY_NUM_VT0_MIN_RECORD_R {
        SITE1_DELAY_NUM_VT0_MIN_RECORD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - needs field desc"]
    #[inline(always)]
    pub fn site1_delay_num_vt1_min_record(&self) -> SITE1_DELAY_NUM_VT1_MIN_RECORD_R {
        SITE1_DELAY_NUM_VT1_MIN_RECORD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - needs field desc"]
    #[inline(always)]
    pub fn site1_delay_num_vt2_min_record(&self) -> SITE1_DELAY_NUM_VT2_MIN_RECORD_R {
        SITE1_DELAY_NUM_VT2_MIN_RECORD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - needs field desc"]
    #[inline(always)]
    pub fn site1_delay_num_vt3_min_record(&self) -> SITE1_DELAY_NUM_VT3_MIN_RECORD_R {
        SITE1_DELAY_NUM_VT3_MIN_RECORD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY_NUM_REC5")
            .field(
                "site1_delay_num_vt0_min_record",
                &self.site1_delay_num_vt0_min_record(),
            )
            .field(
                "site1_delay_num_vt1_min_record",
                &self.site1_delay_num_vt1_min_record(),
            )
            .field(
                "site1_delay_num_vt2_min_record",
                &self.site1_delay_num_vt2_min_record(),
            )
            .field(
                "site1_delay_num_vt3_min_record",
                &self.site1_delay_num_vt3_min_record(),
            )
            .finish()
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dly_num_rec5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLY_NUM_REC5_SPEC;
impl crate::RegisterSpec for DLY_NUM_REC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly_num_rec5::R`](R) reader structure"]
impl crate::Readable for DLY_NUM_REC5_SPEC {}
#[doc = "`reset()` method sets DLY_NUM_REC5 to value 0"]
impl crate::Resettable for DLY_NUM_REC5_SPEC {}
