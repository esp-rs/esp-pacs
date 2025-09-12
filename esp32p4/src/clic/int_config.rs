#[doc = "Register `INT_CONFIG` reader"]
pub type R = crate::R<INT_CONFIG_SPEC>;
#[doc = "Register `INT_CONFIG` writer"]
pub type W = crate::W<INT_CONFIG_SPEC>;
#[doc = "Field `INT_CONFIG_NVBITS` reader - Number of vector bits for interrupt"]
pub type INT_CONFIG_NVBITS_R = crate::BitReader;
#[doc = "Field `INT_CONFIG_NVBITS` writer - Number of vector bits for interrupt"]
pub type INT_CONFIG_NVBITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CONFIG_NLBITS` reader - Number of level bits for interrupt"]
pub type INT_CONFIG_NLBITS_R = crate::FieldReader;
#[doc = "Field `INT_CONFIG_NLBITS` writer - Number of level bits for interrupt"]
pub type INT_CONFIG_NLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INT_CONFIG_NMBITS` reader - Number of mode bits for interrupt"]
pub type INT_CONFIG_NMBITS_R = crate::FieldReader;
#[doc = "Field `INT_CONFIG_NMBITS` writer - Number of mode bits for interrupt"]
pub type INT_CONFIG_NMBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Number of vector bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nvbits(&self) -> INT_CONFIG_NVBITS_R {
        INT_CONFIG_NVBITS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Number of level bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nlbits(&self) -> INT_CONFIG_NLBITS_R {
        INT_CONFIG_NLBITS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Number of mode bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nmbits(&self) -> INT_CONFIG_NMBITS_R {
        INT_CONFIG_NMBITS_R::new(((self.bits >> 5) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CONFIG")
            .field("int_config_nmbits", &self.int_config_nmbits())
            .field("int_config_nlbits", &self.int_config_nlbits())
            .field("int_config_nvbits", &self.int_config_nvbits())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Number of vector bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nvbits(&mut self) -> INT_CONFIG_NVBITS_W<'_, INT_CONFIG_SPEC> {
        INT_CONFIG_NVBITS_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Number of level bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nlbits(&mut self) -> INT_CONFIG_NLBITS_W<'_, INT_CONFIG_SPEC> {
        INT_CONFIG_NLBITS_W::new(self, 1)
    }
    #[doc = "Bits 5:6 - Number of mode bits for interrupt"]
    #[inline(always)]
    pub fn int_config_nmbits(&mut self) -> INT_CONFIG_NMBITS_W<'_, INT_CONFIG_SPEC> {
        INT_CONFIG_NMBITS_W::new(self, 5)
    }
}
#[doc = "Interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
