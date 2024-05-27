///Register `STATUS_7` reader
pub type R = crate::R<STATUS_7_SPEC>;
///Field `PAD7_DATA` reader - need_des
pub type PAD7_DATA_R = crate::FieldReader<u16>;
///Field `PAD7_DEBOUNCE_CNT` reader - need_des
pub type PAD7_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD7_NEG_NOISE_CNT` reader - need_des
pub type PAD7_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad7_data(&self) -> PAD7_DATA_R {
        PAD7_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad7_debounce_cnt(&self) -> PAD7_DEBOUNCE_CNT_R {
        PAD7_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad7_neg_noise_cnt(&self) -> PAD7_NEG_NOISE_CNT_R {
        PAD7_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_7")
            .field("pad7_data", &self.pad7_data())
            .field("pad7_debounce_cnt", &self.pad7_debounce_cnt())
            .field("pad7_neg_noise_cnt", &self.pad7_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_7_SPEC;
impl crate::RegisterSpec for STATUS_7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_7::R`](R) reader structure
impl crate::Readable for STATUS_7_SPEC {}
///`reset()` method sets STATUS_7 to value 0
impl crate::Resettable for STATUS_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
