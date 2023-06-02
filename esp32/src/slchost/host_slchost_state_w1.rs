#[doc = "Register `HOST_SLCHOST_STATE_W1` reader"]
pub struct R(crate::R<HOST_SLCHOST_STATE_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_STATE_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_STATE_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_STATE_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLCHOST_STATE4` reader - "]
pub type HOST_SLCHOST_STATE4_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_STATE5` reader - "]
pub type HOST_SLCHOST_STATE5_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_STATE6` reader - "]
pub type HOST_SLCHOST_STATE6_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_STATE7` reader - "]
pub type HOST_SLCHOST_STATE7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_state4(&self) -> HOST_SLCHOST_STATE4_R {
        HOST_SLCHOST_STATE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_state5(&self) -> HOST_SLCHOST_STATE5_R {
        HOST_SLCHOST_STATE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_state6(&self) -> HOST_SLCHOST_STATE6_R {
        HOST_SLCHOST_STATE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_state7(&self) -> HOST_SLCHOST_STATE7_R {
        HOST_SLCHOST_STATE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_STATE_W1")
            .field(
                "host_slchost_state4",
                &format_args!("{}", self.host_slchost_state4().bits()),
            )
            .field(
                "host_slchost_state5",
                &format_args!("{}", self.host_slchost_state5().bits()),
            )
            .field(
                "host_slchost_state6",
                &format_args!("{}", self.host_slchost_state6().bits()),
            )
            .field(
                "host_slchost_state7",
                &format_args!("{}", self.host_slchost_state7().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_STATE_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_state_w1](index.html) module"]
pub struct HOST_SLCHOST_STATE_W1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_STATE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_state_w1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_STATE_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_STATE_W1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_STATE_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
