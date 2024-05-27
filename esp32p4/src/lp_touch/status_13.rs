///Register `STATUS_13` reader
pub type R = crate::R<STATUS_13_SPEC>;
///Field `PAD13_DATA` reader - need_des
pub type PAD13_DATA_R = crate::FieldReader<u16>;
///Field `PAD13_DEBOUNCE_CNT` reader - need_des
pub type PAD13_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD13_NEG_NOISE_CNT` reader - need_des
pub type PAD13_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad13_data(&self) -> PAD13_DATA_R {
        PAD13_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad13_debounce_cnt(&self) -> PAD13_DEBOUNCE_CNT_R {
        PAD13_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad13_neg_noise_cnt(&self) -> PAD13_NEG_NOISE_CNT_R {
        PAD13_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_13")
            .field("pad13_data", &self.pad13_data())
            .field("pad13_debounce_cnt", &self.pad13_debounce_cnt())
            .field("pad13_neg_noise_cnt", &self.pad13_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_13_SPEC;
impl crate::RegisterSpec for STATUS_13_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_13::R`](R) reader structure
impl crate::Readable for STATUS_13_SPEC {}
///`reset()` method sets STATUS_13 to value 0
impl crate::Resettable for STATUS_13_SPEC {
    const RESET_VALUE: u32 = 0;
}
