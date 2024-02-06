#[doc = "Register `STATUS_7` reader"]
pub type R = crate::R<STATUS_7_SPEC>;
#[doc = "Field `PAD7_DATA` reader - need_des"]
pub type PAD7_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD7_DEBOUNCE_CNT` reader - need_des"]
pub type PAD7_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD7_NEG_NOISE_CNT` reader - need_des"]
pub type PAD7_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad7_data(&self) -> PAD7_DATA_R {
        PAD7_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad7_debounce_cnt(&self) -> PAD7_DEBOUNCE_CNT_R {
        PAD7_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad7_neg_noise_cnt(&self) -> PAD7_NEG_NOISE_CNT_R {
        PAD7_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_7")
            .field("pad7_data", &format_args!("{}", self.pad7_data().bits()))
            .field(
                "pad7_debounce_cnt",
                &format_args!("{}", self.pad7_debounce_cnt().bits()),
            )
            .field(
                "pad7_neg_noise_cnt",
                &format_args!("{}", self.pad7_neg_noise_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_7_SPEC;
impl crate::RegisterSpec for STATUS_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_7::R`](R) reader structure"]
impl crate::Readable for STATUS_7_SPEC {}
#[doc = "`reset()` method sets STATUS_7 to value 0"]
impl crate::Resettable for STATUS_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
