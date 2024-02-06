#[doc = "Register `STATUS_14` reader"]
pub type R = crate::R<STATUS_14_SPEC>;
#[doc = "Field `PAD14_DATA` reader - need_des"]
pub type PAD14_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD14_DEBOUNCE_CNT` reader - need_des"]
pub type PAD14_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD14_NEG_NOISE_CNT` reader - need_des"]
pub type PAD14_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad14_data(&self) -> PAD14_DATA_R {
        PAD14_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad14_debounce_cnt(&self) -> PAD14_DEBOUNCE_CNT_R {
        PAD14_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad14_neg_noise_cnt(&self) -> PAD14_NEG_NOISE_CNT_R {
        PAD14_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_14")
            .field("pad14_data", &format_args!("{}", self.pad14_data().bits()))
            .field(
                "pad14_debounce_cnt",
                &format_args!("{}", self.pad14_debounce_cnt().bits()),
            )
            .field(
                "pad14_neg_noise_cnt",
                &format_args!("{}", self.pad14_neg_noise_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_14_SPEC;
impl crate::RegisterSpec for STATUS_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_14::R`](R) reader structure"]
impl crate::Readable for STATUS_14_SPEC {}
#[doc = "`reset()` method sets STATUS_14 to value 0"]
impl crate::Resettable for STATUS_14_SPEC {
    const RESET_VALUE: u32 = 0;
}
