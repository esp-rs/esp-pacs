#[doc = "Register `BLECONF` reader"]
pub type R = crate::R<BLECONF_SPEC>;
#[doc = "Register `BLECONF` writer"]
pub type W = crate::W<BLECONF_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BLE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLECONF_SPEC;
impl crate::RegisterSpec for BLECONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleconf::R`](R) reader structure"]
impl crate::Readable for BLECONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleconf::W`](W) writer structure"]
impl crate::Writable for BLECONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLECONF to value 0"]
impl crate::Resettable for BLECONF_SPEC {}
