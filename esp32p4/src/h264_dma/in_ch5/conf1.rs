#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `BLOCK_START_ADDR` reader - RX Channel 5 destination start address"]
pub type BLOCK_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK_START_ADDR` writer - RX Channel 5 destination start address"]
pub type BLOCK_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    pub fn block_start_addr(&self) -> BLOCK_START_ADDR_R {
        BLOCK_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("block_start_addr", &self.block_start_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    pub fn block_start_addr(&mut self) -> BLOCK_START_ADDR_W<CONF1_SPEC> {
        BLOCK_START_ADDR_W::new(self, 0)
    }
}
#[doc = "RX CH5 config1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
