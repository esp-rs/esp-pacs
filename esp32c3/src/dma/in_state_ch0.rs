#[doc = "Register `IN_STATE_CH0` reader"]
pub struct R(crate::R<IN_STATE_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_STATE_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_STATE_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_STATE_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_ADDR` reader - This register stores the current inlink descriptor's address."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IN_DSCR_STATE` reader - reserved"]
pub type IN_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `IN_STATE` reader - reserved"]
pub type IN_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATE_CH0")
            .field(
                "inlink_dscr_addr",
                &format_args!("{}", self.inlink_dscr_addr().bits()),
            )
            .field(
                "in_dscr_state",
                &format_args!("{}", self.in_dscr_state().bits()),
            )
            .field("in_state", &format_args!("{}", self.in_state().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_STATE_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_IN_STATE_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_state_ch0](index.html) module"]
pub struct IN_STATE_CH0_SPEC;
impl crate::RegisterSpec for IN_STATE_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_state_ch0::R](R) reader structure"]
impl crate::Readable for IN_STATE_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_STATE_CH0 to value 0"]
impl crate::Resettable for IN_STATE_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
