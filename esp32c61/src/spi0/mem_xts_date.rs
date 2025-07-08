#[doc = "Register `MEM_XTS_DATE` reader"]
pub type R = crate::R<MEM_XTS_DATE_SPEC>;
#[doc = "Register `MEM_XTS_DATE` writer"]
pub type W = crate::W<MEM_XTS_DATE_SPEC>;
#[doc = "Field `XTS_DATE` reader - This bits stores the last modified-time of manual encryption feature."]
pub type XTS_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `XTS_DATE` writer - This bits stores the last modified-time of manual encryption feature."]
pub type XTS_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - This bits stores the last modified-time of manual encryption feature."]
    #[inline(always)]
    pub fn xts_date(&self) -> XTS_DATE_R {
        XTS_DATE_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_XTS_DATE")
            .field("xts_date", &self.xts_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - This bits stores the last modified-time of manual encryption feature."]
    #[inline(always)]
    pub fn xts_date(&mut self) -> XTS_DATE_W<MEM_XTS_DATE_SPEC> {
        XTS_DATE_W::new(self, 0)
    }
}
#[doc = "Manual Encryption version register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_XTS_DATE_SPEC;
impl crate::RegisterSpec for MEM_XTS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xts_date::R`](R) reader structure"]
impl crate::Readable for MEM_XTS_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_xts_date::W`](W) writer structure"]
impl crate::Writable for MEM_XTS_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_XTS_DATE to value 0x2021_0907"]
impl crate::Resettable for MEM_XTS_DATE_SPEC {
    const RESET_VALUE: u32 = 0x2021_0907;
}
