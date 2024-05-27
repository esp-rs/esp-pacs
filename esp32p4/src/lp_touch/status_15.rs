///Register `STATUS_15` reader
pub type R = crate::R<STATUS_15_SPEC>;
///Field `SLP_DATA` reader - need_des
pub type SLP_DATA_R = crate::FieldReader<u16>;
///Field `SLP_DEBOUNCE_CNT` reader - need_des
pub type SLP_DEBOUNCE_CNT_R = crate::FieldReader;
///Field `SLP_NEG_NOISE_CNT` reader - need_des
pub type SLP_NEG_NOISE_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn slp_data(&self) -> SLP_DATA_R {
        SLP_DATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - need_des
    #[inline(always)]
    pub fn slp_debounce_cnt(&self) -> SLP_DEBOUNCE_CNT_R {
        SLP_DEBOUNCE_CNT_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:22 - need_des
    #[inline(always)]
    pub fn slp_neg_noise_cnt(&self) -> SLP_NEG_NOISE_CNT_R {
        SLP_NEG_NOISE_CNT_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_15")
            .field("slp_data", &self.slp_data())
            .field("slp_debounce_cnt", &self.slp_debounce_cnt())
            .field("slp_neg_noise_cnt", &self.slp_neg_noise_cnt())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`status_15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_15_SPEC;
impl crate::RegisterSpec for STATUS_15_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_15::R`](R) reader structure
impl crate::Readable for STATUS_15_SPEC {}
///`reset()` method sets STATUS_15 to value 0
impl crate::Resettable for STATUS_15_SPEC {
    const RESET_VALUE: u32 = 0;
}
