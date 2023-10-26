#[doc = "Register `SLC_APBWIN_WDATA` reader"]
pub type R = crate::R<SLC_APBWIN_WDATA_SPEC>;
#[doc = "Register `SLC_APBWIN_WDATA` writer"]
pub type W = crate::W<SLC_APBWIN_WDATA_SPEC>;
#[doc = "Field `SLC_APBWIN_WDATA` reader - *******Description***********"]
pub type SLC_APBWIN_WDATA_R = crate::FieldReader<u32>;
#[doc = "Field `SLC_APBWIN_WDATA` writer - *******Description***********"]
pub type SLC_APBWIN_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_wdata(&self) -> SLC_APBWIN_WDATA_R {
        SLC_APBWIN_WDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_APBWIN_WDATA")
            .field(
                "slc_apbwin_wdata",
                &format_args!("{}", self.slc_apbwin_wdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_APBWIN_WDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_wdata(&mut self) -> SLC_APBWIN_WDATA_W<SLC_APBWIN_WDATA_SPEC, 0> {
        SLC_APBWIN_WDATA_W::new(self)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_apbwin_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_apbwin_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_APBWIN_WDATA_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_apbwin_wdata::R`](R) reader structure"]
impl crate::Readable for SLC_APBWIN_WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_apbwin_wdata::W`](W) writer structure"]
impl crate::Writable for SLC_APBWIN_WDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_APBWIN_WDATA to value 0"]
impl crate::Resettable for SLC_APBWIN_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
