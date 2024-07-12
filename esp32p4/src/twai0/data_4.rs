#[doc = "Register `DATA_4` reader"]
pub type R = crate::R<DATA_4_SPEC>;
#[doc = "Register `DATA_4` writer"]
pub type W = crate::W<DATA_4_SPEC>;
#[doc = "Field `DATA_4` reader - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
pub type DATA_4_R = crate::FieldReader;
#[doc = "Field `DATA_4` writer - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
pub type DATA_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
    #[inline(always)]
    pub fn data_4(&self) -> DATA_4_R {
        DATA_4_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_4")
            .field("data_4", &self.data_4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
    #[inline(always)]
    #[must_use]
    pub fn data_4(&mut self) -> DATA_4_W<DATA_4_SPEC> {
        DATA_4_W::new(self, 0)
    }
}
#[doc = "Data register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_4_SPEC;
impl crate::RegisterSpec for DATA_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_4::R`](R) reader structure"]
impl crate::Readable for DATA_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_4::W`](W) writer structure"]
impl crate::Writable for DATA_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_4 to value 0"]
impl crate::Resettable for DATA_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
