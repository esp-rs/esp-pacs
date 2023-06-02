#[doc = "Register `CPU` reader"]
pub struct R(crate::R<CPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU` writer"]
pub struct W(crate::W<CPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_SPEC>;
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
impl From<crate::W<CPU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCORE_DBGM_UNAVALIABLE` reader - need_des"]
pub type LPCORE_DBGM_UNAVALIABLE_R = crate::BitReader;
#[doc = "Field `LPCORE_DBGM_UNAVALIABLE` writer - need_des"]
pub type LPCORE_DBGM_UNAVALIABLE_W<'a, const O: u8> = crate::BitWriter<'a, CPU_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lpcore_dbgm_unavaliable(&self) -> LPCORE_DBGM_UNAVALIABLE_R {
        LPCORE_DBGM_UNAVALIABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU")
            .field(
                "lpcore_dbgm_unavaliable",
                &format_args!("{}", self.lpcore_dbgm_unavaliable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpcore_dbgm_unavaliable(&mut self) -> LPCORE_DBGM_UNAVALIABLE_W<31> {
        LPCORE_DBGM_UNAVALIABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu](index.html) module"]
pub struct CPU_SPEC;
impl crate::RegisterSpec for CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu::R](R) reader structure"]
impl crate::Readable for CPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu::W](W) writer structure"]
impl crate::Writable for CPU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU to value 0x8000_0000"]
impl crate::Resettable for CPU_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
