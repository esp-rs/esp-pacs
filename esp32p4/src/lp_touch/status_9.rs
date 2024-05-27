#[doc = "Register `STATUS_9` reader"]
pub type R = crate::R<STATUS_9_SPEC>;
#[doc = "Field `PAD9_DATA` reader - need_des"]
pub type PAD9_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD9_DEBOUNCE_CNT` reader - need_des"]
pub type PAD9_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD9_NEG_NOISE_CNT` reader - need_des"]
pub type PAD9_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad9_data(&self) -> PAD9_DATA_R {
        PAD9_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad9_debounce_cnt(&self) -> PAD9_DEBOUNCE_CNT_R {
        PAD9_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad9_neg_noise_cnt(&self) -> PAD9_NEG_NOISE_CNT_R {
        PAD9_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_9")
            .field("pad9_data", &self.pad9_data())
            .field("pad9_debounce_cnt", &self.pad9_debounce_cnt())
            .field("pad9_neg_noise_cnt", &self.pad9_neg_noise_cnt())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_9_SPEC;
impl crate::RegisterSpec for STATUS_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_9::R`](R) reader structure"]
impl crate::Readable for STATUS_9_SPEC {}
#[doc = "`reset()` method sets STATUS_9 to value 0"]
impl crate::Resettable for STATUS_9_SPEC {
    const RESET_VALUE: u32 = 0;
}
