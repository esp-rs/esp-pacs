#[doc = "Register `HOST_SLCHOST_TOKEN_CON` writer"]
pub type W = crate::W<HOST_SLCHOST_TOKEN_CON_SPEC>;
#[doc = "Field `HOST_SLC0HOST_TOKEN0_DEC` writer - "]
pub type HOST_SLC0HOST_TOKEN0_DEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC0HOST_TOKEN1_DEC` writer - "]
pub type HOST_SLC0HOST_TOKEN1_DEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC0HOST_TOKEN0_WR` writer - "]
pub type HOST_SLC0HOST_TOKEN0_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC0HOST_TOKEN1_WR` writer - "]
pub type HOST_SLC0HOST_TOKEN1_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC1HOST_TOKEN0_DEC` writer - "]
pub type HOST_SLC1HOST_TOKEN0_DEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC1HOST_TOKEN1_DEC` writer - "]
pub type HOST_SLC1HOST_TOKEN1_DEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC1HOST_TOKEN0_WR` writer - "]
pub type HOST_SLC1HOST_TOKEN0_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC1HOST_TOKEN1_WR` writer - "]
pub type HOST_SLC1HOST_TOKEN1_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HOST_SLC0HOST_LEN_WR` writer - "]
pub type HOST_SLC0HOST_LEN_WR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_TOKEN_CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_token0_dec(
        &mut self,
    ) -> HOST_SLC0HOST_TOKEN0_DEC_W<HOST_SLCHOST_TOKEN_CON_SPEC, 0> {
        HOST_SLC0HOST_TOKEN0_DEC_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_token1_dec(
        &mut self,
    ) -> HOST_SLC0HOST_TOKEN1_DEC_W<HOST_SLCHOST_TOKEN_CON_SPEC, 1> {
        HOST_SLC0HOST_TOKEN1_DEC_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_token0_wr(
        &mut self,
    ) -> HOST_SLC0HOST_TOKEN0_WR_W<HOST_SLCHOST_TOKEN_CON_SPEC, 2> {
        HOST_SLC0HOST_TOKEN0_WR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_token1_wr(
        &mut self,
    ) -> HOST_SLC0HOST_TOKEN1_WR_W<HOST_SLCHOST_TOKEN_CON_SPEC, 3> {
        HOST_SLC0HOST_TOKEN1_WR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token0_dec(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN0_DEC_W<HOST_SLCHOST_TOKEN_CON_SPEC, 4> {
        HOST_SLC1HOST_TOKEN0_DEC_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token1_dec(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN1_DEC_W<HOST_SLCHOST_TOKEN_CON_SPEC, 5> {
        HOST_SLC1HOST_TOKEN1_DEC_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token0_wr(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN0_WR_W<HOST_SLCHOST_TOKEN_CON_SPEC, 6> {
        HOST_SLC1HOST_TOKEN0_WR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token1_wr(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN1_WR_W<HOST_SLCHOST_TOKEN_CON_SPEC, 7> {
        HOST_SLC1HOST_TOKEN1_WR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_len_wr(
        &mut self,
    ) -> HOST_SLC0HOST_LEN_WR_W<HOST_SLCHOST_TOKEN_CON_SPEC, 8> {
        HOST_SLC0HOST_LEN_WR_W::new(self)
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
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_token_con::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_TOKEN_CON_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_TOKEN_CON_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`host_slchost_token_con::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_TOKEN_CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_TOKEN_CON to value 0"]
impl crate::Resettable for HOST_SLCHOST_TOKEN_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
