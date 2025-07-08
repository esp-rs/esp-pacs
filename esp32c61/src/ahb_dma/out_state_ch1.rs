#[doc = "Register `OUT_STATE_CH1` reader"]
pub type R = crate::R<OUT_STATE_CH1_SPEC>;
#[doc = "Field `OUTLINK_DSCR_ADDR_CH1` reader - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
pub type OUTLINK_DSCR_ADDR_CH1_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE_CH1` reader - reserved"]
pub type OUT_DSCR_STATE_CH1_R = crate::FieldReader;
#[doc = "Field `OUT_STATE_CH1` reader - reserved"]
pub type OUT_STATE_CH1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
    #[inline(always)]
    pub fn outlink_dscr_addr_ch1(&self) -> OUTLINK_DSCR_ADDR_CH1_R {
        OUTLINK_DSCR_ADDR_CH1_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn out_dscr_state_ch1(&self) -> OUT_DSCR_STATE_CH1_R {
        OUT_DSCR_STATE_CH1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn out_state_ch1(&self) -> OUT_STATE_CH1_R {
        OUT_STATE_CH1_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_STATE_CH1")
            .field("outlink_dscr_addr_ch1", &self.outlink_dscr_addr_ch1())
            .field("out_dscr_state_ch1", &self.out_dscr_state_ch1())
            .field("out_state_ch1", &self.out_state_ch1())
            .finish()
    }
}
#[doc = "Transmit status of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_STATE_CH1_SPEC;
impl crate::RegisterSpec for OUT_STATE_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_state_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_STATE_CH1_SPEC {}
#[doc = "`reset()` method sets OUT_STATE_CH1 to value 0"]
impl crate::Resettable for OUT_STATE_CH1_SPEC {}
