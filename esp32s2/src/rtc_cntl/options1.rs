#[doc = "Register `OPTIONS1` reader"]
pub struct R(crate::R<OPTIONS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS1` writer"]
pub struct W(crate::W<OPTIONS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS1_SPEC>;
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
impl From<crate::W<OPTIONS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - Set this bit to force the chip to boot from the download mode."]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - Set this bit to force the chip to boot from the download mode."]
pub type FORCE_DOWNLOAD_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to force the chip to boot from the download mode."]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTIONS1")
            .field(
                "force_download_boot",
                &format_args!("{}", self.force_download_boot().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OPTIONS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force the chip to boot from the download mode."]
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
#[doc = "RTC option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options1](index.html) module"]
pub struct OPTIONS1_SPEC;
impl crate::RegisterSpec for OPTIONS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options1::R](R) reader structure"]
impl crate::Readable for OPTIONS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options1::W](W) writer structure"]
impl crate::Writable for OPTIONS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTIONS1 to value 0"]
impl crate::Resettable for OPTIONS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
