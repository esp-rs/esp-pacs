#[doc = "Register `BTINTACK` reader"]
pub type R = crate::R<BTINTACK_SPEC>;
#[doc = "Register `BTINTACK` writer"]
pub type W = crate::W<BTINTACK_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BR/EDR interrupt acknowledgement register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTINTACK_SPEC;
impl crate::RegisterSpec for BTINTACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btintack::R`](R) reader structure"]
impl crate::Readable for BTINTACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btintack::W`](W) writer structure"]
impl crate::Writable for BTINTACK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTINTACK to value 0"]
impl crate::Resettable for BTINTACK_SPEC {}
