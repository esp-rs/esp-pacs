#[doc = "Register `I2SCONF_SIGLE_DATA` reader"]
pub type R = crate::R<I2SCONF_SIGLE_DATA_SPEC>;
#[doc = "Register `I2SCONF_SIGLE_DATA` writer"]
pub type W = crate::W<I2SCONF_SIGLE_DATA_SPEC>;
#[doc = "Field `I2S_I2S_SIGLE_DATA` reader - "]
pub type I2S_I2S_SIGLE_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `I2S_I2S_SIGLE_DATA` writer - "]
pub type I2S_I2S_SIGLE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2s_sigle_data(&self) -> I2S_I2S_SIGLE_DATA_R {
        I2S_I2S_SIGLE_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCONF_SIGLE_DATA")
            .field(
                "i2s_i2s_sigle_data",
                &format_args!("{}", self.i2s_i2s_sigle_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2SCONF_SIGLE_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_sigle_data(&mut self) -> I2S_I2S_SIGLE_DATA_W<I2SCONF_SIGLE_DATA_SPEC> {
        I2S_I2S_SIGLE_DATA_W::new(self, 0)
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
#[doc = "I2SCONF_SIGLE_DATA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sconf_sigle_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sconf_sigle_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for I2SCONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sconf_sigle_data::R`](R) reader structure"]
impl crate::Readable for I2SCONF_SIGLE_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sconf_sigle_data::W`](W) writer structure"]
impl crate::Writable for I2SCONF_SIGLE_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCONF_SIGLE_DATA to value 0"]
impl crate::Resettable for I2SCONF_SIGLE_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
