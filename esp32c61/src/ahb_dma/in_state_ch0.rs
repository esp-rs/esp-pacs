#[doc = "Register `IN_STATE_CH0` reader"]
pub type R = crate::R<IN_STATE_CH0_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR_CH0` reader - reserved"]
pub type INLINK_DSCR_ADDR_CH0_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE_CH0` reader - reserved"]
pub type IN_DSCR_STATE_CH0_R = crate::FieldReader;
#[doc = "Field `IN_STATE_CH0` reader - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
pub type IN_STATE_CH0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - reserved"]
    #[inline(always)]
    pub fn inlink_dscr_addr_ch0(&self) -> INLINK_DSCR_ADDR_CH0_R {
        INLINK_DSCR_ADDR_CH0_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state_ch0(&self) -> IN_DSCR_STATE_CH0_R {
        IN_DSCR_STATE_CH0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
    #[inline(always)]
    pub fn in_state_ch0(&self) -> IN_STATE_CH0_R {
        IN_STATE_CH0_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATE_CH0")
            .field("inlink_dscr_addr_ch0", &self.inlink_dscr_addr_ch0())
            .field("in_dscr_state_ch0", &self.in_dscr_state_ch0())
            .field("in_state_ch0", &self.in_state_ch0())
            .finish()
    }
}
#[doc = "Receive status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_STATE_CH0_SPEC;
impl crate::RegisterSpec for IN_STATE_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_state_ch0::R`](R) reader structure"]
impl crate::Readable for IN_STATE_CH0_SPEC {}
#[doc = "`reset()` method sets IN_STATE_CH0 to value 0"]
impl crate::Resettable for IN_STATE_CH0_SPEC {}
