#[doc = "Register `MULT_PX_MEM[%s]` reader"]
pub type R = crate::R<MULT_PX_MEM_SPEC>;
#[doc = "Register `MULT_PX_MEM[%s]` writer"]
pub type W = crate::W<MULT_PX_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores Px.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_px_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_px_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_PX_MEM_SPEC;
impl crate::RegisterSpec for MULT_PX_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mult_px_mem::R`](R) reader structure"]
impl crate::Readable for MULT_PX_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_px_mem::W`](W) writer structure"]
impl crate::Writable for MULT_PX_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_PX_MEM[%s] to value 0"]
impl crate::Resettable for MULT_PX_MEM_SPEC {}
