#[doc = "Register `STATUS_16` reader"]
pub type R = crate::R<STATUS_16_SPEC>;
#[doc = "Field `APPROACH_PAD2_CNT` reader - need_des"]
pub type APPROACH_PAD2_CNT_R = crate::FieldReader;
#[doc = "Field `APPROACH_PAD1_CNT` reader - need_des"]
pub type APPROACH_PAD1_CNT_R = crate::FieldReader;
#[doc = "Field `APPROACH_PAD0_CNT` reader - need_des"]
pub type APPROACH_PAD0_CNT_R = crate::FieldReader;
#[doc = "Field `SLP_APPROACH_CNT` reader - need_des"]
pub type SLP_APPROACH_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn approach_pad2_cnt(&self) -> APPROACH_PAD2_CNT_R {
        APPROACH_PAD2_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn approach_pad1_cnt(&self) -> APPROACH_PAD1_CNT_R {
        APPROACH_PAD1_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn approach_pad0_cnt(&self) -> APPROACH_PAD0_CNT_R {
        APPROACH_PAD0_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn slp_approach_cnt(&self) -> SLP_APPROACH_CNT_R {
        SLP_APPROACH_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_16")
            .field("approach_pad2_cnt", &self.approach_pad2_cnt())
            .field("approach_pad1_cnt", &self.approach_pad1_cnt())
            .field("approach_pad0_cnt", &self.approach_pad0_cnt())
            .field("slp_approach_cnt", &self.slp_approach_cnt())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_16_SPEC;
impl crate::RegisterSpec for STATUS_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_16::R`](R) reader structure"]
impl crate::Readable for STATUS_16_SPEC {}
#[doc = "`reset()` method sets STATUS_16 to value 0"]
impl crate::Resettable for STATUS_16_SPEC {
    const RESET_VALUE: u32 = 0;
}
