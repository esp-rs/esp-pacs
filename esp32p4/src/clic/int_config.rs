#[doc = "Register `INT_CONFIG` reader"]
pub type R = crate::R<INT_CONFIG_SPEC>;
#[doc = "Register `INT_CONFIG` writer"]
pub type W = crate::W<INT_CONFIG_SPEC>;
#[doc = "Field `NVBITS` reader - Hardware vector interrupt implementation flag."]
pub type NVBITS_R = crate::BitReader;
#[doc = "Field `MNLBITS` reader - Interrupt priority effective digits, maximum value is 8."]
pub type MNLBITS_R = crate::FieldReader;
#[doc = "Field `MNLBITS` writer - Interrupt priority effective digits, maximum value is 8."]
pub type MNLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NMBITS` reader - Effective number of privilege-state bits."]
pub type NMBITS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Hardware vector interrupt implementation flag."]
    #[inline(always)]
    pub fn nvbits(&self) -> NVBITS_R {
        NVBITS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Interrupt priority effective digits, maximum value is 8."]
    #[inline(always)]
    pub fn mnlbits(&self) -> MNLBITS_R {
        MNLBITS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Effective number of privilege-state bits."]
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
            .field("mnlbits", &self.mnlbits())
            .field("nvbits", &self.nvbits())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:4 - Interrupt priority effective digits, maximum value is 8."]
    #[inline(always)]
    pub fn mnlbits(&mut self) -> MNLBITS_W<'_, INT_CONFIG_SPEC> {
        MNLBITS_W::new(self, 1)
    }
}
#[doc = "CLIC interrupt configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
