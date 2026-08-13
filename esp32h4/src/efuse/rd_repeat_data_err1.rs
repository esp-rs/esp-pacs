#[doc = "Register `RD_REPEAT_DATA_ERR1` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR1_SPEC>;
#[doc = "Field `KEY_PURPOSE_0_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_0"]
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_1"]
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
pub type KEY_PURPOSE_2_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
pub type KEY_PURPOSE_3_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
pub type KEY_PURPOSE_4_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
pub type KEY_PURPOSE_5_ERR_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL_ERR` reader - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
pub type SEC_DPA_LEVEL_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Represents the programming error of EFUSE_KEY_PURPOSE_0"]
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Represents the programming error of EFUSE_KEY_PURPOSE_1"]
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
    #[inline(always)]
    pub fn sec_dpa_level_err(&self) -> SEC_DPA_LEVEL_ERR_R {
        SEC_DPA_LEVEL_ERR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR1")
            .field("key_purpose_0_err", &self.key_purpose_0_err())
            .field("key_purpose_1_err", &self.key_purpose_1_err())
            .field("key_purpose_2_err", &self.key_purpose_2_err())
            .field("key_purpose_3_err", &self.key_purpose_3_err())
            .field("key_purpose_4_err", &self.key_purpose_4_err())
            .field("key_purpose_5_err", &self.key_purpose_5_err())
            .field("sec_dpa_level_err", &self.sec_dpa_level_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR1_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR1_SPEC {}
