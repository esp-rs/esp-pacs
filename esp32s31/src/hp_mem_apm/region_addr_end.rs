#[doc = "Register `REGION%s_ADDR_END` reader"]
pub type R = crate::R<REGION_ADDR_END_SPEC>;
#[doc = "Register `REGION%s_ADDR_END` writer"]
pub type W = crate::W<REGION_ADDR_END_SPEC>;
#[doc = "Field `L` reader - Low 12 bit, end address of region %s."]
pub type L_R = crate::FieldReader;
#[doc = "Field `REGION_ADDR_END` reader - Configures end address of region %s."]
pub type REGION_ADDR_END_R = crate::FieldReader<u16>;
#[doc = "Field `REGION_ADDR_END` writer - Configures end address of region %s."]
pub type REGION_ADDR_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `H` reader - High 13 bit, end address of region %s."]
pub type H_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:6 - Low 12 bit, end address of region %s."]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:18 - Configures end address of region %s."]
    #[inline(always)]
    pub fn region_addr_end(&self) -> REGION_ADDR_END_R {
        REGION_ADDR_END_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bits 19:31 - High 13 bit, end address of region %s."]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_ADDR_END")
            .field("l", &self.l())
            .field("region_addr_end", &self.region_addr_end())
            .field("h", &self.h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:18 - Configures end address of region %s."]
    #[inline(always)]
    pub fn region_addr_end(&mut self) -> REGION_ADDR_END_W<'_, REGION_ADDR_END_SPEC> {
        REGION_ADDR_END_W::new(self, 7)
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_addr_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_addr_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_ADDR_END_SPEC;
impl crate::RegisterSpec for REGION_ADDR_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_addr_end::R`](R) reader structure"]
impl crate::Readable for REGION_ADDR_END_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region_addr_end::W`](W) writer structure"]
impl crate::Writable for REGION_ADDR_END_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION%s_ADDR_END to value 0x2f07_ffff"]
impl crate::Resettable for REGION_ADDR_END_SPEC {
    const RESET_VALUE: u32 = 0x2f07_ffff;
}
