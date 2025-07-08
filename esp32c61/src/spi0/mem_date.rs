#[doc = "Register `MEM_DATE` reader"]
pub type R = crate::R<MEM_DATE_SPEC>;
#[doc = "Register `MEM_DATE` writer"]
pub type W = crate::W<MEM_DATE_SPEC>;
#[doc = "Field `MEM_DATE` reader - SPI0 register version."]
pub type MEM_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_DATE` writer - SPI0 register version."]
pub type MEM_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - SPI0 register version."]
    #[inline(always)]
    pub fn mem_date(&self) -> MEM_DATE_R {
        MEM_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_DATE")
            .field("mem_date", &self.mem_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - SPI0 register version."]
    #[inline(always)]
    pub fn mem_date(&mut self) -> MEM_DATE_W<MEM_DATE_SPEC> {
        MEM_DATE_W::new(self, 0)
    }
}
#[doc = "SPI0 version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_DATE_SPEC;
impl crate::RegisterSpec for MEM_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_date::R`](R) reader structure"]
impl crate::Readable for MEM_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_date::W`](W) writer structure"]
impl crate::Writable for MEM_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_DATE to value 0x0241_2300"]
impl crate::Resettable for MEM_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0241_2300;
}
