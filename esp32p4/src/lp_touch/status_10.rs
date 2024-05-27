///Register `STATUS_10` reader
pub type R = crate::R<STATUS_10_SPEC>;
///Field `PAD10_DATA` reader - need_des
pub type PAD10_DATA_R = crate::FieldReader<u16>;
///Field `PAD10_DEBOUNCE_CNT` reader - need_des
pub type PAD10_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD10_NEG_NOISE_CNT` reader - need_des
pub type PAD10_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad10_data(&self) -> PAD10_DATA_R {
        PAD10_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad10_debounce_cnt(&self) -> PAD10_DEBOUNCE_CNT_R {
        PAD10_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad10_neg_noise_cnt(&self) -> PAD10_NEG_NOISE_CNT_R {
        PAD10_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_10")
            .field("pad10_data", &self.pad10_data())
            .field("pad10_debounce_cnt", &self.pad10_debounce_cnt())
            .field("pad10_neg_noise_cnt", &self.pad10_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_10_SPEC;
impl crate::RegisterSpec for STATUS_10_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_10::R`](R) reader structure
impl crate::Readable for STATUS_10_SPEC {}
///`reset()` method sets STATUS_10 to value 0
impl crate::Resettable for STATUS_10_SPEC {
    const RESET_VALUE: u32 = 0;
}
