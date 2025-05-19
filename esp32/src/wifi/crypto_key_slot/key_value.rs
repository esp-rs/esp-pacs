#[doc = "Register `KEY_VALUE%s` reader"]
pub type R = crate::R<KEY_VALUE_SPEC>;
#[doc = "Register `KEY_VALUE%s` writer"]
pub type W = crate::W<KEY_VALUE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Actual key data\n\nYou can [`read`](crate::Reg::read) this register and get [`key_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_VALUE_SPEC;
impl crate::RegisterSpec for KEY_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_value::R`](R) reader structure"]
impl crate::Readable for KEY_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_value::W`](W) writer structure"]
impl crate::Writable for KEY_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_VALUE%s to value 0"]
impl crate::Resettable for KEY_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
