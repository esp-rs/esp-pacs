#[doc = "Register `SYS_STATUS` reader"]
pub type R = crate::R<SYS_STATUS_SPEC>;
#[doc = "Field `FRAME_NUM` reader - Represents current frame number."]
pub type FRAME_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DUAL_STREAM_SEL` reader - Represents which register group is used for cur frame.\\\\0: Register group A is used\\\\1: Register group B is used."]
pub type DUAL_STREAM_SEL_R = crate::BitReader;
#[doc = "Field `INTRA_FLAG` reader - Represents the type of current encoding frame.\\\\0: P frame\\\\1: I frame."]
pub type INTRA_FLAG_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Represents current frame number."]
    #[inline(always)]
    pub fn frame_num(&self) -> FRAME_NUM_R {
        FRAME_NUM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Represents which register group is used for cur frame.\\\\0: Register group A is used\\\\1: Register group B is used."]
    #[inline(always)]
    pub fn dual_stream_sel(&self) -> DUAL_STREAM_SEL_R {
        DUAL_STREAM_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents the type of current encoding frame.\\\\0: P frame\\\\1: I frame."]
    #[inline(always)]
    pub fn intra_flag(&self) -> INTRA_FLAG_R {
        INTRA_FLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_STATUS")
            .field("frame_num", &self.frame_num().bits())
            .field("dual_stream_sel", &self.dual_stream_sel().bit())
            .field("intra_flag", &self.intra_flag().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYS_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "System status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_STATUS_SPEC;
impl crate::RegisterSpec for SYS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_status::R`](R) reader structure"]
impl crate::Readable for SYS_STATUS_SPEC {}
#[doc = "`reset()` method sets SYS_STATUS to value 0"]
impl crate::Resettable for SYS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
