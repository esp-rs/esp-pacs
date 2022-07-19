#[doc = "Register `CPU_INT_PRI_20` reader"]
pub struct R(crate::R<CPU_INT_PRI_20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INT_PRI_20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INT_PRI_20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INT_PRI_20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INT_PRI_20` writer"]
pub struct W(crate::W<CPU_INT_PRI_20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INT_PRI_20_SPEC>;
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
impl From<crate::W<CPU_INT_PRI_20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INT_PRI_20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_PRI_20_MAP` reader - Need add description"]
pub type CPU_PRI_20_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_PRI_20_MAP` writer - Need add description"]
pub type CPU_PRI_20_MAP_W<'a> = crate::FieldWriter<'a, u32, CPU_INT_PRI_20_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn cpu_pri_20_map(&self) -> CPU_PRI_20_MAP_R {
        CPU_PRI_20_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn cpu_pri_20_map(&mut self) -> CPU_PRI_20_MAP_W {
        CPU_PRI_20_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_int_pri_20](index.html) module"]
pub struct CPU_INT_PRI_20_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_int_pri_20::R](R) reader structure"]
impl crate::Readable for CPU_INT_PRI_20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_int_pri_20::W](W) writer structure"]
impl crate::Writable for CPU_INT_PRI_20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_INT_PRI_20 to value 0"]
impl crate::Resettable for CPU_INT_PRI_20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
