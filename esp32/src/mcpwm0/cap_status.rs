#[doc = "Register `CAP_STATUS` reader"]
pub type R = crate::R<CAP_STATUS_SPEC>;
#[doc = "Field `CAP0_EDGE` reader - "]
pub type CAP0_EDGE_R = crate::BitReader;
#[doc = "Field `CAP1_EDGE` reader - "]
pub type CAP1_EDGE_R = crate::BitReader;
#[doc = "Field `CAP2_EDGE` reader - "]
pub type CAP2_EDGE_R = crate::BitReader;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_STATUS")
            .field("cap0_edge", &self.cap0_edge())
            .field("cap1_edge", &self.cap1_edge())
            .field("cap2_edge", &self.cap2_edge())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_STATUS_SPEC;
impl crate::RegisterSpec for CAP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_status::R`](R) reader structure"]
impl crate::Readable for CAP_STATUS_SPEC {}
#[doc = "`reset()` method sets CAP_STATUS to value 0"]
impl crate::Resettable for CAP_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
