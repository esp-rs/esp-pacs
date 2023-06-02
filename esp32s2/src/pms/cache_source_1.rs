#[doc = "Register `CACHE_SOURCE_1` reader"]
pub struct R(crate::R<CACHE_SOURCE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SOURCE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SOURCE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SOURCE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SOURCE_1` writer"]
pub struct W(crate::W<CACHE_SOURCE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SOURCE_1_SPEC>;
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
impl From<crate::W<CACHE_SOURCE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SOURCE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` reader - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_IRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IRAM1` writer - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_IRAM1_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IROM0` reader - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_IROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_IROM0` writer - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_IROM0_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_DROM0` reader - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_DROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_I_SOURCE_PRO_DROM0` writer - xx"]
pub type PRO_CACHE_I_SOURCE_PRO_DROM0_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` reader - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DRAM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DRAM0` writer - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DRAM0_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DPORT` reader - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DPORT_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DPORT` writer - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DPORT_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DROM0` reader - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_D_SOURCE_PRO_DROM0` writer - xx"]
pub type PRO_CACHE_D_SOURCE_PRO_DROM0_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SOURCE_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_iram1(&self) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_R {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_irom0(&self) -> PRO_CACHE_I_SOURCE_PRO_IROM0_R {
        PRO_CACHE_I_SOURCE_PRO_IROM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - xx"]
    #[inline(always)]
    pub fn pro_cache_i_source_pro_drom0(&self) -> PRO_CACHE_I_SOURCE_PRO_DROM0_R {
        PRO_CACHE_I_SOURCE_PRO_DROM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dram0(&self) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_R {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_dport(&self) -> PRO_CACHE_D_SOURCE_PRO_DPORT_R {
        PRO_CACHE_D_SOURCE_PRO_DPORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - xx"]
    #[inline(always)]
    pub fn pro_cache_d_source_pro_drom0(&self) -> PRO_CACHE_D_SOURCE_PRO_DROM0_R {
        PRO_CACHE_D_SOURCE_PRO_DROM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SOURCE_1")
            .field(
                "pro_cache_i_source_pro_iram1",
                &format_args!("{}", self.pro_cache_i_source_pro_iram1().bit()),
            )
            .field(
                "pro_cache_i_source_pro_irom0",
                &format_args!("{}", self.pro_cache_i_source_pro_irom0().bit()),
            )
            .field(
                "pro_cache_i_source_pro_drom0",
                &format_args!("{}", self.pro_cache_i_source_pro_drom0().bit()),
            )
            .field(
                "pro_cache_d_source_pro_dram0",
                &format_args!("{}", self.pro_cache_d_source_pro_dram0().bit()),
            )
            .field(
                "pro_cache_d_source_pro_dport",
                &format_args!("{}", self.pro_cache_d_source_pro_dport().bit()),
            )
            .field(
                "pro_cache_d_source_pro_drom0",
                &format_args!("{}", self.pro_cache_d_source_pro_drom0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_SOURCE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_iram1(&mut self) -> PRO_CACHE_I_SOURCE_PRO_IRAM1_W<0> {
        PRO_CACHE_I_SOURCE_PRO_IRAM1_W::new(self)
    }
    #[doc = "Bit 1 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_irom0(&mut self) -> PRO_CACHE_I_SOURCE_PRO_IROM0_W<1> {
        PRO_CACHE_I_SOURCE_PRO_IROM0_W::new(self)
    }
    #[doc = "Bit 2 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_i_source_pro_drom0(&mut self) -> PRO_CACHE_I_SOURCE_PRO_DROM0_W<2> {
        PRO_CACHE_I_SOURCE_PRO_DROM0_W::new(self)
    }
    #[doc = "Bit 3 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_dram0(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DRAM0_W<3> {
        PRO_CACHE_D_SOURCE_PRO_DRAM0_W::new(self)
    }
    #[doc = "Bit 4 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_dport(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DPORT_W<4> {
        PRO_CACHE_D_SOURCE_PRO_DPORT_W::new(self)
    }
    #[doc = "Bit 5 - xx"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_d_source_pro_drom0(&mut self) -> PRO_CACHE_D_SOURCE_PRO_DROM0_W<5> {
        PRO_CACHE_D_SOURCE_PRO_DROM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache access permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_source_1](index.html) module"]
pub struct CACHE_SOURCE_1_SPEC;
impl crate::RegisterSpec for CACHE_SOURCE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_source_1::R](R) reader structure"]
impl crate::Readable for CACHE_SOURCE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_source_1::W](W) writer structure"]
impl crate::Writable for CACHE_SOURCE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_SOURCE_1 to value 0"]
impl crate::Resettable for CACHE_SOURCE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
