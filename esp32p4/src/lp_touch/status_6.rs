#[doc = "Register `STATUS_6` reader"]
pub type R = crate::R<STATUS_6_SPEC>;
#[doc = "Field `PAD6_DATA` reader - need_des"]
pub type PAD6_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD6_DEBOUNCE_CNT` reader - need_des"]
pub type PAD6_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD6_NEG_NOISE_CNT` reader - need_des"]
pub type PAD6_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad6_data(&self) -> PAD6_DATA_R {
        PAD6_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad6_debounce_cnt(&self) -> PAD6_DEBOUNCE_CNT_R {
        PAD6_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad6_neg_noise_cnt(&self) -> PAD6_NEG_NOISE_CNT_R {
        PAD6_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_6")
            .field("pad6_data", &self.pad6_data())
            .field("pad6_debounce_cnt", &self.pad6_debounce_cnt())
            .field("pad6_neg_noise_cnt", &self.pad6_neg_noise_cnt())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_6_SPEC;
impl crate::RegisterSpec for STATUS_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_6::R`](R) reader structure"]
impl crate::Readable for STATUS_6_SPEC {}
#[doc = "`reset()` method sets STATUS_6 to value 0"]
impl crate::Resettable for STATUS_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
