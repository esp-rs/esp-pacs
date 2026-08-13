#[doc = "Register `CHANNEL_STATUS` reader"]
pub type R = crate::R<CHANNEL_STATUS_SPEC>;
#[doc = "Field `CHANNEL_3_PAD_COMP_STATUS` reader - record the pad comp status for channel 3, 0 means now is neg int , 1 means now is pos int"]
pub type CHANNEL_3_PAD_COMP_STATUS_R = crate::BitReader;
#[doc = "Field `CHANNEL_2_PAD_COMP_STATUS` reader - record the pad comp status for channel 2, 0 means now is neg int , 1 means now is pos int"]
pub type CHANNEL_2_PAD_COMP_STATUS_R = crate::BitReader;
#[doc = "Field `CHANNEL_1_PAD_COMP_STATUS` reader - record the pad comp status for channel 1, 0 means now is neg int , 1 means now is pos int"]
pub type CHANNEL_1_PAD_COMP_STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - record the pad comp status for channel 3, 0 means now is neg int , 1 means now is pos int"]
    #[inline(always)]
    pub fn channel_3_pad_comp_status(&self) -> CHANNEL_3_PAD_COMP_STATUS_R {
        CHANNEL_3_PAD_COMP_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - record the pad comp status for channel 2, 0 means now is neg int , 1 means now is pos int"]
    #[inline(always)]
    pub fn channel_2_pad_comp_status(&self) -> CHANNEL_2_PAD_COMP_STATUS_R {
        CHANNEL_2_PAD_COMP_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - record the pad comp status for channel 1, 0 means now is neg int , 1 means now is pos int"]
    #[inline(always)]
    pub fn channel_1_pad_comp_status(&self) -> CHANNEL_1_PAD_COMP_STATUS_R {
        CHANNEL_1_PAD_COMP_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANNEL_STATUS")
            .field(
                "channel_3_pad_comp_status",
                &self.channel_3_pad_comp_status(),
            )
            .field(
                "channel_2_pad_comp_status",
                &self.channel_2_pad_comp_status(),
            )
            .field(
                "channel_1_pad_comp_status",
                &self.channel_1_pad_comp_status(),
            )
            .finish()
    }
}
#[doc = "pad comp status reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_STATUS_SPEC;
impl crate::RegisterSpec for CHANNEL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_status::R`](R) reader structure"]
impl crate::Readable for CHANNEL_STATUS_SPEC {}
#[doc = "`reset()` method sets CHANNEL_STATUS to value 0"]
impl crate::Resettable for CHANNEL_STATUS_SPEC {}
