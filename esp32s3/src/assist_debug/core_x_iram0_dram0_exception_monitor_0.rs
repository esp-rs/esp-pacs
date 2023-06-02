#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` reader"]
pub struct R(crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` writer"]
pub struct W(crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` reader - busy monitor window cycle"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` writer - busy monitor window cycle"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC, 20, O, u32, u32>;
impl R {
    #[doc = "Bits 0:19 - busy monitor window cycle"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_0(&self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_x_iram0_dram0_limit_cycle_0",
                &format_args!("{}", self.core_x_iram0_dram0_limit_cycle_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - busy monitor window cycle"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_limit_cycle_0(&mut self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<0> {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bus busy configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_dram0_exception_monitor_0](index.html) module"]
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_dram0_exception_monitor_0::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_dram0_exception_monitor_0::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 to value 0x000f_ffff"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}
