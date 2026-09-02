#[doc = "Register `ADDR0LOW` reader"]
pub type R = crate::R<ADDR0LOW_SPEC>;
#[doc = "Register `ADDR0LOW` writer"]
pub type W = crate::W<ADDR0LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Contains the lower 32 bits of the first MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR0LOW_SPEC;
impl crate::RegisterSpec for ADDR0LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0low::R`](R) reader structure"]
impl crate::Readable for ADDR0LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr0low::W`](W) writer structure"]
impl crate::Writable for ADDR0LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR0LOW to value 0xffff_ffff"]
impl crate::Resettable for ADDR0LOW_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
