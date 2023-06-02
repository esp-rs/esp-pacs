#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_SIZE` reader"]
pub struct R(crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_SIZE` writer"]
pub struct W(crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
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
impl From<crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_SIZE` reader - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_SIZE` writer - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC, 24, O, u32, u32>;
impl R {
    #[doc = "Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_size(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R {
        PRO_ICACHE_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_AUTOLOAD_SECTION1_SIZE")
            .field(
                "pro_icache_autoload_sct1_size",
                &format_args!("{}", self.pro_icache_autoload_sct1_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_sct1_size(&mut self) -> PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W<0> {
        PRO_ICACHE_AUTOLOAD_SCT1_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_autoload_section1_size](index.html) module"]
pub struct PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_autoload_section1_size::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_autoload_section1_size::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_SECTION1_SIZE to value 0x8000"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
