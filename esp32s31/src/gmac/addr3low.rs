#[doc = "Register `ADDR3LOW` reader"]
pub type R = crate::R<ADDR3LOW_SPEC>;
#[doc = "Register `ADDR3LOW` writer"]
pub type W = crate::W<ADDR3LOW_SPEC>;
#[doc = "Field `ADDRLO_3` reader - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_3_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_3` writer - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_3(&self) -> ADDRLO_3_R {
        ADDRLO_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR3LOW")
            .field("addrlo_3", &self.addrlo_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_3(&mut self) -> ADDRLO_3_W<'_, ADDR3LOW_SPEC> {
        ADDRLO_3_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`addr3low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR3LOW_SPEC;
impl crate::RegisterSpec for ADDR3LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr3low::R`](R) reader structure"]
impl crate::Readable for ADDR3LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr3low::W`](W) writer structure"]
impl crate::Writable for ADDR3LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR3LOW to value 0xffff_ffff"]
impl crate::Resettable for ADDR3LOW_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
