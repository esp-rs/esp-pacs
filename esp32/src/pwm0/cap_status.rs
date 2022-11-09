#[doc = "Register `CAP_STATUS` reader"]
pub struct R(crate::R<CAP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP0_EDGE` reader - "]
pub type CAP0_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CAP1_EDGE` reader - "]
pub type CAP1_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CAP2_EDGE` reader - "]
pub type CAP2_EDGE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cap0_edge(&self) -> CAP0_EDGE_R {
        CAP0_EDGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cap1_edge(&self) -> CAP1_EDGE_R {
        CAP1_EDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cap2_edge(&self) -> CAP2_EDGE_R {
        CAP2_EDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_status](index.html) module"]
pub struct CAP_STATUS_SPEC;
impl crate::RegisterSpec for CAP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_status::R](R) reader structure"]
impl crate::Readable for CAP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP_STATUS to value 0"]
impl crate::Resettable for CAP_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
