#[doc = "Register `CACHE_DATAARRAY_CONNECT_1` reader"]
pub type R = crate::R<CACHE_DATAARRAY_CONNECT_1_SPEC>;
#[doc = "Register `CACHE_DATAARRAY_CONNECT_1` writer"]
pub type W = crate::W<CACHE_DATAARRAY_CONNECT_1_SPEC>;
#[doc = "Field `CACHE_DATAARRAY_CONNECT_FLATTEN` reader - Cache data array connection configuration."]
pub type CACHE_DATAARRAY_CONNECT_FLATTEN_R = crate::FieldReader;
#[doc = "Field `CACHE_DATAARRAY_CONNECT_FLATTEN` writer - Cache data array connection configuration."]
pub type CACHE_DATAARRAY_CONNECT_FLATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cache data array connection configuration."]
    #[inline(always)]
    pub fn cache_dataarray_connect_flatten(&self) -> CACHE_DATAARRAY_CONNECT_FLATTEN_R {
        CACHE_DATAARRAY_CONNECT_FLATTEN_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATAARRAY_CONNECT_1")
            .field(
                "cache_dataarray_connect_flatten",
                &self.cache_dataarray_connect_flatten(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Cache data array connection configuration."]
    #[inline(always)]
    pub fn cache_dataarray_connect_flatten(
        &mut self,
    ) -> CACHE_DATAARRAY_CONNECT_FLATTEN_W<CACHE_DATAARRAY_CONNECT_1_SPEC> {
        CACHE_DATAARRAY_CONNECT_FLATTEN_W::new(self, 0)
    }
}
#[doc = "Cache data array configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_dataarray_connect_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_dataarray_connect_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_DATAARRAY_CONNECT_1_SPEC;
impl crate::RegisterSpec for CACHE_DATAARRAY_CONNECT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_dataarray_connect_1::R`](R) reader structure"]
impl crate::Readable for CACHE_DATAARRAY_CONNECT_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_dataarray_connect_1::W`](W) writer structure"]
impl crate::Writable for CACHE_DATAARRAY_CONNECT_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_DATAARRAY_CONNECT_1 to value 0xff"]
impl crate::Resettable for CACHE_DATAARRAY_CONNECT_1_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
