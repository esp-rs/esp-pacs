#[doc = "Register `OUT_STATE_CH%s` reader"]
pub type R = crate::R<OUT_STATE_CH_SPEC>;
#[doc = "Field `OUTLINK_DSCR_ADDR_CH` reader - This register stores the current outlink descriptor's address."]
pub type OUTLINK_DSCR_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE_CH` reader - This register stores the current descriptor state machine state."]
pub type OUT_DSCR_STATE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_STATE_CH` reader - This register stores the current control module state machine state."]
pub type OUT_STATE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_RESET_AVAIL_CH` reader - This register indicate that if the channel reset is safety."]
pub type OUT_RESET_AVAIL_CH_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr_ch(&self) -> OUTLINK_DSCR_ADDR_CH_R {
        OUTLINK_DSCR_ADDR_CH_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - This register stores the current descriptor state machine state."]
    #[inline(always)]
    pub fn out_dscr_state_ch(&self) -> OUT_DSCR_STATE_CH_R {
        OUT_DSCR_STATE_CH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - This register stores the current control module state machine state."]
    #[inline(always)]
    pub fn out_state_ch(&self) -> OUT_STATE_CH_R {
        OUT_STATE_CH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - This register indicate that if the channel reset is safety."]
    #[inline(always)]
    pub fn out_reset_avail_ch(&self) -> OUT_RESET_AVAIL_CH_R {
        OUT_RESET_AVAIL_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_STATE_CH")
            .field("outlink_dscr_addr_ch", &self.outlink_dscr_addr_ch())
            .field("out_dscr_state_ch", &self.out_dscr_state_ch())
            .field("out_state_ch", &self.out_state_ch())
            .field("out_reset_avail_ch", &self.out_reset_avail_ch())
            .finish()
    }
}
#[doc = "Represents the working status of the tx descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_STATE_CH_SPEC;
impl crate::RegisterSpec for OUT_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_state_ch::R`](R) reader structure"]
impl crate::Readable for OUT_STATE_CH_SPEC {}
#[doc = "`reset()` method sets OUT_STATE_CH%s to value 0x0100_0000"]
impl crate::Resettable for OUT_STATE_CH_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
