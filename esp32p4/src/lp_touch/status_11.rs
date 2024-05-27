///Register `STATUS_11` reader
pub type R = crate::R<STATUS_11_SPEC>;
///Field `PAD11_DATA` reader - need_des
pub type PAD11_DATA_R = crate::FieldReader<u16>;
///Field `PAD11_DEBOUNCE_CNT` reader - need_des
pub type PAD11_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `PAD11_NEG_NOISE_CNT` reader - need_des
pub type PAD11_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn pad11_data(&self) -> PAD11_DATA_R {
        PAD11_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn pad11_debounce_cnt(&self) -> PAD11_DEBOUNCE_CNT_R {
        PAD11_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn pad11_neg_noise_cnt(&self) -> PAD11_NEG_NOISE_CNT_R {
        PAD11_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_11")
            .field("pad11_data", &self.pad11_data())
            .field("pad11_debounce_cnt", &self.pad11_debounce_cnt())
            .field("pad11_neg_noise_cnt", &self.pad11_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_11_SPEC;
impl crate::RegisterSpec for STATUS_11_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_11::R`](R) reader structure
impl crate::Readable for STATUS_11_SPEC {}
///`reset()` method sets STATUS_11 to value 0
impl crate::Resettable for STATUS_11_SPEC {
    const RESET_VALUE: u32 = 0;
}
