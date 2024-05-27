///Register `STATUS_5` reader
pub type R = crate::R<STATUS_5_SPEC>;
///Field `PAD5_DATA` reader - need_des
pub type PAD5_DATA_R = crate::FieldReader<u16>;
///Field `PAD5_DEBOUNCE_CNT` reader - need_des
pub type PAD5_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD5_NEG_NOISE_CNT` reader - need_des
pub type PAD5_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad5_data(&self) -> PAD5_DATA_R {
        PAD5_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad5_debounce_cnt(&self) -> PAD5_DEBOUNCE_CNT_R {
        PAD5_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad5_neg_noise_cnt(&self) -> PAD5_NEG_NOISE_CNT_R {
        PAD5_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_5")
            .field("pad5_data", &self.pad5_data())
            .field("pad5_debounce_cnt", &self.pad5_debounce_cnt())
            .field("pad5_neg_noise_cnt", &self.pad5_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_5_SPEC;
impl crate::RegisterSpec for STATUS_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_5::R`](R) reader structure
impl crate::Readable for STATUS_5_SPEC {}
///`reset()` method sets STATUS_5 to value 0
impl crate::Resettable for STATUS_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
