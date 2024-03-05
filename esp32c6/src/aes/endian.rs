#[doc = "Register `ENDIAN` reader"]
pub type R = crate::R<ENDIAN_SPEC>;
#[doc = "Register `ENDIAN` writer"]
pub type W = crate::W<ENDIAN_SPEC>;
#[doc = "Field `ENDIAN` reader - endian. \\[1:0\\] key endian, \\[3:2\\] text_in endian or in_stream endian, \\[5:4\\] text_out endian or out_stream endian"]
pub type ENDIAN_R = crate::FieldReader;
#[doc = "Field `ENDIAN` writer - endian. \\[1:0\\] key endian, \\[3:2\\] text_in endian or in_stream endian, \\[5:4\\] text_out endian or out_stream endian"]
pub type ENDIAN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - endian. \\[1:0\\] key endian, \\[3:2\\] text_in endian or in_stream endian, \\[5:4\\] text_out endian or out_stream endian"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDIAN")
            .field("endian", &format_args!("{}", self.endian().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENDIAN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - endian. \\[1:0\\] key endian, \\[3:2\\] text_in endian or in_stream endian, \\[5:4\\] text_out endian or out_stream endian"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<ENDIAN_SPEC> {
        ENDIAN_W::new(self, 0)
    }
}
#[doc = "AES Endian configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDIAN_SPEC;
impl crate::RegisterSpec for ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endian::R`](R) reader structure"]
impl crate::Readable for ENDIAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endian::W`](W) writer structure"]
impl crate::Writable for ENDIAN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDIAN to value 0"]
impl crate::Resettable for ENDIAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
