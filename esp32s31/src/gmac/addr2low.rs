#[doc = "Register `ADDR2LOW` reader"]
pub type R = crate::R<ADDR2LOW_SPEC>;
#[doc = "Register `ADDR2LOW` writer"]
pub type W = crate::W<ADDR2LOW_SPEC>;
#[doc = "Field `ADDRLO_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_2_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_2(&self) -> ADDRLO_2_R {
        ADDRLO_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR2LOW")
            .field("addrlo_2", &self.addrlo_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_2(&mut self) -> ADDRLO_2_W<'_, ADDR2LOW_SPEC> {
        ADDRLO_2_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR2LOW_SPEC;
impl crate::RegisterSpec for ADDR2LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr2low::R`](R) reader structure"]
impl crate::Readable for ADDR2LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr2low::W`](W) writer structure"]
impl crate::Writable for ADDR2LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR2LOW to value 0xffff_ffff"]
impl crate::Resettable for ADDR2LOW_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
