#[doc = "Register `PRO_CPU_INTR_FROM_CPU_3_MAP` reader"]
pub struct R(crate::R<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_INTR_FROM_CPU_3_MAP` writer"]
pub struct W(crate::W<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
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
impl From<crate::W<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CPU_INTR_FROM_CPU_3_MAP` reader - "]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_CPU_INTR_FROM_CPU_3_MAP` writer - "]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_cpu_intr_from_cpu_3_map(&self) -> PRO_CPU_INTR_FROM_CPU_3_MAP_R {
        PRO_CPU_INTR_FROM_CPU_3_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_INTR_FROM_CPU_3_MAP")
            .field(
                "pro_cpu_intr_from_cpu_3_map",
                &format_args!("{}", self.pro_cpu_intr_from_cpu_3_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_intr_from_cpu_3_map(&mut self) -> PRO_CPU_INTR_FROM_CPU_3_MAP_W<0> {
        PRO_CPU_INTR_FROM_CPU_3_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_intr_from_cpu_3_map](index.html) module"]
pub struct PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC;
impl crate::RegisterSpec for PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_intr_from_cpu_3_map::R](R) reader structure"]
impl crate::Readable for PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_intr_from_cpu_3_map::W](W) writer structure"]
impl crate::Writable for PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CPU_INTR_FROM_CPU_3_MAP to value 0x10"]
impl crate::Resettable for PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
