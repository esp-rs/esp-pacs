#[doc = "Register `UART3_INT_MAP` reader"]
pub type R = crate::R<UART3_INT_MAP_SPEC>;
#[doc = "Register `UART3_INT_MAP` writer"]
pub type W = crate::W<UART3_INT_MAP_SPEC>;
#[doc = "Field `UART3_INT_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type UART3_INT_MAP_R = crate::FieldReader;
#[doc = "Field `UART3_INT_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type UART3_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UART3_INT_SRC_PASS_IN_SEC` reader - NA"]
pub type UART3_INT_SRC_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `UART3_INT_SRC_PASS_IN_SEC` writer - NA"]
pub type UART3_INT_SRC_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_INT_SRC_IN_SEC_FLAG` reader - NA"]
pub type UART3_INT_SRC_IN_SEC_FLAG_R = crate::BitReader;
#[doc = "Field `UART3_INT_SRC_IN_SEC_FLAG` writer - NA"]
pub type UART3_INT_SRC_IN_SEC_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn uart3_int_map(&self) -> UART3_INT_MAP_R {
        UART3_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn uart3_int_src_pass_in_sec(&self) -> UART3_INT_SRC_PASS_IN_SEC_R {
        UART3_INT_SRC_PASS_IN_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn uart3_int_src_in_sec_flag(&self) -> UART3_INT_SRC_IN_SEC_FLAG_R {
        UART3_INT_SRC_IN_SEC_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART3_INT_MAP")
            .field("uart3_int_map", &self.uart3_int_map())
            .field(
                "uart3_int_src_pass_in_sec",
                &self.uart3_int_src_pass_in_sec(),
            )
            .field(
                "uart3_int_src_in_sec_flag",
                &self.uart3_int_src_in_sec_flag(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn uart3_int_map(&mut self) -> UART3_INT_MAP_W<'_, UART3_INT_MAP_SPEC> {
        UART3_INT_MAP_W::new(self, 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn uart3_int_src_pass_in_sec(
        &mut self,
    ) -> UART3_INT_SRC_PASS_IN_SEC_W<'_, UART3_INT_MAP_SPEC> {
        UART3_INT_SRC_PASS_IN_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn uart3_int_src_in_sec_flag(
        &mut self,
    ) -> UART3_INT_SRC_IN_SEC_FLAG_W<'_, UART3_INT_MAP_SPEC> {
        UART3_INT_SRC_IN_SEC_FLAG_W::new(self, 7)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART3_INT_MAP_SPEC;
impl crate::RegisterSpec for UART3_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_int_map::R`](R) reader structure"]
impl crate::Readable for UART3_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart3_int_map::W`](W) writer structure"]
impl crate::Writable for UART3_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART3_INT_MAP to value 0"]
impl crate::Resettable for UART3_INT_MAP_SPEC {}
