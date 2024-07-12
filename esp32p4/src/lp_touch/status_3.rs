#[doc = "Register `STATUS_3` reader"]
pub type R = crate::R<STATUS_3_SPEC>;
#[doc = "Field `PAD3_DATA` reader - need_des"]
pub type PAD3_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD3_DEBOUNCE_CNT` reader - need_des"]
pub type PAD3_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD3_NEG_NOISE_CNT` reader - need_des"]
pub type PAD3_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad3_data(&self) -> PAD3_DATA_R {
        PAD3_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad3_debounce_cnt(&self) -> PAD3_DEBOUNCE_CNT_R {
        PAD3_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad3_neg_noise_cnt(&self) -> PAD3_NEG_NOISE_CNT_R {
        PAD3_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_3")
            .field("pad3_data", &self.pad3_data())
            .field("pad3_debounce_cnt", &self.pad3_debounce_cnt())
            .field("pad3_neg_noise_cnt", &self.pad3_neg_noise_cnt())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_3_SPEC;
impl crate::RegisterSpec for STATUS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_3::R`](R) reader structure"]
impl crate::Readable for STATUS_3_SPEC {}
#[doc = "`reset()` method sets STATUS_3 to value 0"]
impl crate::Resettable for STATUS_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
