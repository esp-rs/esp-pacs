#[doc = "Register `STATE_W0` reader"]
pub struct R(crate::R<STATE_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLCHOST_STATE0` reader - *******Description***********"]
pub type SLCHOST_STATE0_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE1` reader - *******Description***********"]
pub type SLCHOST_STATE1_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE2` reader - *******Description***********"]
pub type SLCHOST_STATE2_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE3` reader - *******Description***********"]
pub type SLCHOST_STATE3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state0(&self) -> SLCHOST_STATE0_R {
        SLCHOST_STATE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state1(&self) -> SLCHOST_STATE1_R {
        SLCHOST_STATE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state2(&self) -> SLCHOST_STATE2_R {
        SLCHOST_STATE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state3(&self) -> SLCHOST_STATE3_R {
        SLCHOST_STATE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE_W0")
            .field(
                "slchost_state0",
                &format_args!("{}", self.slchost_state0().bits()),
            )
            .field(
                "slchost_state1",
                &format_args!("{}", self.slchost_state1().bits()),
            )
            .field(
                "slchost_state2",
                &format_args!("{}", self.slchost_state2().bits()),
            )
            .field(
                "slchost_state3",
                &format_args!("{}", self.slchost_state3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state_w0](index.html) module"]
pub struct STATE_W0_SPEC;
impl crate::RegisterSpec for STATE_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state_w0::R](R) reader structure"]
impl crate::Readable for STATE_W0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE_W0 to value 0"]
impl crate::Resettable for STATE_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
