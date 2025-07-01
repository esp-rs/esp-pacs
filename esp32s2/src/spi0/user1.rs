#[doc = "Register `USER1` reader"]
pub type R = crate::R<USER1_SPEC>;
#[doc = "Register `USER1` writer"]
pub type W = crate::W<USER1_SPEC>;
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - "]
pub type USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - "]
pub type USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USR_ADDR_BITLEN` reader - "]
pub type USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `USR_ADDR_BITLEN` writer - "]
pub type USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER1")
            .field("usr_addr_bitlen", &self.usr_addr_bitlen())
            .field("usr_dummy_cyclelen", &self.usr_dummy_cyclelen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W<USER1_SPEC> {
        USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W<USER1_SPEC> {
        USR_ADDR_BITLEN_W::new(self, 26)
    }
}
#[doc = "SPI Memory User1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user1::R`](R) reader structure"]
impl crate::Readable for USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user1::W`](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER1 to value 0"]
impl crate::Resettable for USER1_SPEC {}
