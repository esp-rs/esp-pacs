#[doc = "Register `PKT_LEN1` reader"]
pub struct R(crate::R<PKT_LEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKT_LEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKT_LEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKT_LEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOSTSLCHOST_SLC0_LEN1` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN1_R = crate::FieldReader<u32>;
#[doc = "Field `HOSTSLCHOST_SLC0_LEN1_CHECK` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN1_CHECK_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len1(&self) -> HOSTSLCHOST_SLC0_LEN1_R {
        HOSTSLCHOST_SLC0_LEN1_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len1_check(&self) -> HOSTSLCHOST_SLC0_LEN1_CHECK_R {
        HOSTSLCHOST_SLC0_LEN1_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_LEN1")
            .field(
                "hostslchost_slc0_len1",
                &format_args!("{}", self.hostslchost_slc0_len1().bits()),
            )
            .field(
                "hostslchost_slc0_len1_check",
                &format_args!("{}", self.hostslchost_slc0_len1_check().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PKT_LEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkt_len1](index.html) module"]
pub struct PKT_LEN1_SPEC;
impl crate::RegisterSpec for PKT_LEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkt_len1::R](R) reader structure"]
impl crate::Readable for PKT_LEN1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKT_LEN1 to value 0"]
impl crate::Resettable for PKT_LEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
