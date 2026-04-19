#[doc = "Register `DLY_NUM_REC3` reader"]
pub type R = crate::R<DLY_NUM_REC3_SPEC>;
#[doc = "Field `SITE3_DELAY_NUM_VT0_MAX_RECORD` reader - needs field desc"]
pub type SITE3_DELAY_NUM_VT0_MAX_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE3_DELAY_NUM_VT1_MAX_RECORD` reader - needs field desc"]
pub type SITE3_DELAY_NUM_VT1_MAX_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE3_DELAY_NUM_VT2_MAX_RECORD` reader - needs field desc"]
pub type SITE3_DELAY_NUM_VT2_MAX_RECORD_R = crate::FieldReader;
#[doc = "Field `SITE3_DELAY_NUM_VT3_MAX_RECORD` reader - needs field desc"]
pub type SITE3_DELAY_NUM_VT3_MAX_RECORD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - needs field desc"]
    #[inline(always)]
    pub fn site3_delay_num_vt0_max_record(&self) -> SITE3_DELAY_NUM_VT0_MAX_RECORD_R {
        SITE3_DELAY_NUM_VT0_MAX_RECORD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - needs field desc"]
    #[inline(always)]
    pub fn site3_delay_num_vt1_max_record(&self) -> SITE3_DELAY_NUM_VT1_MAX_RECORD_R {
        SITE3_DELAY_NUM_VT1_MAX_RECORD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - needs field desc"]
    #[inline(always)]
    pub fn site3_delay_num_vt2_max_record(&self) -> SITE3_DELAY_NUM_VT2_MAX_RECORD_R {
        SITE3_DELAY_NUM_VT2_MAX_RECORD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - needs field desc"]
    #[inline(always)]
    pub fn site3_delay_num_vt3_max_record(&self) -> SITE3_DELAY_NUM_VT3_MAX_RECORD_R {
        SITE3_DELAY_NUM_VT3_MAX_RECORD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY_NUM_REC3")
            .field(
                "site3_delay_num_vt0_max_record",
                &self.site3_delay_num_vt0_max_record(),
            )
            .field(
                "site3_delay_num_vt1_max_record",
                &self.site3_delay_num_vt1_max_record(),
            )
            .field(
                "site3_delay_num_vt2_max_record",
                &self.site3_delay_num_vt2_max_record(),
            )
            .field(
                "site3_delay_num_vt3_max_record",
                &self.site3_delay_num_vt3_max_record(),
            )
            .finish()
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dly_num_rec3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLY_NUM_REC3_SPEC;
impl crate::RegisterSpec for DLY_NUM_REC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly_num_rec3::R`](R) reader structure"]
impl crate::Readable for DLY_NUM_REC3_SPEC {}
#[doc = "`reset()` method sets DLY_NUM_REC3 to value 0"]
impl crate::Resettable for DLY_NUM_REC3_SPEC {}
