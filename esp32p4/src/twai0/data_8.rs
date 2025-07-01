#[doc = "Register `DATA_8` reader"]
pub type R = crate::R<DATA_8_SPEC>;
#[doc = "Register `DATA_8` writer"]
pub type W = crate::W<DATA_8_SPEC>;
#[doc = "Field `DATA_8` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
pub type DATA_8_R = crate::FieldReader;
#[doc = "Field `DATA_8` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
pub type DATA_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
    #[inline(always)]
    pub fn data_8(&self) -> DATA_8_R {
        DATA_8_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_8")
            .field("data_8", &self.data_8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 8 and when software initiate read operation, it is rx data register 8."]
    #[inline(always)]
    pub fn data_8(&mut self) -> DATA_8_W<DATA_8_SPEC> {
        DATA_8_W::new(self, 0)
    }
}
#[doc = "Data register 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_8_SPEC;
impl crate::RegisterSpec for DATA_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_8::R`](R) reader structure"]
impl crate::Readable for DATA_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_8::W`](W) writer structure"]
impl crate::Writable for DATA_8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_8 to value 0"]
impl crate::Resettable for DATA_8_SPEC {}
