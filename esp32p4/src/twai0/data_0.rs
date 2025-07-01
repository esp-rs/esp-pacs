#[doc = "Register `DATA_0` reader"]
pub type R = crate::R<DATA_0_SPEC>;
#[doc = "Register `DATA_0` writer"]
pub type W = crate::W<DATA_0_SPEC>;
#[doc = "Field `DATA_0` reader - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0."]
pub type DATA_0_R = crate::FieldReader;
#[doc = "Field `DATA_0` writer - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0."]
pub type DATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0."]
    #[inline(always)]
    pub fn data_0(&self) -> DATA_0_R {
        DATA_0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_0")
            .field("data_0", &self.data_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 0 and when software initiate read operation, it is rx data register 0."]
    #[inline(always)]
    pub fn data_0(&mut self) -> DATA_0_W<DATA_0_SPEC> {
        DATA_0_W::new(self, 0)
    }
}
#[doc = "Data register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_0_SPEC;
impl crate::RegisterSpec for DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_0::R`](R) reader structure"]
impl crate::Readable for DATA_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_0::W`](W) writer structure"]
impl crate::Writable for DATA_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_0 to value 0"]
impl crate::Resettable for DATA_0_SPEC {}
