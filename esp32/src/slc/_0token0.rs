#[doc = "Register `_0TOKEN0` reader"]
pub type R = crate::R<_0TOKEN0_SPEC>;
#[doc = "Register `_0TOKEN0` writer"]
pub type W = crate::W<_0TOKEN0_SPEC>;
#[doc = "Field `SLC0_TOKEN0_WDATA` writer - "]
pub type SLC0_TOKEN0_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `SLC0_TOKEN0_WR` writer - "]
pub type SLC0_TOKEN0_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOKEN0_INC` writer - "]
pub type SLC0_TOKEN0_INC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOKEN0_INC_MORE` writer - "]
pub type SLC0_TOKEN0_INC_MORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOKEN0` reader - "]
pub type SLC0_TOKEN0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc0_token0(&self) -> SLC0_TOKEN0_R {
        SLC0_TOKEN0_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0TOKEN0")
            .field(
                "slc0_token0",
                &format_args!("{}", self.slc0_token0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0TOKEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_wdata(&mut self) -> SLC0_TOKEN0_WDATA_W<_0TOKEN0_SPEC, 0> {
        SLC0_TOKEN0_WDATA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_wr(&mut self) -> SLC0_TOKEN0_WR_W<_0TOKEN0_SPEC, 12> {
        SLC0_TOKEN0_WR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_inc(&mut self) -> SLC0_TOKEN0_INC_W<_0TOKEN0_SPEC, 13> {
        SLC0_TOKEN0_INC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_inc_more(&mut self) -> SLC0_TOKEN0_INC_MORE_W<_0TOKEN0_SPEC, 14> {
        SLC0_TOKEN0_INC_MORE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0token0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0token0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0TOKEN0_SPEC;
impl crate::RegisterSpec for _0TOKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0token0::R`](R) reader structure"]
impl crate::Readable for _0TOKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0token0::W`](W) writer structure"]
impl crate::Writable for _0TOKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0TOKEN0 to value 0"]
impl crate::Resettable for _0TOKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
