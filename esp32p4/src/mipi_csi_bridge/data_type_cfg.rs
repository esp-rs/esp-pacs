#[doc = "Register `DATA_TYPE_CFG` reader"]
pub type R = crate::R<DATA_TYPE_CFG_SPEC>;
#[doc = "Register `DATA_TYPE_CFG` writer"]
pub type W = crate::W<DATA_TYPE_CFG_SPEC>;
#[doc = "Field `DATA_TYPE_MIN` reader - the min value of data type used for pixel filter."]
pub type DATA_TYPE_MIN_R = crate::FieldReader;
#[doc = "Field `DATA_TYPE_MIN` writer - the min value of data type used for pixel filter."]
pub type DATA_TYPE_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATA_TYPE_MAX` reader - the max value of data type used for pixel filter."]
pub type DATA_TYPE_MAX_R = crate::FieldReader;
#[doc = "Field `DATA_TYPE_MAX` writer - the max value of data type used for pixel filter."]
pub type DATA_TYPE_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - the min value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_min(&self) -> DATA_TYPE_MIN_R {
        DATA_TYPE_MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - the max value of data type used for pixel filter."]
    #[inline(always)]
    pub fn data_type_max(&self) -> DATA_TYPE_MAX_R {
        DATA_TYPE_MAX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_TYPE_CFG")
            .field("data_type_min", &self.data_type_min())
            .field("data_type_max", &self.data_type_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - the min value of data type used for pixel filter."]
    #[inline(always)]
    #[must_use]
    pub fn data_type_min(&mut self) -> DATA_TYPE_MIN_W<DATA_TYPE_CFG_SPEC> {
        DATA_TYPE_MIN_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - the max value of data type used for pixel filter."]
    #[inline(always)]
    #[must_use]
    pub fn data_type_max(&mut self) -> DATA_TYPE_MAX_W<DATA_TYPE_CFG_SPEC> {
        DATA_TYPE_MAX_W::new(self, 8)
    }
}
#[doc = "pixel data type configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_type_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_type_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_TYPE_CFG_SPEC;
impl crate::RegisterSpec for DATA_TYPE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_type_cfg::R`](R) reader structure"]
impl crate::Readable for DATA_TYPE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_type_cfg::W`](W) writer structure"]
impl crate::Writable for DATA_TYPE_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_TYPE_CFG to value 0x2f18"]
impl crate::Resettable for DATA_TYPE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x2f18;
}
