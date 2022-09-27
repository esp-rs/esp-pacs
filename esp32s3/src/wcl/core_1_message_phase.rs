#[doc = "Register `Core_1_MESSAGE_PHASE` reader"]
pub struct R(crate::R<CORE_1_MESSAGE_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_MESSAGE_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_MESSAGE_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_MESSAGE_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_MESSAGE_MATCH` reader - This bit indicates whether the check is successful"]
pub type CORE_1_MESSAGE_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_MESSAGE_EXPECT` reader - This field indicates the data to be written next time"]
pub type CORE_1_MESSAGE_EXPECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_MESSAGE_DATAPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation, and is checking data"]
pub type CORE_1_MESSAGE_DATAPHASE_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_MESSAGE_ADDRESSPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation, and is checking address."]
pub type CORE_1_MESSAGE_ADDRESSPHASE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates whether the check is successful"]
    #[inline(always)]
    pub fn core_1_message_match(&self) -> CORE_1_MESSAGE_MATCH_R {
        CORE_1_MESSAGE_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This field indicates the data to be written next time"]
    #[inline(always)]
    pub fn core_1_message_expect(&self) -> CORE_1_MESSAGE_EXPECT_R {
        CORE_1_MESSAGE_EXPECT_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - If this bit is 1, it means that is checking clear write_buffer operation, and is checking data"]
    #[inline(always)]
    pub fn core_1_message_dataphase(&self) -> CORE_1_MESSAGE_DATAPHASE_R {
        CORE_1_MESSAGE_DATAPHASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is 1, it means that is checking clear write_buffer operation, and is checking address."]
    #[inline(always)]
    pub fn core_1_message_addressphase(&self) -> CORE_1_MESSAGE_ADDRESSPHASE_R {
        CORE_1_MESSAGE_ADDRESSPHASE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Clear writer_buffer status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_message_phase](index.html) module"]
pub struct CORE_1_MESSAGE_PHASE_SPEC;
impl crate::RegisterSpec for CORE_1_MESSAGE_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_message_phase::R](R) reader structure"]
impl crate::Readable for CORE_1_MESSAGE_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets Core_1_MESSAGE_PHASE to value 0"]
impl crate::Resettable for CORE_1_MESSAGE_PHASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
