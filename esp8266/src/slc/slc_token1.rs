#[doc = "Register `SLC_TOKEN1` reader"]
pub type R = crate::R<SLC_TOKEN1_SPEC>;
#[doc = "Register `SLC_TOKEN1` writer"]
pub type W = crate::W<SLC_TOKEN1_SPEC>;
#[doc = "Field `SLC_TOKEN1_LOCAL_WDATA` reader - "]
pub type SLC_TOKEN1_LOCAL_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC_TOKEN1_LOCAL_WDATA` writer - "]
pub type SLC_TOKEN1_LOCAL_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `SLC_TOKEN1_LOCAL_WR` reader - "]
pub type SLC_TOKEN1_LOCAL_WR_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN1_LOCAL_WR` writer - "]
pub type SLC_TOKEN1_LOCAL_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TOKEN1_LOCAL_INC` reader - "]
pub type SLC_TOKEN1_LOCAL_INC_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN1_LOCAL_INC` writer - "]
pub type SLC_TOKEN1_LOCAL_INC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TOKEN1_LOCAL_INC_MORE` reader - "]
pub type SLC_TOKEN1_LOCAL_INC_MORE_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN1_LOCAL_INC_MORE` writer - "]
pub type SLC_TOKEN1_LOCAL_INC_MORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TOKEN1` reader - "]
pub type SLC_TOKEN1_R = crate::FieldReader<u16>;
#[doc = "Field `SLC_TOKEN1` writer - "]
pub type SLC_TOKEN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc_token1_local_wdata(&self) -> SLC_TOKEN1_LOCAL_WDATA_R {
        SLC_TOKEN1_LOCAL_WDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_token1_local_wr(&self) -> SLC_TOKEN1_LOCAL_WR_R {
        SLC_TOKEN1_LOCAL_WR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_token1_local_inc(&self) -> SLC_TOKEN1_LOCAL_INC_R {
        SLC_TOKEN1_LOCAL_INC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_token1_local_inc_more(&self) -> SLC_TOKEN1_LOCAL_INC_MORE_R {
        SLC_TOKEN1_LOCAL_INC_MORE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc_token1(&self) -> SLC_TOKEN1_R {
        SLC_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TOKEN1")
            .field("slc_token1", &format_args!("{}", self.slc_token1().bits()))
            .field(
                "slc_token1_local_inc_more",
                &format_args!("{}", self.slc_token1_local_inc_more().bit()),
            )
            .field(
                "slc_token1_local_inc",
                &format_args!("{}", self.slc_token1_local_inc().bit()),
            )
            .field(
                "slc_token1_local_wr",
                &format_args!("{}", self.slc_token1_local_wr().bit()),
            )
            .field(
                "slc_token1_local_wdata",
                &format_args!("{}", self.slc_token1_local_wdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TOKEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1_local_wdata(&mut self) -> SLC_TOKEN1_LOCAL_WDATA_W<SLC_TOKEN1_SPEC, 0> {
        SLC_TOKEN1_LOCAL_WDATA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1_local_wr(&mut self) -> SLC_TOKEN1_LOCAL_WR_W<SLC_TOKEN1_SPEC, 12> {
        SLC_TOKEN1_LOCAL_WR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1_local_inc(&mut self) -> SLC_TOKEN1_LOCAL_INC_W<SLC_TOKEN1_SPEC, 13> {
        SLC_TOKEN1_LOCAL_INC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1_local_inc_more(
        &mut self,
    ) -> SLC_TOKEN1_LOCAL_INC_MORE_W<SLC_TOKEN1_SPEC, 14> {
        SLC_TOKEN1_LOCAL_INC_MORE_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token1(&mut self) -> SLC_TOKEN1_W<SLC_TOKEN1_SPEC, 16> {
        SLC_TOKEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_TOKEN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_token1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_token1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TOKEN1_SPEC;
impl crate::RegisterSpec for SLC_TOKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_token1::R`](R) reader structure"]
impl crate::Readable for SLC_TOKEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_token1::W`](W) writer structure"]
impl crate::Writable for SLC_TOKEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_TOKEN1 to value 0"]
impl crate::Resettable for SLC_TOKEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
