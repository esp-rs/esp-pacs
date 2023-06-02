#[doc = "Register `APP_CPU_RECORD_CTRL` reader"]
pub struct R(crate::R<APP_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CPU_RECORD_CTRL` writer"]
pub struct W(crate::W<APP_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CPU_RECORD_CTRL_SPEC>;
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
impl From<crate::W<APP_CPU_RECORD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CPU_RECORD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CPU_RECORD_ENABLE` reader - "]
pub type APP_CPU_RECORD_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_RECORD_ENABLE` writer - "]
pub type APP_CPU_RECORD_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, APP_CPU_RECORD_CTRL_SPEC, O>;
#[doc = "Field `APP_CPU_RECORD_DISABLE` reader - "]
pub type APP_CPU_RECORD_DISABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_RECORD_DISABLE` writer - "]
pub type APP_CPU_RECORD_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, APP_CPU_RECORD_CTRL_SPEC, O>;
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` reader - "]
pub type APP_CPU_PDEBUG_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` writer - "]
pub type APP_CPU_PDEBUG_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, APP_CPU_RECORD_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&self) -> APP_CPU_RECORD_ENABLE_R {
        APP_CPU_RECORD_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&self) -> APP_CPU_RECORD_DISABLE_R {
        APP_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&self) -> APP_CPU_PDEBUG_ENABLE_R {
        APP_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_CTRL")
            .field(
                "app_cpu_record_enable",
                &format_args!("{}", self.app_cpu_record_enable().bit()),
            )
            .field(
                "app_cpu_record_disable",
                &format_args!("{}", self.app_cpu_record_disable().bit()),
            )
            .field(
                "app_cpu_pdebug_enable",
                &format_args!("{}", self.app_cpu_pdebug_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_CPU_RECORD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_record_enable(&mut self) -> APP_CPU_RECORD_ENABLE_W<0> {
        APP_CPU_RECORD_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_record_disable(&mut self) -> APP_CPU_RECORD_DISABLE_W<4> {
        APP_CPU_RECORD_DISABLE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_pdebug_enable(&mut self) -> APP_CPU_PDEBUG_ENABLE_W<8> {
        APP_CPU_PDEBUG_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_ctrl](index.html) module"]
pub struct APP_CPU_RECORD_CTRL_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_ctrl::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_ctrl::W](W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_CTRL to value 0x0100"]
impl crate::Resettable for APP_CPU_RECORD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
