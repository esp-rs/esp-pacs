#[doc = "Register `PRO_ICACHE_AUTOLOAD_CFG` reader"]
pub struct R(crate::R<PRO_ICACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_AUTOLOAD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_AUTOLOAD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_AUTOLOAD_CFG` writer"]
pub struct W(crate::W<PRO_ICACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_AUTOLOAD_CFG_SPEC>;
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
impl From<crate::W<PRO_ICACHE_AUTOLOAD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_AUTOLOAD_MODE` reader - Reserved."]
pub type PRO_ICACHE_AUTOLOAD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_MODE` writer - Reserved."]
pub type PRO_ICACHE_AUTOLOAD_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, bool, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_STEP` reader - Reserved."]
pub type PRO_ICACHE_AUTOLOAD_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_STEP` writer - Reserved."]
pub type PRO_ICACHE_AUTOLOAD_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_ICACHE_AUTOLOAD_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_ICACHE_AUTOLOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, bool, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_ICACHE_AUTOLOAD_RQST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_ICACHE_AUTOLOAD_RQST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SCT0_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, bool, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_ICACHE_AUTOLOAD_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_icache_autoload_mode(&self) -> PRO_ICACHE_AUTOLOAD_MODE_R {
        PRO_ICACHE_AUTOLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_icache_autoload_step(&self) -> PRO_ICACHE_AUTOLOAD_STEP_R {
        PRO_ICACHE_AUTOLOAD_STEP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_icache_autoload_order(&self) -> PRO_ICACHE_AUTOLOAD_ORDER_R {
        PRO_ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_icache_autoload_rqst(&self) -> PRO_ICACHE_AUTOLOAD_RQST_R {
        PRO_ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_size(&self) -> PRO_ICACHE_AUTOLOAD_SIZE_R {
        PRO_ICACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct0_ena(&self) -> PRO_ICACHE_AUTOLOAD_SCT0_ENA_R {
        PRO_ICACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_ena(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_ENA_R {
        PRO_ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_icache_autoload_mode(&mut self) -> PRO_ICACHE_AUTOLOAD_MODE_W<0> {
        PRO_ICACHE_AUTOLOAD_MODE_W::new(self)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_icache_autoload_step(&mut self) -> PRO_ICACHE_AUTOLOAD_STEP_W<1> {
        PRO_ICACHE_AUTOLOAD_STEP_W::new(self)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_icache_autoload_order(&mut self) -> PRO_ICACHE_AUTOLOAD_ORDER_W<3> {
        PRO_ICACHE_AUTOLOAD_ORDER_W::new(self)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_icache_autoload_rqst(&mut self) -> PRO_ICACHE_AUTOLOAD_RQST_W<4> {
        PRO_ICACHE_AUTOLOAD_RQST_W::new(self)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_size(&mut self) -> PRO_ICACHE_AUTOLOAD_SIZE_W<6> {
        PRO_ICACHE_AUTOLOAD_SIZE_W::new(self)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct0_ena(&mut self) -> PRO_ICACHE_AUTOLOAD_SCT0_ENA_W<8> {
        PRO_ICACHE_AUTOLOAD_SCT0_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_ena(&mut self) -> PRO_ICACHE_AUTOLOAD_SCT1_ENA_W<9> {
        PRO_ICACHE_AUTOLOAD_SCT1_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_autoload_cfg](index.html) module"]
pub struct PRO_ICACHE_AUTOLOAD_CFG_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_autoload_cfg::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_autoload_cfg::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_CFG to value 0"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
