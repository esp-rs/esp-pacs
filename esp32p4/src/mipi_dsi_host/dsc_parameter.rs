#[doc = "Register `DSC_PARAMETER` reader"]
pub type R = crate::R<DSC_PARAMETER_SPEC>;
#[doc = "Register `DSC_PARAMETER` writer"]
pub type W = crate::W<DSC_PARAMETER_SPEC>;
#[doc = "Field `COMPRESSION_MODE` reader - NA"]
pub type COMPRESSION_MODE_R = crate::BitReader;
#[doc = "Field `COMPRESSION_MODE` writer - NA"]
pub type COMPRESSION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRESS_ALGO` reader - NA"]
pub type COMPRESS_ALGO_R = crate::FieldReader;
#[doc = "Field `COMPRESS_ALGO` writer - NA"]
pub type COMPRESS_ALGO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PPS_SEL` reader - NA"]
pub type PPS_SEL_R = crate::FieldReader;
#[doc = "Field `PPS_SEL` writer - NA"]
pub type PPS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn compression_mode(&self) -> COMPRESSION_MODE_R {
        COMPRESSION_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    pub fn compress_algo(&self) -> COMPRESS_ALGO_R {
        COMPRESS_ALGO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn pps_sel(&self) -> PPS_SEL_R {
        PPS_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSC_PARAMETER")
            .field(
                "compression_mode",
                &format_args!("{}", self.compression_mode().bit()),
            )
            .field(
                "compress_algo",
                &format_args!("{}", self.compress_algo().bits()),
            )
            .field("pps_sel", &format_args!("{}", self.pps_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DSC_PARAMETER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn compression_mode(&mut self) -> COMPRESSION_MODE_W<DSC_PARAMETER_SPEC> {
        COMPRESSION_MODE_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn compress_algo(&mut self) -> COMPRESS_ALGO_W<DSC_PARAMETER_SPEC> {
        COMPRESS_ALGO_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn pps_sel(&mut self) -> PPS_SEL_W<DSC_PARAMETER_SPEC> {
        PPS_SEL_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsc_parameter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsc_parameter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSC_PARAMETER_SPEC;
impl crate::RegisterSpec for DSC_PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsc_parameter::R`](R) reader structure"]
impl crate::Readable for DSC_PARAMETER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsc_parameter::W`](W) writer structure"]
impl crate::Writable for DSC_PARAMETER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSC_PARAMETER to value 0"]
impl crate::Resettable for DSC_PARAMETER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
