#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` reader"]
pub struct R(crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` writer"]
pub struct W(crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
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
impl From<crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_addr(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_addr(&mut self) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<0> {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_autoload_section1_addr](index.html) module"]
pub struct PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_autoload_section1_addr::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_autoload_section1_addr::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_SECTION1_ADDR to value 0"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
