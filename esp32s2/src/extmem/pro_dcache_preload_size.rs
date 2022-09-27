#[doc = "Register `PRO_DCACHE_PRELOAD_SIZE` reader"]
pub struct R(crate::R<PRO_DCACHE_PRELOAD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_PRELOAD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_PRELOAD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_PRELOAD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_PRELOAD_SIZE` writer"]
pub struct W(crate::W<PRO_DCACHE_PRELOAD_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_PRELOAD_SIZE_SPEC>;
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
impl From<crate::W<PRO_DCACHE_PRELOAD_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_PRELOAD_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DCACHE_PRELOAD_SIZE` reader - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
pub type PRO_DCACHE_PRELOAD_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_DCACHE_PRELOAD_SIZE` writer - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
pub type PRO_DCACHE_PRELOAD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_DCACHE_PRELOAD_SIZE_SPEC, u16, u16, 10, O>;
#[doc = "Field `PRO_DCACHE_PRELOAD_ORDER` reader - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_PRELOAD_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `PRO_DCACHE_PRELOAD_ORDER` writer - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_PRELOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_DCACHE_PRELOAD_SIZE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    pub fn pro_dcache_preload_size(&self) -> PRO_DCACHE_PRELOAD_SIZE_R {
        PRO_DCACHE_PRELOAD_SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_preload_order(&self) -> PRO_DCACHE_PRELOAD_ORDER_R {
        PRO_DCACHE_PRELOAD_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    pub fn pro_dcache_preload_size(&mut self) -> PRO_DCACHE_PRELOAD_SIZE_W<0> {
        PRO_DCACHE_PRELOAD_SIZE_W::new(self)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_preload_order(&mut self) -> PRO_DCACHE_PRELOAD_ORDER_W<10> {
        PRO_DCACHE_PRELOAD_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_preload_size](index.html) module"]
pub struct PRO_DCACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_preload_size::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_preload_size::W](W) writer structure"]
impl crate::Writable for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_PRELOAD_SIZE to value 0x0200"]
impl crate::Resettable for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
