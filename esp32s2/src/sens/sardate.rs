#[doc = "Register `SARDATE` reader"]
pub type R = crate::R<SARDATE_SPEC>;
#[doc = "Register `SARDATE` writer"]
pub type W = crate::W<SARDATE_SPEC>;
#[doc = "Field `SAR_DATE` reader - Version Control Register"]
pub type SAR_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_DATE` writer - Version Control Register"]
pub type SAR_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version Control Register"]
    #[inline(always)]
    pub fn sar_date(&self) -> SAR_DATE_R {
        SAR_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SARDATE")
            .field("sar_date", &format_args!("{}", self.sar_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SARDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn sar_date(&mut self) -> SAR_DATE_W<SARDATE_SPEC> {
        SAR_DATE_W::new(self, 0)
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
#[doc = "Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sardate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sardate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARDATE_SPEC;
impl crate::RegisterSpec for SARDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sardate::R`](R) reader structure"]
impl crate::Readable for SARDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sardate::W`](W) writer structure"]
impl crate::Writable for SARDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SARDATE to value 0x0190_6140"]
impl crate::Resettable for SARDATE_SPEC {
    const RESET_VALUE: u32 = 0x0190_6140;
}
