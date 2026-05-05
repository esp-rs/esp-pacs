#[doc = "Register `CHN_TMP_STATUS` reader"]
pub type R = crate::R<CHN_TMP_STATUS_SPEC>;
#[doc = "Field `PAD_INACTIVE_STATUS` reader - need_des"]
pub type PAD_INACTIVE_STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `PAD_ACTIVE_STATUS` reader - need_des"]
pub type PAD_ACTIVE_STATUS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    pub fn pad_inactive_status(&self) -> PAD_INACTIVE_STATUS_R {
        PAD_INACTIVE_STATUS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    pub fn pad_active_status(&self) -> PAD_ACTIVE_STATUS_R {
        PAD_ACTIVE_STATUS_R::new(((self.bits >> 15) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHN_TMP_STATUS")
            .field("pad_inactive_status", &self.pad_inactive_status())
            .field("pad_active_status", &self.pad_active_status())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_tmp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHN_TMP_STATUS_SPEC;
impl crate::RegisterSpec for CHN_TMP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_tmp_status::R`](R) reader structure"]
impl crate::Readable for CHN_TMP_STATUS_SPEC {}
#[doc = "`reset()` method sets CHN_TMP_STATUS to value 0"]
impl crate::Resettable for CHN_TMP_STATUS_SPEC {}
