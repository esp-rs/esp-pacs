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
            .field("invact_lpcmd_time", &self.invact_lpcmd_time())
            .field("outvact_lpcmd_time", &self.outvact_lpcmd_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn invact_lpcmd_time(&mut self) -> INVACT_LPCMD_TIME_W<'_, DPI_LP_CMD_TIM_SPEC> {
        INVACT_LPCMD_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn outvact_lpcmd_time(&mut self) -> OUTVACT_LPCMD_TIME_W<'_, DPI_LP_CMD_TIM_SPEC> {
        OUTVACT_LPCMD_TIME_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_lp_cmd_tim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_lp_cmd_tim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_LP_CMD_TIM_SPEC;
impl crate::RegisterSpec for DPI_LP_CMD_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lp_cmd_tim::R`](R) reader structure"]
impl crate::Readable for DPI_LP_CMD_TIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_lp_cmd_tim::W`](W) writer structure"]
impl crate::Writable for DPI_LP_CMD_TIM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_LP_CMD_TIM to value 0"]
impl crate::Resettable for DPI_LP_CMD_TIM_SPEC {}
