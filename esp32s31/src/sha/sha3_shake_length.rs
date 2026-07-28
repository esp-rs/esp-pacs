#[doc = "Register `SHA3_SHAKE_LENGTH` reader"]
pub type R = crate::R<SHA3_SHAKE_LENGTH_SPEC>;
#[doc = "Register `SHA3_SHAKE_LENGTH` writer"]
pub type W = crate::W<SHA3_SHAKE_LENGTH_SPEC>;
#[doc = "Field `SHAKE_LENGTH` reader - SHAKE output hash word length"]
pub type SHAKE_LENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `SHAKE_LENGTH` writer - SHAKE output hash word length"]
pub type SHAKE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - SHAKE output hash word length"]
    #[inline(always)]
    pub fn shake_length(&self) -> SHAKE_LENGTH_R {
        SHAKE_LENGTH_R::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA3_SHAKE_LENGTH")
            .field("shake_length", &self.shake_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:20 - SHAKE output hash word length"]
    #[inline(always)]
    pub fn shake_length(&mut self) -> SHAKE_LENGTH_W<'_, SHA3_SHAKE_LENGTH_SPEC> {
        SHAKE_LENGTH_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sha3_shake_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha3_shake_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA3_SHAKE_LENGTH_SPEC;
impl crate::RegisterSpec for SHA3_SHAKE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha3_shake_length::R`](R) reader structure"]
impl crate::Readable for SHA3_SHAKE_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sha3_shake_length::W`](W) writer structure"]
impl crate::Writable for SHA3_SHAKE_LENGTH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA3_SHAKE_LENGTH to value 0x32"]
impl crate::Resettable for SHA3_SHAKE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0x32;
}
