#[doc = "Register `CHNL0_TRACE0` reader"]
pub type R = crate::R<CHNL0_TRACE0_SPEC>;
#[doc = "Field `CHNL0_FRAC_FSM_CS` reader - Represents fractional re-sampler status in channel1."]
pub type CHNL0_FRAC_FSM_CS_R = crate::FieldReader;
#[doc = "Field `CHNL0_RS2_STG1_FSM_CS` reader - Represents re-sampler 2 stag1 status in channel1."]
pub type CHNL0_RS2_STG1_FSM_CS_R = crate::FieldReader;
#[doc = "Field `CHNL0_RS2_STG0_FSM_CS` reader - Represents re-sampler 2 stag0 status in channel1."]
pub type CHNL0_RS2_STG0_FSM_CS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents fractional re-sampler status in channel1."]
    #[inline(always)]
    pub fn chnl0_frac_fsm_cs(&self) -> CHNL0_FRAC_FSM_CS_R {
        CHNL0_FRAC_FSM_CS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents re-sampler 2 stag1 status in channel1."]
    #[inline(always)]
    pub fn chnl0_rs2_stg1_fsm_cs(&self) -> CHNL0_RS2_STG1_FSM_CS_R {
        CHNL0_RS2_STG1_FSM_CS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents re-sampler 2 stag0 status in channel1."]
    #[inline(always)]
    pub fn chnl0_rs2_stg0_fsm_cs(&self) -> CHNL0_RS2_STG0_FSM_CS_R {
        CHNL0_RS2_STG0_FSM_CS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_TRACE0")
            .field("chnl0_frac_fsm_cs", &self.chnl0_frac_fsm_cs())
            .field("chnl0_rs2_stg1_fsm_cs", &self.chnl0_rs2_stg1_fsm_cs())
            .field("chnl0_rs2_stg0_fsm_cs", &self.chnl0_rs2_stg0_fsm_cs())
            .finish()
    }
}
#[doc = "Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_trace0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_TRACE0_SPEC;
impl crate::RegisterSpec for CHNL0_TRACE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_trace0::R`](R) reader structure"]
impl crate::Readable for CHNL0_TRACE0_SPEC {}
#[doc = "`reset()` method sets CHNL0_TRACE0 to value 0"]
impl crate::Resettable for CHNL0_TRACE0_SPEC {}
