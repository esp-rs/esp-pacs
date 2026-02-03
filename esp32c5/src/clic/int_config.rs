#[doc = "Register `INT_CONFIG` reader"]
pub type R = crate::R<INT_CONFIG_SPEC>;
#[doc = "Register `INT_CONFIG` writer"]
pub type W = crate::W<INT_CONFIG_SPEC>;
#[doc = "Field `MNLBITS` reader - Machine mode interrupt priority effective digits, the maximum value is 8."]
pub type MNLBITS_R = crate::FieldReader;
#[doc = "Field `MNLBITS` writer - Machine mode interrupt priority effective digits, the maximum value is 8."]
pub type MNLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NMBITS` reader - The effective number of bits in the privileged state."]
pub type NMBITS_R = crate::FieldReader;
#[doc = "Field `NMBITS` writer - The effective number of bits in the privileged state."]
pub type NMBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNLBITS` reader - Supervisor mode interrupt priority effective digits, the maximum value is 8."]
pub type SNLBITS_R = crate::FieldReader;
#[doc = "Field `SNLBITS` writer - Supervisor mode interrupt priority effective digits, the maximum value is 8."]
pub type SNLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UNLBITS` reader - User mode interrupt priority effective digits, the maximum value is 8."]
pub type UNLBITS_R = crate::FieldReader;
#[doc = "Field `UNLBITS` writer - User mode interrupt priority effective digits, the maximum value is 8."]
pub type UNLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Machine mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn mnlbits(&self) -> MNLBITS_R {
        MNLBITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - The effective number of bits in the privileged state."]
    #[inline(always)]
    pub fn nmbits(&self) -> NMBITS_R {
        NMBITS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Supervisor mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn snlbits(&self) -> SNLBITS_R {
        SNLBITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn unlbits(&self) -> UNLBITS_R {
        UNLBITS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CONFIG")
            .field("unlbits", &self.unlbits())
            .field("snlbits", &self.snlbits())
            .field("nmbits", &self.nmbits())
            .field("mnlbits", &self.mnlbits())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Machine mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn mnlbits(&mut self) -> MNLBITS_W<'_, INT_CONFIG_SPEC> {
        MNLBITS_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - The effective number of bits in the privileged state."]
    #[inline(always)]
    pub fn nmbits(&mut self) -> NMBITS_W<'_, INT_CONFIG_SPEC> {
        NMBITS_W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Supervisor mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn snlbits(&mut self) -> SNLBITS_W<'_, INT_CONFIG_SPEC> {
        SNLBITS_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - User mode interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn unlbits(&mut self) -> UNLBITS_W<'_, INT_CONFIG_SPEC> {
        UNLBITS_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CONFIG_SPEC;
impl crate::RegisterSpec for INT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_config::R`](R) reader structure"]
impl crate::Readable for INT_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_config::W`](W) writer structure"]
impl crate::Writable for INT_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CONFIG to value 0"]
impl crate::Resettable for INT_CONFIG_SPEC {}
