#[doc = "Register `CACHE_TAG_ACCESS_1` reader"]
pub struct R(crate::R<CACHE_TAG_ACCESS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_TAG_ACCESS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_TAG_ACCESS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_TAG_ACCESS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_TAG_ACCESS_1` writer"]
pub struct W(crate::W<CACHE_TAG_ACCESS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_TAG_ACCESS_1_SPEC>;
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
impl From<crate::W<CACHE_TAG_ACCESS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_TAG_ACCESS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_I_TAG_RD_ACS` reader - Set 1 to enable Icache read access tag memory."]
pub type PRO_I_TAG_RD_ACS_R = crate::BitReader;
#[doc = "Field `PRO_I_TAG_RD_ACS` writer - Set 1 to enable Icache read access tag memory."]
pub type PRO_I_TAG_RD_ACS_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_TAG_ACCESS_1_SPEC, O>;
#[doc = "Field `PRO_I_TAG_WR_ACS` reader - Set 1 to enable Icache wrtie access tag memory."]
pub type PRO_I_TAG_WR_ACS_R = crate::BitReader;
#[doc = "Field `PRO_I_TAG_WR_ACS` writer - Set 1 to enable Icache wrtie access tag memory."]
pub type PRO_I_TAG_WR_ACS_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_TAG_ACCESS_1_SPEC, O>;
#[doc = "Field `PRO_D_TAG_RD_ACS` reader - Set 1 to enable Dcache read access tag memory."]
pub type PRO_D_TAG_RD_ACS_R = crate::BitReader;
#[doc = "Field `PRO_D_TAG_RD_ACS` writer - Set 1 to enable Dcache read access tag memory."]
pub type PRO_D_TAG_RD_ACS_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_TAG_ACCESS_1_SPEC, O>;
#[doc = "Field `PRO_D_TAG_WR_ACS` reader - Set 1 to enable Dcache wrtie access tag memory."]
pub type PRO_D_TAG_WR_ACS_R = crate::BitReader;
#[doc = "Field `PRO_D_TAG_WR_ACS` writer - Set 1 to enable Dcache wrtie access tag memory."]
pub type PRO_D_TAG_WR_ACS_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_TAG_ACCESS_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable Icache read access tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_rd_acs(&self) -> PRO_I_TAG_RD_ACS_R {
        PRO_I_TAG_RD_ACS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable Icache wrtie access tag memory."]
    #[inline(always)]
    pub fn pro_i_tag_wr_acs(&self) -> PRO_I_TAG_WR_ACS_R {
        PRO_I_TAG_WR_ACS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable Dcache read access tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_rd_acs(&self) -> PRO_D_TAG_RD_ACS_R {
        PRO_D_TAG_RD_ACS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to enable Dcache wrtie access tag memory."]
    #[inline(always)]
    pub fn pro_d_tag_wr_acs(&self) -> PRO_D_TAG_WR_ACS_R {
        PRO_D_TAG_WR_ACS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TAG_ACCESS_1")
            .field(
                "pro_i_tag_rd_acs",
                &format_args!("{}", self.pro_i_tag_rd_acs().bit()),
            )
            .field(
                "pro_i_tag_wr_acs",
                &format_args!("{}", self.pro_i_tag_wr_acs().bit()),
            )
            .field(
                "pro_d_tag_rd_acs",
                &format_args!("{}", self.pro_d_tag_rd_acs().bit()),
            )
            .field(
                "pro_d_tag_wr_acs",
                &format_args!("{}", self.pro_d_tag_wr_acs().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_TAG_ACCESS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable Icache read access tag memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_i_tag_rd_acs(&mut self) -> PRO_I_TAG_RD_ACS_W<0> {
        PRO_I_TAG_RD_ACS_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable Icache wrtie access tag memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_i_tag_wr_acs(&mut self) -> PRO_I_TAG_WR_ACS_W<1> {
        PRO_I_TAG_WR_ACS_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable Dcache read access tag memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_d_tag_rd_acs(&mut self) -> PRO_D_TAG_RD_ACS_W<2> {
        PRO_D_TAG_RD_ACS_W::new(self)
    }
    #[doc = "Bit 3 - Set 1 to enable Dcache wrtie access tag memory."]
    #[inline(always)]
    #[must_use]
    pub fn pro_d_tag_wr_acs(&mut self) -> PRO_D_TAG_WR_ACS_W<3> {
        PRO_D_TAG_WR_ACS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache tag configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_tag_access_1](index.html) module"]
pub struct CACHE_TAG_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_TAG_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_tag_access_1::R](R) reader structure"]
impl crate::Readable for CACHE_TAG_ACCESS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_tag_access_1::W](W) writer structure"]
impl crate::Writable for CACHE_TAG_ACCESS_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_TAG_ACCESS_1 to value 0x0f"]
impl crate::Resettable for CACHE_TAG_ACCESS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
