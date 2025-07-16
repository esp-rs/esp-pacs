#[doc = "Register `MESSAGE_PHASE` reader"]
pub type R = crate::R<MESSAGE_PHASE_SPEC>;
#[doc = "Field `MESSAGE_MATCH` reader - This bit indicates whether the check is successful"]
pub type MESSAGE_MATCH_R = crate::BitReader;
#[doc = "Field `MESSAGE_EXPECT` reader - This field indicates the data to be written next time"]
pub type MESSAGE_EXPECT_R = crate::FieldReader;
#[doc = "Field `MESSAGE_DATAPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
pub type MESSAGE_DATAPHASE_R = crate::BitReader;
#[doc = "Field `MESSAGE_ADDRESSPHASE` reader - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
pub type MESSAGE_ADDRESSPHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates whether the check is successful"]
    #[inline(always)]
    pub fn message_match(&self) -> MESSAGE_MATCH_R {
        MESSAGE_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This field indicates the data to be written next time"]
    #[inline(always)]
    pub fn message_expect(&self) -> MESSAGE_EXPECT_R {
        MESSAGE_EXPECT_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking data"]
    #[inline(always)]
    pub fn message_dataphase(&self) -> MESSAGE_DATAPHASE_R {
        MESSAGE_DATAPHASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is 1, it means that is checking clear write_buffer operation,and is checking address."]
    #[inline(always)]
    pub fn message_addressphase(&self) -> MESSAGE_ADDRESSPHASE_R {
        MESSAGE_ADDRESSPHASE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESSAGE_PHASE")
            .field("message_match", &self.message_match())
            .field("message_expect", &self.message_expect())
            .field("message_dataphase", &self.message_dataphase())
            .field("message_addressphase", &self.message_addressphase())
            .finish()
    }
}
#[doc = "Clear writer_buffer status register\n\nYou can [`read`](crate::Reg::read) this register and get [`message_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESSAGE_PHASE_SPEC;
impl crate::RegisterSpec for MESSAGE_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`message_phase::R`](R) reader structure"]
impl crate::Readable for MESSAGE_PHASE_SPEC {}
#[doc = "`reset()` method sets MESSAGE_PHASE to value 0"]
impl crate::Resettable for MESSAGE_PHASE_SPEC {}
