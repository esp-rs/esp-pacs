///Register `STATUS_14` reader
pub type R = crate::R<STATUS_14_SPEC>;
///Field `PAD14_DATA` reader - need_des
pub type PAD14_DATA_R = crate::FieldReader<u16>;
///Field `PAD14_DEBOUNCE_CNT` reader - need_des
pub type PAD14_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD14_NEG_NOISE_CNT` reader - need_des
pub type PAD14_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad14_data(&self) -> PAD14_DATA_R {
        PAD14_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad14_debounce_cnt(&self) -> PAD14_DEBOUNCE_CNT_R {
        PAD14_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad14_neg_noise_cnt(&self) -> PAD14_NEG_NOISE_CNT_R {
        PAD14_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_14")
            .field("pad14_data", &self.pad14_data())
            .field("pad14_debounce_cnt", &self.pad14_debounce_cnt())
            .field("pad14_neg_noise_cnt", &self.pad14_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_14_SPEC;
impl crate::RegisterSpec for STATUS_14_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_14::R`](R) reader structure
impl crate::Readable for STATUS_14_SPEC {}
///`reset()` method sets STATUS_14 to value 0
impl crate::Resettable for STATUS_14_SPEC {
    const RESET_VALUE: u32 = 0;
}
