#[doc = "Register `PKT_LEN2` reader"]
pub struct R(crate::R<PKT_LEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKT_LEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKT_LEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKT_LEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOSTSLCHOST_SLC0_LEN2` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOSTSLCHOST_SLC0_LEN2_CHECK` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN2_CHECK_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:19 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len2(&self) -> HOSTSLCHOST_SLC0_LEN2_R {
        HOSTSLCHOST_SLC0_LEN2_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len2_check(&self) -> HOSTSLCHOST_SLC0_LEN2_CHECK_R {
        HOSTSLCHOST_SLC0_LEN2_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkt_len2](index.html) module"]
pub struct PKT_LEN2_SPEC;
impl crate::RegisterSpec for PKT_LEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkt_len2::R](R) reader structure"]
impl crate::Readable for PKT_LEN2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKT_LEN2 to value 0"]
impl crate::Resettable for PKT_LEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}