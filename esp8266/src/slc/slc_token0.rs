#[doc = "Register `SLC_TOKEN0` reader"]
pub struct R(crate::R<SLC_TOKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_TOKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_TOKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_TOKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_TOKEN0` writer"]
pub struct W(crate::W<SLC_TOKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_TOKEN0_SPEC>;
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
impl From<crate::W<SLC_TOKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_TOKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TOKEN0_LOCAL_WDATA` reader - "]
pub type SLC_TOKEN0_LOCAL_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC_TOKEN0_LOCAL_WDATA` writer - "]
pub type SLC_TOKEN0_LOCAL_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLC_TOKEN0_SPEC, 12, O, u16, u16>;
#[doc = "Field `SLC_TOKEN0_LOCAL_WR` reader - "]
pub type SLC_TOKEN0_LOCAL_WR_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN0_LOCAL_WR` writer - "]
pub type SLC_TOKEN0_LOCAL_WR_W<'a, const O: u8> = crate::BitWriter<'a, SLC_TOKEN0_SPEC, O>;
#[doc = "Field `SLC_TOKEN0_LOCAL_INC` reader - "]
pub type SLC_TOKEN0_LOCAL_INC_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN0_LOCAL_INC` writer - "]
pub type SLC_TOKEN0_LOCAL_INC_W<'a, const O: u8> = crate::BitWriter<'a, SLC_TOKEN0_SPEC, O>;
#[doc = "Field `SLC_TOKEN0_LOCAL_INC_MORE` reader - "]
pub type SLC_TOKEN0_LOCAL_INC_MORE_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN0_LOCAL_INC_MORE` writer - "]
pub type SLC_TOKEN0_LOCAL_INC_MORE_W<'a, const O: u8> = crate::BitWriter<'a, SLC_TOKEN0_SPEC, O>;
#[doc = "Field `SLC_TOKEN0` reader - "]
pub type SLC_TOKEN0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC_TOKEN0` writer - "]
pub type SLC_TOKEN0_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_TOKEN0_SPEC, 12, O, u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_token0_local_wdata(&self) -> SLC_TOKEN0_LOCAL_WDATA_R {
        SLC_TOKEN0_LOCAL_WDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_token0_local_wr(&self) -> SLC_TOKEN0_LOCAL_WR_R {
        SLC_TOKEN0_LOCAL_WR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_token0_local_inc(&self) -> SLC_TOKEN0_LOCAL_INC_R {
        SLC_TOKEN0_LOCAL_INC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_token0_local_inc_more(&self) -> SLC_TOKEN0_LOCAL_INC_MORE_R {
        SLC_TOKEN0_LOCAL_INC_MORE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_token0(&self) -> SLC_TOKEN0_R {
        SLC_TOKEN0_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TOKEN0")
            .field("slc_token0", &format_args!("{}", self.slc_token0().bits()))
            .field(
                "slc_token0_local_inc_more",
                &format_args!("{}", self.slc_token0_local_inc_more().bit()),
            )
            .field(
                "slc_token0_local_inc",
                &format_args!("{}", self.slc_token0_local_inc().bit()),
            )
            .field(
                "slc_token0_local_wr",
                &format_args!("{}", self.slc_token0_local_wr().bit()),
            )
            .field(
                "slc_token0_local_wdata",
                &format_args!("{}", self.slc_token0_local_wdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TOKEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0_local_wdata(&mut self) -> SLC_TOKEN0_LOCAL_WDATA_W<0> {
        SLC_TOKEN0_LOCAL_WDATA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0_local_wr(&mut self) -> SLC_TOKEN0_LOCAL_WR_W<12> {
        SLC_TOKEN0_LOCAL_WR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0_local_inc(&mut self) -> SLC_TOKEN0_LOCAL_INC_W<13> {
        SLC_TOKEN0_LOCAL_INC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0_local_inc_more(&mut self) -> SLC_TOKEN0_LOCAL_INC_MORE_W<14> {
        SLC_TOKEN0_LOCAL_INC_MORE_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token0(&mut self) -> SLC_TOKEN0_W<16> {
        SLC_TOKEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_TOKEN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_token0](index.html) module"]
pub struct SLC_TOKEN0_SPEC;
impl crate::RegisterSpec for SLC_TOKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_token0::R](R) reader structure"]
impl crate::Readable for SLC_TOKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_token0::W](W) writer structure"]
impl crate::Writable for SLC_TOKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_TOKEN0 to value 0"]
impl crate::Resettable for SLC_TOKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
