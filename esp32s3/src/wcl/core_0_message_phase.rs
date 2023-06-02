#[doc = "Register `Core_0_MESSAGE_PHASE` reader"]
pub struct R(crate::R<CORE_0_MESSAGE_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_MESSAGE_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_MESSAGE_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_MESSAGE_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_MESSAGE_MATCH` reader - This bit indicates whether the check is successful"]
pub type CORE_0_MESSAGE_MATCH_R = crate::BitReader;
#[doc = "Field `CORE_0_MESSAGE_EXPECT` reader - This field indicates the data to be written next time"]
pub type CORE_0_MESSAGE_EXPECT_R = crate::FieldReader;
#[doc = "Field `CORE_0_MESSAGE_DATAPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
pub type CORE_0_MESSAGE_DATAPHASE_R = crate::BitReader;
#[doc = "Field `CORE_0_MESSAGE_ADDRESSPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
pub type CORE_0_MESSAGE_ADDRESSPHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates whether the check is successful"]
    #[inline(always)]
    pub fn core_0_message_match(&self) -> CORE_0_MESSAGE_MATCH_R {
        CORE_0_MESSAGE_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This field indicates the data to be written next time"]
    #[inline(always)]
    pub fn core_0_message_expect(&self) -> CORE_0_MESSAGE_EXPECT_R {
        CORE_0_MESSAGE_EXPECT_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
    #[inline(always)]
    pub fn core_0_message_dataphase(&self) -> CORE_0_MESSAGE_DATAPHASE_R {
        CORE_0_MESSAGE_DATAPHASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
    #[inline(always)]
    pub fn core_0_message_addressphase(&self) -> CORE_0_MESSAGE_ADDRESSPHASE_R {
        CORE_0_MESSAGE_ADDRESSPHASE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_MESSAGE_PHASE")
            .field(
                "core_0_message_match",
                &format_args!("{}", self.core_0_message_match().bit()),
            )
            .field(
                "core_0_message_expect",
                &format_args!("{}", self.core_0_message_expect().bits()),
            )
            .field(
                "core_0_message_dataphase",
                &format_args!("{}", self.core_0_message_dataphase().bit()),
            )
            .field(
                "core_0_message_addressphase",
                &format_args!("{}", self.core_0_message_addressphase().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_MESSAGE_PHASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Clear writer_buffer status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_message_phase](index.html) module"]
pub struct CORE_0_MESSAGE_PHASE_SPEC;
impl crate::RegisterSpec for CORE_0_MESSAGE_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_message_phase::R](R) reader structure"]
impl crate::Readable for CORE_0_MESSAGE_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets Core_0_MESSAGE_PHASE to value 0"]
impl crate::Resettable for CORE_0_MESSAGE_PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
