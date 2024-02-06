#[doc = "Register `STATUS_1` reader"]
pub type R = crate::R<STATUS_1_SPEC>;
#[doc = "Field `PAD1_DATA` reader - need_des"]
pub type PAD1_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PAD1_DEBOUNCE_CNT` reader - need_des"]
pub type PAD1_DEBOUNCE_CNT_R = crate::FieldReader;
#[doc = "Field `PAD1_NEG_NOISE_CNT` reader - need_des"]
pub type PAD1_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pad1_data(&self) -> PAD1_DATA_R {
        PAD1_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn pad1_debounce_cnt(&self) -> PAD1_DEBOUNCE_CNT_R {
        PAD1_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn pad1_neg_noise_cnt(&self) -> PAD1_NEG_NOISE_CNT_R {
        PAD1_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_1")
            .field("pad1_data", &format_args!("{}", self.pad1_data().bits()))
            .field(
                "pad1_debounce_cnt",
                &format_args!("{}", self.pad1_debounce_cnt().bits()),
            )
            .field(
                "pad1_neg_noise_cnt",
                &format_args!("{}", self.pad1_neg_noise_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_1_SPEC;
impl crate::RegisterSpec for STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_1::R`](R) reader structure"]
impl crate::Readable for STATUS_1_SPEC {}
#[doc = "`reset()` method sets STATUS_1 to value 0"]
impl crate::Resettable for STATUS_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
