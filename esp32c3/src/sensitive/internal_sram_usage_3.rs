#[doc = "Register `INTERNAL_SRAM_USAGE_3` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_3` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_3_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM` reader - internal_sram_usage_mac_dump_sram"]
pub type INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM` writer - internal_sram_usage_mac_dump_sram"]
pub type INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERNAL_SRAM_USAGE_3_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTERNAL_SRAM_ALLOC_MAC_DUMP` reader - internal_sram_alloc_mac_dump"]
pub type INTERNAL_SRAM_ALLOC_MAC_DUMP_R = crate::BitReader<bool>;
#[doc = "Field `INTERNAL_SRAM_ALLOC_MAC_DUMP` writer - internal_sram_alloc_mac_dump"]
pub type INTERNAL_SRAM_ALLOC_MAC_DUMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERNAL_SRAM_USAGE_3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - internal_sram_usage_mac_dump_sram"]
    #[inline(always)]
    pub fn internal_sram_usage_mac_dump_sram(&self) -> INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R {
        INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - internal_sram_alloc_mac_dump"]
    #[inline(always)]
    pub fn internal_sram_alloc_mac_dump(&self) -> INTERNAL_SRAM_ALLOC_MAC_DUMP_R {
        INTERNAL_SRAM_ALLOC_MAC_DUMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - internal_sram_usage_mac_dump_sram"]
    #[inline(always)]
    pub fn internal_sram_usage_mac_dump_sram(&mut self) -> INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<0> {
        INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W::new(self)
    }
    #[doc = "Bit 3 - internal_sram_alloc_mac_dump"]
    #[inline(always)]
    pub fn internal_sram_alloc_mac_dump(&mut self) -> INTERNAL_SRAM_ALLOC_MAC_DUMP_W<3> {
        INTERNAL_SRAM_ALLOC_MAC_DUMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_3](index.html) module"]
pub struct INTERNAL_SRAM_USAGE_3_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_3::R](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_3::W](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_3 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
