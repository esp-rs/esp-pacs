#[doc = "Register `MEM_REGISTERRND_ECO_LOW` reader"]
pub type R = crate::R<MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "Register `MEM_REGISTERRND_ECO_LOW` writer"]
pub type W = crate::W<MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "Field `MEM_REGISTERRND_ECO_LOW` reader - ECO low register"]
pub type MEM_REGISTERRND_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_REGISTERRND_ECO_LOW` writer - ECO low register"]
pub type MEM_REGISTERRND_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn mem_registerrnd_eco_low(&self) -> MEM_REGISTERRND_ECO_LOW_R {
        MEM_REGISTERRND_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_REGISTERRND_ECO_LOW")
            .field("mem_registerrnd_eco_low", &self.mem_registerrnd_eco_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn mem_registerrnd_eco_low(
        &mut self,
    ) -> MEM_REGISTERRND_ECO_LOW_W<MEM_REGISTERRND_ECO_LOW_SPEC> {
        MEM_REGISTERRND_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "MSPI ECO low register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_registerrnd_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_registerrnd_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_REGISTERRND_ECO_LOW_SPEC;
impl crate::RegisterSpec for MEM_REGISTERRND_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_registerrnd_eco_low::R`](R) reader structure"]
impl crate::Readable for MEM_REGISTERRND_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_registerrnd_eco_low::W`](W) writer structure"]
impl crate::Writable for MEM_REGISTERRND_ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_REGISTERRND_ECO_LOW to value 0x037c"]
impl crate::Resettable for MEM_REGISTERRND_ECO_LOW_SPEC {
    const RESET_VALUE: u32 = 0x037c;
}
