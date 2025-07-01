#[doc = "Register `VER_DATE` reader"]
pub type R = crate::R<VER_DATE_SPEC>;
#[doc = "Register `VER_DATE` writer"]
pub type W = crate::W<VER_DATE_SPEC>;
#[doc = "Field `VER_DATA` reader - csv version"]
pub type VER_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `VER_DATA` writer - csv version"]
pub type VER_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - csv version"]
    #[inline(always)]
    pub fn ver_data(&self) -> VER_DATA_R {
        VER_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_DATE")
            .field("ver_data", &self.ver_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - csv version"]
    #[inline(always)]
    pub fn ver_data(&mut self) -> VER_DATA_W<VER_DATE_SPEC> {
        VER_DATA_W::new(self, 0)
    }
}
#[doc = "version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_DATE_SPEC;
impl crate::RegisterSpec for VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_date::R`](R) reader structure"]
impl crate::Readable for VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_date::W`](W) writer structure"]
impl crate::Writable for VER_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VER_DATE to value 0x2021_0608"]
impl crate::Resettable for VER_DATE_SPEC {
    const RESET_VALUE: u32 = 0x2021_0608;
}
