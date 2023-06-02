#[doc = "Register `OPTION1` reader"]
pub struct R(crate::R<OPTION1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTION1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTION1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTION1` writer"]
pub struct W(crate::W<OPTION1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTION1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OPTION1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTION1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - force chip entry download mode"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - force chip entry download mode"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, OPTION1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - force chip entry download mode"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTION1")
            .field(
                "force_download_boot",
                &format_args!("{}", self.force_download_boot().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OPTION1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - force chip entry download mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<0> {
        FORCE_DOWNLOAD_BOOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [option1](index.html) module"]
pub struct OPTION1_SPEC;
impl crate::RegisterSpec for OPTION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [option1::R](R) reader structure"]
impl crate::Readable for OPTION1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [option1::W](W) writer structure"]
impl crate::Writable for OPTION1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTION1 to value 0"]
impl crate::Resettable for OPTION1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
