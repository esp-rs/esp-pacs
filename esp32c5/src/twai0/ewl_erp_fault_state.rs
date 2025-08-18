#[doc = "Register `EWL_ERP_FAULT_STATE` reader"]
pub type R = crate::R<EWL_ERP_FAULT_STATE_SPEC>;
#[doc = "Register `EWL_ERP_FAULT_STATE` writer"]
pub type W = crate::W<EWL_ERP_FAULT_STATE_SPEC>;
#[doc = "Field `EW_LIMIT` reader - Error warning limit. If error warning limit is reached interrupt can be generated. Error warning limit indicates heavily disturbed bus."]
pub type EW_LIMIT_R = crate::FieldReader;
#[doc = "Field `EW_LIMIT` writer - Error warning limit. If error warning limit is reached interrupt can be generated. Error warning limit indicates heavily disturbed bus."]
pub type EW_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERP_LIMIT` reader - Error Passive Limit. When one of error counters (REC/TEC) exceeds this value, Fault confinement state changes to error-passive."]
pub type ERP_LIMIT_R = crate::FieldReader;
#[doc = "Field `ERP_LIMIT` writer - Error Passive Limit. When one of error counters (REC/TEC) exceeds this value, Fault confinement state changes to error-passive."]
pub type ERP_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERA` reader - Represents the fault state of error active."]
pub type ERA_R = crate::BitReader;
#[doc = "Field `ERP` reader - Represents the fault state of error passive."]
pub type ERP_R = crate::BitReader;
#[doc = "Field `BOF` reader - Represents the fault state of bus off."]
pub type BOF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Error warning limit. If error warning limit is reached interrupt can be generated. Error warning limit indicates heavily disturbed bus."]
    #[inline(always)]
    pub fn ew_limit(&self) -> EW_LIMIT_R {
        EW_LIMIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Error Passive Limit. When one of error counters (REC/TEC) exceeds this value, Fault confinement state changes to error-passive."]
    #[inline(always)]
    pub fn erp_limit(&self) -> ERP_LIMIT_R {
        ERP_LIMIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Represents the fault state of error active."]
    #[inline(always)]
    pub fn era(&self) -> ERA_R {
        ERA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents the fault state of error passive."]
    #[inline(always)]
    pub fn erp(&self) -> ERP_R {
        ERP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents the fault state of bus off."]
    #[inline(always)]
    pub fn bof(&self) -> BOF_R {
        BOF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWL_ERP_FAULT_STATE")
            .field("ew_limit", &self.ew_limit())
            .field("erp_limit", &self.erp_limit())
            .field("era", &self.era())
            .field("erp", &self.erp())
            .field("bof", &self.bof())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Error warning limit. If error warning limit is reached interrupt can be generated. Error warning limit indicates heavily disturbed bus."]
    #[inline(always)]
    pub fn ew_limit(&mut self) -> EW_LIMIT_W<'_, EWL_ERP_FAULT_STATE_SPEC> {
        EW_LIMIT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Error Passive Limit. When one of error counters (REC/TEC) exceeds this value, Fault confinement state changes to error-passive."]
    #[inline(always)]
    pub fn erp_limit(&mut self) -> ERP_LIMIT_W<'_, EWL_ERP_FAULT_STATE_SPEC> {
        ERP_LIMIT_W::new(self, 8)
    }
}
#[doc = "TWAI FD error threshold and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewl_erp_fault_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewl_erp_fault_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EWL_ERP_FAULT_STATE_SPEC;
impl crate::RegisterSpec for EWL_ERP_FAULT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewl_erp_fault_state::R`](R) reader structure"]
impl crate::Readable for EWL_ERP_FAULT_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ewl_erp_fault_state::W`](W) writer structure"]
impl crate::Writable for EWL_ERP_FAULT_STATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWL_ERP_FAULT_STATE to value 0x0004_8060"]
impl crate::Resettable for EWL_ERP_FAULT_STATE_SPEC {
    const RESET_VALUE: u32 = 0x0004_8060;
}
