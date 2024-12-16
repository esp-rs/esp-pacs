#[doc = "Register `DATA_5` reader"]
pub type R = crate::R<DATA_5_SPEC>;
#[doc = "Register `DATA_5` writer"]
pub type W = crate::W<DATA_5_SPEC>;
#[doc = "Field `DATA_5` reader - In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5."]
pub type DATA_5_R = crate::FieldReader;
#[doc = "Field `DATA_5` writer - In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5."]
pub type DATA_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5."]
    #[inline(always)]
    pub fn data_5(&self) -> DATA_5_R {
        DATA_5_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_5")
            .field("data_5", &self.data_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 1 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 5 and when software initiate read operation, it is rx data register 5."]
    #[inline(always)]
    pub fn data_5(&mut self) -> DATA_5_W<DATA_5_SPEC> {
        DATA_5_W::new(self, 0)
    }
}
#[doc = "Data register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_5_SPEC;
impl crate::RegisterSpec for DATA_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_5::R`](R) reader structure"]
impl crate::Readable for DATA_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_5::W`](W) writer structure"]
impl crate::Writable for DATA_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_5 to value 0"]
impl crate::Resettable for DATA_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
