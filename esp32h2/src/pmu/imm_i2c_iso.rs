#[doc = "Register `IMM_I2C_ISO` writer"]
pub type W = crate::W<IMM_I2C_ISO_SPEC>;
#[doc = "Field `TIE_HIGH_I2C_ISO_EN` writer - need_des"]
pub type TIE_HIGH_I2C_ISO_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE_LOW_I2C_ISO_EN` writer - need_des"]
pub type TIE_LOW_I2C_ISO_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_I2C_ISO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_i2c_iso_en(&mut self) -> TIE_HIGH_I2C_ISO_EN_W<IMM_I2C_ISO_SPEC, 30> {
        TIE_HIGH_I2C_ISO_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_i2c_iso_en(&mut self) -> TIE_LOW_I2C_ISO_EN_W<IMM_I2C_ISO_SPEC, 31> {
        TIE_LOW_I2C_ISO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_i2c_iso::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_I2C_ISO_SPEC;
impl crate::RegisterSpec for IMM_I2C_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_i2c_iso::W`](W) writer structure"]
impl crate::Writable for IMM_I2C_ISO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_I2C_ISO to value 0"]
impl crate::Resettable for IMM_I2C_ISO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
