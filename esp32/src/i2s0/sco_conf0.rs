#[doc = "Register `SCO_CONF0` reader"]
pub type R = crate::R<SCO_CONF0_SPEC>;
#[doc = "Register `SCO_CONF0` writer"]
pub type W = crate::W<SCO_CONF0_SPEC>;
#[doc = "Field `SCO_WITH_I2S_EN` reader - "]
pub type SCO_WITH_I2S_EN_R = crate::BitReader;
#[doc = "Field `SCO_WITH_I2S_EN` writer - "]
pub type SCO_WITH_I2S_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCO_NO_I2S_EN` reader - "]
pub type SCO_NO_I2S_EN_R = crate::BitReader;
#[doc = "Field `SCO_NO_I2S_EN` writer - "]
pub type SCO_NO_I2S_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CVSD_ENC_START` reader - "]
pub type CVSD_ENC_START_R = crate::BitReader;
#[doc = "Field `CVSD_ENC_START` writer - "]
pub type CVSD_ENC_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CVSD_ENC_RESET` reader - "]
pub type CVSD_ENC_RESET_R = crate::BitReader;
#[doc = "Field `CVSD_ENC_RESET` writer - "]
pub type CVSD_ENC_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sco_with_i2s_en(&self) -> SCO_WITH_I2S_EN_R {
        SCO_WITH_I2S_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sco_no_i2s_en(&self) -> SCO_NO_I2S_EN_R {
        SCO_NO_I2S_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cvsd_enc_start(&self) -> CVSD_ENC_START_R {
        CVSD_ENC_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cvsd_enc_reset(&self) -> CVSD_ENC_RESET_R {
        CVSD_ENC_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCO_CONF0")
            .field(
                "sco_with_i2s_en",
                &format_args!("{}", self.sco_with_i2s_en().bit()),
            )
            .field(
                "sco_no_i2s_en",
                &format_args!("{}", self.sco_no_i2s_en().bit()),
            )
            .field(
                "cvsd_enc_start",
                &format_args!("{}", self.cvsd_enc_start().bit()),
            )
            .field(
                "cvsd_enc_reset",
                &format_args!("{}", self.cvsd_enc_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCO_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sco_with_i2s_en(&mut self) -> SCO_WITH_I2S_EN_W<SCO_CONF0_SPEC, 0> {
        SCO_WITH_I2S_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sco_no_i2s_en(&mut self) -> SCO_NO_I2S_EN_W<SCO_CONF0_SPEC, 1> {
        SCO_NO_I2S_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_enc_start(&mut self) -> CVSD_ENC_START_W<SCO_CONF0_SPEC, 2> {
        CVSD_ENC_START_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_enc_reset(&mut self) -> CVSD_ENC_RESET_W<SCO_CONF0_SPEC, 3> {
        CVSD_ENC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sco_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sco_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCO_CONF0_SPEC;
impl crate::RegisterSpec for SCO_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sco_conf0::R`](R) reader structure"]
impl crate::Readable for SCO_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sco_conf0::W`](W) writer structure"]
impl crate::Writable for SCO_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCO_CONF0 to value 0"]
impl crate::Resettable for SCO_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
