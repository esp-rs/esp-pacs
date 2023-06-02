#[doc = "Register `L1_CACHE_PRELOCK_CONF` reader"]
pub struct R(crate::R<L1_CACHE_PRELOCK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_PRELOCK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_PRELOCK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_PRELOCK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_PRELOCK_CONF` writer"]
pub struct W(crate::W<L1_CACHE_PRELOCK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_PRELOCK_CONF_SPEC>;
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
impl From<crate::W<L1_CACHE_PRELOCK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_PRELOCK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-Cache."]
pub type L1_CACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on L1-Cache."]
pub type L1_CACHE_PRELOCK_SCT0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_PRELOCK_CONF_SPEC, O>;
#[doc = "Field `L1_CACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-Cache."]
pub type L1_CACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on L1-Cache."]
pub type L1_CACHE_PRELOCK_SCT1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_PRELOCK_CONF_SPEC, O>;
#[doc = "Field `L1_CACHE_PRELOCK_RGID` reader - The bit is used to set the gid of l1 cache prelock."]
pub type L1_CACHE_PRELOCK_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_prelock_sct0_en(&self) -> L1_CACHE_PRELOCK_SCT0_EN_R {
        L1_CACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_prelock_sct1_en(&self) -> L1_CACHE_PRELOCK_SCT1_EN_R {
        L1_CACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 cache prelock."]
    #[inline(always)]
    pub fn l1_cache_prelock_rgid(&self) -> L1_CACHE_PRELOCK_RGID_R {
        L1_CACHE_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_PRELOCK_CONF")
            .field(
                "l1_cache_prelock_sct0_en",
                &format_args!("{}", self.l1_cache_prelock_sct0_en().bit()),
            )
            .field(
                "l1_cache_prelock_sct1_en",
                &format_args!("{}", self.l1_cache_prelock_sct1_en().bit()),
            )
            .field(
                "l1_cache_prelock_rgid",
                &format_args!("{}", self.l1_cache_prelock_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_PRELOCK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_prelock_sct0_en(&mut self) -> L1_CACHE_PRELOCK_SCT0_EN_W<0> {
        L1_CACHE_PRELOCK_SCT0_EN_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_prelock_sct1_en(&mut self) -> L1_CACHE_PRELOCK_SCT1_EN_W<1> {
        L1_CACHE_PRELOCK_SCT1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 Cache prelock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_prelock_conf](index.html) module"]
pub struct L1_CACHE_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for L1_CACHE_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_prelock_conf::R](R) reader structure"]
impl crate::Readable for L1_CACHE_PRELOCK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_prelock_conf::W](W) writer structure"]
impl crate::Writable for L1_CACHE_PRELOCK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_PRELOCK_CONF to value 0"]
impl crate::Resettable for L1_CACHE_PRELOCK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
