///Register `STATUS_8` reader
pub type R = crate::R<STATUS_8_SPEC>;
///Field `PAD8_DATA` reader - need_des
pub type PAD8_DATA_R = crate::FieldReader<u16>;
///Field `PAD8_DEBOUNCE_CNT` reader - need_des
pub type PAD8_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD8_NEG_NOISE_CNT` reader - need_des
pub type PAD8_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad8_data(&self) -> PAD8_DATA_R {
        PAD8_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad8_debounce_cnt(&self) -> PAD8_DEBOUNCE_CNT_R {
        PAD8_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad8_neg_noise_cnt(&self) -> PAD8_NEG_NOISE_CNT_R {
        PAD8_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_8")
            .field("pad8_data", &self.pad8_data())
            .field("pad8_debounce_cnt", &self.pad8_debounce_cnt())
            .field("pad8_neg_noise_cnt", &self.pad8_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_8_SPEC;
impl crate::RegisterSpec for STATUS_8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_8::R`](R) reader structure
impl crate::Readable for STATUS_8_SPEC {}
///`reset()` method sets STATUS_8 to value 0
impl crate::Resettable for STATUS_8_SPEC {
    const RESET_VALUE: u32 = 0;
}
