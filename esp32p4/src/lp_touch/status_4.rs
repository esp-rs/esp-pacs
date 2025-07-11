#[doc = "Register `STATUS_4` reader"]
pub type R = crate::R<STATUS_4_SPEC>;
#[doc = "Field `PAD4_DATA` reader - need_des"]
pub type PAD4_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD4_DEBOUNCE_CNT` reader - need_des"]
pub type PAD4_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD4_NEG_NOISE_CNT` reader - need_des"]
pub type PAD4_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad4_data(&self) -> PAD4_DATA_R {
        PAD4_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad4_debounce_cnt(&self) -> PAD4_DEBOUNCE_CNT_R {
        PAD4_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad4_neg_noise_cnt(&self) -> PAD4_NEG_NOISE_CNT_R {
        PAD4_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_4")
            .field("pad4_data", &self.pad4_data())
            .field("pad4_debounce_cnt", &self.pad4_debounce_cnt())
            .field("pad4_neg_noise_cnt", &self.pad4_neg_noise_cnt())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_4_SPEC;
impl crate::RegisterSpec for STATUS_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_4::R`](R) reader structure"]
impl crate::Readable for STATUS_4_SPEC {}
#[doc = "`reset()` method sets STATUS_4 to value 0"]
impl crate::Resettable for STATUS_4_SPEC {}
