#[doc = "Register `ADDR_HIGH` reader"]
pub type R = crate::R<ADDR_HIGH_SPEC>;
#[doc = "Register `ADDR_HIGH` writer"]
pub type W = crate::W<ADDR_HIGH_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ALGORITHM` reader - "]
pub type ALGORITHM_R = crate::FieldReader;
#[doc = "Field `ALGORITHM` writer - "]
pub type ALGORITHM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTERFACE` reader - "]
pub type INTERFACE_R = crate::FieldReader;
#[doc = "Field `INTERFACE` writer - "]
pub type INTERFACE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUPPLICANT_KEY_INDEX` reader - "]
pub type SUPPLICANT_KEY_INDEX_R = crate::FieldReader;
#[doc = "Field `SUPPLICANT_KEY_INDEX` writer - "]
pub type SUPPLICANT_KEY_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn algorithm(&self) -> ALGORITHM_R {
        ALGORITHM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn interface(&self) -> INTERFACE_R {
        INTERFACE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn supplicant_key_index(&self) -> SUPPLICANT_KEY_INDEX_R {
        SUPPLICANT_KEY_INDEX_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_HIGH")
            .field("addr", &self.addr())
            .field("algorithm", &self.algorithm())
            .field("interface", &self.interface())
            .field("supplicant_key_index", &self.supplicant_key_index())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<ADDR_HIGH_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn algorithm(&mut self) -> ALGORITHM_W<ADDR_HIGH_SPEC> {
        ALGORITHM_W::new(self, 18)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn interface(&mut self) -> INTERFACE_W<ADDR_HIGH_SPEC> {
        INTERFACE_W::new(self, 24)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn supplicant_key_index(&mut self) -> SUPPLICANT_KEY_INDEX_W<ADDR_HIGH_SPEC> {
        SUPPLICANT_KEY_INDEX_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_HIGH_SPEC;
impl crate::RegisterSpec for ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_high::R`](R) reader structure"]
impl crate::Readable for ADDR_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_high::W`](W) writer structure"]
impl crate::Writable for ADDR_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR_HIGH to value 0"]
impl crate::Resettable for ADDR_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
