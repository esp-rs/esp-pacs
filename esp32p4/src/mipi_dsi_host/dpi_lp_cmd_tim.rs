#[doc = "Register `DPI_LP_CMD_TIM` reader"]
pub type R = crate::R<DPI_LP_CMD_TIM_SPEC>;
#[doc = "Register `DPI_LP_CMD_TIM` writer"]
pub type W = crate::W<DPI_LP_CMD_TIM_SPEC>;
#[doc = "Field `INVACT_LPCMD_TIME` reader - NA"]
pub type INVACT_LPCMD_TIME_R = crate::FieldReader;
#[doc = "Field `INVACT_LPCMD_TIME` writer - NA"]
pub type INVACT_LPCMD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTVACT_LPCMD_TIME` reader - NA"]
pub type OUTVACT_LPCMD_TIME_R = crate::FieldReader;
#[doc = "Field `OUTVACT_LPCMD_TIME` writer - NA"]
pub type OUTVACT_LPCMD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn invact_lpcmd_time(&self) -> INVACT_LPCMD_TIME_R {
        INVACT_LPCMD_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn outvact_lpcmd_time(&self) -> OUTVACT_LPCMD_TIME_R {
        OUTVACT_LPCMD_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_LP_CMD_TIM")
            .field(
                "invact_lpcmd_time",
                &format_args!("{}", self.invact_lpcmd_time().bits()),
            )
            .field(
                "outvact_lpcmd_time",
                &format_args!("{}", self.outvact_lpcmd_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPI_LP_CMD_TIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn invact_lpcmd_time(&mut self) -> INVACT_LPCMD_TIME_W<DPI_LP_CMD_TIM_SPEC> {
        INVACT_LPCMD_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn outvact_lpcmd_time(&mut self) -> OUTVACT_LPCMD_TIME_W<DPI_LP_CMD_TIM_SPEC> {
        OUTVACT_LPCMD_TIME_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lp_cmd_tim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_LP_CMD_TIM_SPEC;
impl crate::RegisterSpec for DPI_LP_CMD_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lp_cmd_tim::R`](R) reader structure"]
impl crate::Readable for DPI_LP_CMD_TIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_lp_cmd_tim::W`](W) writer structure"]
impl crate::Writable for DPI_LP_CMD_TIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_LP_CMD_TIM to value 0"]
impl crate::Resettable for DPI_LP_CMD_TIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
