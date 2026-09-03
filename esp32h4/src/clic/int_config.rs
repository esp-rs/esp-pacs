#[doc = "Register `INT_CONFIG` reader"]
pub type R = crate::R<INT_CONFIG_SPEC>;
#[doc = "Register `INT_CONFIG` writer"]
pub type W = crate::W<INT_CONFIG_SPEC>;
#[doc = "Field `NVBITS` reader - Hardware vector interrupt implementation flag bit."]
pub type NVBITS_R = crate::BitReader;
#[doc = "Field `NVBITS` writer - Hardware vector interrupt implementation flag bit."]
pub type NVBITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NLBITS` reader - Interrupt priority effective digits, the maximum value is 8."]
pub type NLBITS_R = crate::FieldReader;
#[doc = "Field `NLBITS` writer - Interrupt priority effective digits, the maximum value is 8."]
pub type NLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NMBITS` reader - The effective number of bits in the privileged state."]
pub type NMBITS_R = crate::FieldReader;
#[doc = "Field `NMBITS` writer - The effective number of bits in the privileged state."]
pub type NMBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Hardware vector interrupt implementation flag bit."]
    #[inline(always)]
    pub fn nvbits(&self) -> NVBITS_R {
        NVBITS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn nlbits(&self) -> NLBITS_R {
        NLBITS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - The effective number of bits in the privileged state."]
    #[inline(always)]
    pub fn nmbits(&self) -> NMBITS_R {
        NMBITS_R::new(((self.bits >> 5) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CONFIG")
            .field("nmbits", &self.nmbits())
            .field("nlbits", &self.nlbits())
            .field("nvbits", &self.nvbits())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Hardware vector interrupt implementation flag bit."]
    #[inline(always)]
    pub fn nvbits(&mut self) -> NVBITS_W<'_, INT_CONFIG_SPEC> {
        NVBITS_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Interrupt priority effective digits, the maximum value is 8."]
    #[inline(always)]
    pub fn nlbits(&mut self) -> NLBITS_W<'_, INT_CONFIG_SPEC> {
        NLBITS_W::new(self, 1)
    }
    #[doc = "Bits 5:6 - The effective number of bits in the privileged state."]
    #[inline(always)]
    pub fn nmbits(&mut self) -> NMBITS_W<'_, INT_CONFIG_SPEC> {
        NMBITS_W::new(self, 5)
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
