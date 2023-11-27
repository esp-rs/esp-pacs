#[doc = "Register `STATUS_2` reader"]
pub type R = crate::R<STATUS_2_SPEC>;
#[doc = "Field `PAD2_DATA` reader - need_des"]
pub type PAD2_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD2_DEBOUNCE_CNT` reader - need_des"]
pub type PAD2_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD2_NEG_NOISE_CNT` reader - need_des"]
pub type PAD2_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad2_data(&self) -> PAD2_DATA_R {
        PAD2_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad2_debounce_cnt(&self) -> PAD2_DEBOUNCE_CNT_R {
        PAD2_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad2_neg_noise_cnt(&self) -> PAD2_NEG_NOISE_CNT_R {
        PAD2_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_2")
            .field("pad2_data", &format_args!("{}", self.pad2_data().bits()))
            .field(
                "pad2_debounce_cnt",
                &format_args!("{}", self.pad2_debounce_cnt().bits()),
            )
            .field(
                "pad2_neg_noise_cnt",
                &format_args!("{}", self.pad2_neg_noise_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_2_SPEC;
impl crate::RegisterSpec for STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_2::R`](R) reader structure"]
impl crate::Readable for STATUS_2_SPEC {}
#[doc = "`reset()` method sets STATUS_2 to value 0"]
impl crate::Resettable for STATUS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
