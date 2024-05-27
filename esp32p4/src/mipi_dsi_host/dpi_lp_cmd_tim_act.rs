#[doc = "Register `DPI_LP_CMD_TIM_ACT` reader"]
pub type R = crate::R<DPI_LP_CMD_TIM_ACT_SPEC>;
#[doc = "Field `INVACT_LPCMD_TIME_ACT` reader - NA"]
pub type INVACT_LPCMD_TIME_ACT_R = crate::FieldReader;
#[doc = "Field `OUTVACT_LPCMD_TIME_ACT` reader - NA"]
pub type OUTVACT_LPCMD_TIME_ACT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn invact_lpcmd_time_act(&self) -> INVACT_LPCMD_TIME_ACT_R {
        INVACT_LPCMD_TIME_ACT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn outvact_lpcmd_time_act(&self) -> OUTVACT_LPCMD_TIME_ACT_R {
        OUTVACT_LPCMD_TIME_ACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_LP_CMD_TIM_ACT")
            .field("invact_lpcmd_time_act", &self.invact_lpcmd_time_act())
            .field("outvact_lpcmd_time_act", &self.outvact_lpcmd_time_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_LP_CMD_TIM_ACT_SPEC;
impl crate::RegisterSpec for DPI_LP_CMD_TIM_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lp_cmd_tim_act::R`](R) reader structure"]
impl crate::Readable for DPI_LP_CMD_TIM_ACT_SPEC {}
#[doc = "`reset()` method sets DPI_LP_CMD_TIM_ACT to value 0"]
impl crate::Resettable for DPI_LP_CMD_TIM_ACT_SPEC {
    const RESET_VALUE: u32 = 0;
}
