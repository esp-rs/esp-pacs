#[doc = "Register `CHNL1_TRACE0` reader"]
pub type R = crate::R<CHNL1_TRACE0_SPEC>;
#[doc = "Field `CHNL1_FRAC_FSM_CS` reader - Represents fractional re-sampler status in channel1."]
pub type CHNL1_FRAC_FSM_CS_R = crate::FieldReader;
#[doc = "Field `CHNL1_RS2_STG1_FSM_CS` reader - Represents re-sampler 2 stag1 status in channel1."]
pub type CHNL1_RS2_STG1_FSM_CS_R = crate::FieldReader;
#[doc = "Field `CHNL1_RS2_STG0_FSM_CS` reader - Represents re-sampler 2 stag0 status in channel1."]
pub type CHNL1_RS2_STG0_FSM_CS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents fractional re-sampler status in channel1."]
    #[inline(always)]
    pub fn chnl1_frac_fsm_cs(&self) -> CHNL1_FRAC_FSM_CS_R {
        CHNL1_FRAC_FSM_CS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents re-sampler 2 stag1 status in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg1_fsm_cs(&self) -> CHNL1_RS2_STG1_FSM_CS_R {
        CHNL1_RS2_STG1_FSM_CS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents re-sampler 2 stag0 status in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg0_fsm_cs(&self) -> CHNL1_RS2_STG0_FSM_CS_R {
        CHNL1_RS2_STG0_FSM_CS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_TRACE0")
            .field("chnl1_frac_fsm_cs", &self.chnl1_frac_fsm_cs())
            .field("chnl1_rs2_stg1_fsm_cs", &self.chnl1_rs2_stg1_fsm_cs())
            .field("chnl1_rs2_stg0_fsm_cs", &self.chnl1_rs2_stg0_fsm_cs())
            .finish()
    }
}
#[doc = "Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_trace0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_TRACE0_SPEC;
impl crate::RegisterSpec for CHNL1_TRACE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_trace0::R`](R) reader structure"]
impl crate::Readable for CHNL1_TRACE0_SPEC {}
#[doc = "`reset()` method sets CHNL1_TRACE0 to value 0"]
impl crate::Resettable for CHNL1_TRACE0_SPEC {}
