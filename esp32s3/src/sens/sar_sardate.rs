#[doc = "Register `SAR_SARDATE` reader"]
pub type R = crate::R<SAR_SARDATE_SPEC>;
#[doc = "Register `SAR_SARDATE` writer"]
pub type W = crate::W<SAR_SARDATE_SPEC>;
#[doc = "Field `SAR_DATE` reader - version"]
pub type SAR_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_DATE` writer - version"]
pub type SAR_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - version"]
    #[inline(always)]
    pub fn sar_date(&self) -> SAR_DATE_R {
        SAR_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SARDATE")
            .field("sar_date", &format_args!("{}", self.sar_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_SARDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - version"]
    #[inline(always)]
    #[must_use]
    pub fn sar_date(&mut self) -> SAR_DATE_W<SAR_SARDATE_SPEC, 0> {
        SAR_DATE_W::new(self)
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
#[doc = "version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_sardate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_sardate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SARDATE_SPEC;
impl crate::RegisterSpec for SAR_SARDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_sardate::R`](R) reader structure"]
impl crate::Readable for SAR_SARDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_sardate::W`](W) writer structure"]
impl crate::Writable for SAR_SARDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_SARDATE to value 0x0210_1180"]
impl crate::Resettable for SAR_SARDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_1180;
}
