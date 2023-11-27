#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SR_EOF_INT_RAW` reader - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
pub type SR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SR_EOF_INT_RAW` writer - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
pub type SR_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_EOF_INT_RAW` reader - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
pub type BLEND_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `BLEND_EOF_INT_RAW` writer - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
pub type BLEND_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_PARAM_CFG_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
pub type SR_PARAM_CFG_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SR_PARAM_CFG_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
pub type SR_PARAM_CFG_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
    #[inline(always)]
    pub fn sr_eof_int_raw(&self) -> SR_EOF_INT_RAW_R {
        SR_EOF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
    #[inline(always)]
    pub fn blend_eof_int_raw(&self) -> BLEND_EOF_INT_RAW_R {
        BLEND_EOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
    #[inline(always)]
    pub fn sr_param_cfg_err_int_raw(&self) -> SR_PARAM_CFG_ERR_INT_RAW_R {
        SR_PARAM_CFG_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "sr_eof_int_raw",
                &format_args!("{}", self.sr_eof_int_raw().bit()),
            )
            .field(
                "blend_eof_int_raw",
                &format_args!("{}", self.blend_eof_int_raw().bit()),
            )
            .field(
                "sr_param_cfg_err_int_raw",
                &format_args!("{}", self.sr_param_cfg_err_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
    #[inline(always)]
    #[must_use]
    pub fn sr_eof_int_raw(&mut self) -> SR_EOF_INT_RAW_W<INT_RAW_SPEC> {
        SR_EOF_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
    #[inline(always)]
    #[must_use]
    pub fn blend_eof_int_raw(&mut self) -> BLEND_EOF_INT_RAW_W<INT_RAW_SPEC> {
        BLEND_EOF_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
    #[inline(always)]
    #[must_use]
    pub fn sr_param_cfg_err_int_raw(&mut self) -> SR_PARAM_CFG_ERR_INT_RAW_W<INT_RAW_SPEC> {
        SR_PARAM_CFG_ERR_INT_RAW_W::new(self, 2)
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
#[doc = "Raw status interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
