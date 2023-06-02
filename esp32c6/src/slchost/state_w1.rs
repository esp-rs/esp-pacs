#[doc = "Register `STATE_W1` reader"]
pub struct R(crate::R<STATE_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLCHOST_STATE4` reader - *******Description***********"]
pub type SLCHOST_STATE4_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE5` reader - *******Description***********"]
pub type SLCHOST_STATE5_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE6` reader - *******Description***********"]
pub type SLCHOST_STATE6_R = crate::FieldReader;
#[doc = "Field `SLCHOST_STATE7` reader - *******Description***********"]
pub type SLCHOST_STATE7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state4(&self) -> SLCHOST_STATE4_R {
        SLCHOST_STATE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state5(&self) -> SLCHOST_STATE5_R {
        SLCHOST_STATE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state6(&self) -> SLCHOST_STATE6_R {
        SLCHOST_STATE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_state7(&self) -> SLCHOST_STATE7_R {
        SLCHOST_STATE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE_W1")
            .field(
                "slchost_state4",
                &format_args!("{}", self.slchost_state4().bits()),
            )
            .field(
                "slchost_state5",
                &format_args!("{}", self.slchost_state5().bits()),
            )
            .field(
                "slchost_state6",
                &format_args!("{}", self.slchost_state6().bits()),
            )
            .field(
                "slchost_state7",
                &format_args!("{}", self.slchost_state7().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state_w1](index.html) module"]
pub struct STATE_W1_SPEC;
impl crate::RegisterSpec for STATE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state_w1::R](R) reader structure"]
impl crate::Readable for STATE_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE_W1 to value 0"]
impl crate::Resettable for STATE_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
