#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` reader"]
pub struct R(crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` writer"]
pub struct W(crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
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
impl From<crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` reader - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_MODE_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` writer - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` reader - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_STEP_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` writer - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, 2, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_AUTOLOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_DCACHE_AUTOLOAD_RQST_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_DCACHE_AUTOLOAD_RQST_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, 2, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SIZE_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, 2, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, O>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_DCACHE_AUTOLOAD_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_mode(&self) -> PRO_DCACHE_AUTOLOAD_MODE_R {
        PRO_DCACHE_AUTOLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_step(&self) -> PRO_DCACHE_AUTOLOAD_STEP_R {
        PRO_DCACHE_AUTOLOAD_STEP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_autoload_order(&self) -> PRO_DCACHE_AUTOLOAD_ORDER_R {
        PRO_DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_dcache_autoload_rqst(&self) -> PRO_DCACHE_AUTOLOAD_RQST_R {
        PRO_DCACHE_AUTOLOAD_RQST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_size(&self) -> PRO_DCACHE_AUTOLOAD_SIZE_R {
        PRO_DCACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct1_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_AUTOLOAD_CFG")
            .field(
                "pro_dcache_autoload_mode",
                &format_args!("{}", self.pro_dcache_autoload_mode().bit()),
            )
            .field(
                "pro_dcache_autoload_step",
                &format_args!("{}", self.pro_dcache_autoload_step().bits()),
            )
            .field(
                "pro_dcache_autoload_order",
                &format_args!("{}", self.pro_dcache_autoload_order().bit()),
            )
            .field(
                "pro_dcache_autoload_rqst",
                &format_args!("{}", self.pro_dcache_autoload_rqst().bits()),
            )
            .field(
                "pro_dcache_autoload_size",
                &format_args!("{}", self.pro_dcache_autoload_size().bits()),
            )
            .field(
                "pro_dcache_autoload_sct0_ena",
                &format_args!("{}", self.pro_dcache_autoload_sct0_ena().bit()),
            )
            .field(
                "pro_dcache_autoload_sct1_ena",
                &format_args!("{}", self.pro_dcache_autoload_sct1_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_mode(&mut self) -> PRO_DCACHE_AUTOLOAD_MODE_W<0> {
        PRO_DCACHE_AUTOLOAD_MODE_W::new(self)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_step(&mut self) -> PRO_DCACHE_AUTOLOAD_STEP_W<1> {
        PRO_DCACHE_AUTOLOAD_STEP_W::new(self)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_order(&mut self) -> PRO_DCACHE_AUTOLOAD_ORDER_W<3> {
        PRO_DCACHE_AUTOLOAD_ORDER_W::new(self)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_rqst(&mut self) -> PRO_DCACHE_AUTOLOAD_RQST_W<4> {
        PRO_DCACHE_AUTOLOAD_RQST_W::new(self)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_size(&mut self) -> PRO_DCACHE_AUTOLOAD_SIZE_W<6> {
        PRO_DCACHE_AUTOLOAD_SIZE_W::new(self)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct0_ena(&mut self) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<8> {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct1_ena(&mut self) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<9> {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_autoload_cfg](index.html) module"]
pub struct PRO_DCACHE_AUTOLOAD_CFG_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_autoload_cfg::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_autoload_cfg::W](W) writer structure"]
impl crate::Writable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_AUTOLOAD_CFG to value 0"]
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
