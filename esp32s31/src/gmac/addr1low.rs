#[doc = "Register `ADDR1LOW` reader"]
pub type R = crate::R<ADDR1LOW_SPEC>;
#[doc = "Register `ADDR1LOW` writer"]
pub type W = crate::W<ADDR1LOW_SPEC>;
#[doc = "Field `ADDRLO_1` reader - MAC Address1 \\[31:0\\] This field contains the lower 32 bits of the second 6byte MAC address The content of this field is undefined until loaded by the Application after the initialization process"]
pub type ADDRLO_1_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_1` writer - MAC Address1 \\[31:0\\] This field contains the lower 32 bits of the second 6byte MAC address The content of this field is undefined until loaded by the Application after the initialization process"]
pub type ADDRLO_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\] This field contains the lower 32 bits of the second 6byte MAC address The content of this field is undefined until loaded by the Application after the initialization process"]
    #[inline(always)]
    pub fn addrlo_1(&self) -> ADDRLO_1_R {
        ADDRLO_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR1LOW")
            .field("addrlo_1", &self.addrlo_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\] This field contains the lower 32 bits of the second 6byte MAC address The content of this field is undefined until loaded by the Application after the initialization process"]
    #[inline(always)]
    pub fn addrlo_1(&mut self) -> ADDRLO_1_W<'_, ADDR1LOW_SPEC> {
        ADDRLO_1_W::new(self, 0)
    }
}
#[doc = "Contains the lower 32 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR1LOW_SPEC;
impl crate::RegisterSpec for ADDR1LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1low::R`](R) reader structure"]
impl crate::Readable for ADDR1LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr1low::W`](W) writer structure"]
impl crate::Writable for ADDR1LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR1LOW to value 0xffff_ffff"]
impl crate::Resettable for ADDR1LOW_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
