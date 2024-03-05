#[doc = "Register `CONF_SIGLE_DATA` reader"]
pub type R = crate::R<CONF_SIGLE_DATA_SPEC>;
#[doc = "Register `CONF_SIGLE_DATA` writer"]
pub type W = crate::W<CONF_SIGLE_DATA_SPEC>;
#[doc = "Field `SINGLE_DATA` reader - The configured constant channel data to be sent out."]
pub type SINGLE_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `SINGLE_DATA` writer - The configured constant channel data to be sent out."]
pub type SINGLE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    pub fn single_data(&self) -> SINGLE_DATA_R {
        SINGLE_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_SIGLE_DATA")
            .field(
                "single_data",
                &format_args!("{}", self.single_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SIGLE_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    #[must_use]
    pub fn single_data(&mut self) -> SINGLE_DATA_W<CONF_SIGLE_DATA_SPEC> {
        SINGLE_DATA_W::new(self, 0)
    }
}
#[doc = "I2S signal data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_sigle_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_sigle_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for CONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_sigle_data::R`](R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_sigle_data::W`](W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for CONF_SIGLE_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
