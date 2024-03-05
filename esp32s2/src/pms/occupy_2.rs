#[doc = "Register `OCCUPY_2` reader"]
pub type R = crate::R<OCCUPY_2_SPEC>;
#[doc = "Register `OCCUPY_2` writer"]
pub type W = crate::W<OCCUPY_2_SPEC>;
#[doc = "Field `OCCUPY_MAC_DUMP` reader - Configure whether SRAM Block 18-21 is used as mac dump."]
pub type OCCUPY_MAC_DUMP_R = crate::FieldReader;
#[doc = "Field `OCCUPY_MAC_DUMP` writer - Configure whether SRAM Block 18-21 is used as mac dump."]
pub type OCCUPY_MAC_DUMP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 18-21 is used as mac dump."]
    #[inline(always)]
    pub fn occupy_mac_dump(&self) -> OCCUPY_MAC_DUMP_R {
        OCCUPY_MAC_DUMP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_2")
            .field(
                "occupy_mac_dump",
                &format_args!("{}", self.occupy_mac_dump().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OCCUPY_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 18-21 is used as mac dump."]
    #[inline(always)]
    #[must_use]
    pub fn occupy_mac_dump(&mut self) -> OCCUPY_MAC_DUMP_W<OCCUPY_2_SPEC> {
        OCCUPY_MAC_DUMP_W::new(self, 0)
    }
}
#[doc = "Occupy permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCCUPY_2_SPEC;
impl crate::RegisterSpec for OCCUPY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`occupy_2::R`](R) reader structure"]
impl crate::Readable for OCCUPY_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`occupy_2::W`](W) writer structure"]
impl crate::Writable for OCCUPY_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCCUPY_2 to value 0"]
impl crate::Resettable for OCCUPY_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
