#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SR_EOF` reader - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
pub type SR_EOF_R = crate::BitReader;
#[doc = "Field `SR_EOF` writer - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
pub type SR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_EOF` reader - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
pub type BLEND_EOF_R = crate::BitReader;
#[doc = "Field `BLEND_EOF` writer - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
pub type BLEND_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_PARAM_CFG_ERR` reader - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
pub type SR_PARAM_CFG_ERR_R = crate::BitReader;
#[doc = "Field `SR_PARAM_CFG_ERR` writer - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
pub type SR_PARAM_CFG_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
    #[inline(always)]
    pub fn sr_eof(&self) -> SR_EOF_R {
        SR_EOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
    #[inline(always)]
    pub fn blend_eof(&self) -> BLEND_EOF_R {
        BLEND_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
    #[inline(always)]
    pub fn sr_param_cfg_err(&self) -> SR_PARAM_CFG_ERR_R {
        SR_PARAM_CFG_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("sr_eof", &self.sr_eof())
            .field("blend_eof", &self.blend_eof())
            .field("sr_param_cfg_err", &self.sr_param_cfg_err())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image."]
    #[inline(always)]
    pub fn sr_eof(&mut self) -> SR_EOF_W<INT_RAW_SPEC> {
        SR_EOF_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when blending engine calculate one frame image."]
    #[inline(always)]
    pub fn blend_eof(&mut self) -> BLEND_EOF_W<INT_RAW_SPEC> {
        BLEND_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG."]
    #[inline(always)]
    pub fn sr_param_cfg_err(&mut self) -> SR_PARAM_CFG_ERR_W<INT_RAW_SPEC> {
        SR_PARAM_CFG_ERR_W::new(self, 2)
    }
}
#[doc = "Raw status interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
