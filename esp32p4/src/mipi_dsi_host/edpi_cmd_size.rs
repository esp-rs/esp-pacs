#[doc = "Register `EDPI_CMD_SIZE` reader"]
pub type R = crate::R<EDPI_CMD_SIZE_SPEC>;
#[doc = "Register `EDPI_CMD_SIZE` writer"]
pub type W = crate::W<EDPI_CMD_SIZE_SPEC>;
#[doc = "Field `EDPI_ALLOWED_CMD_SIZE` reader - NA"]
pub type EDPI_ALLOWED_CMD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `EDPI_ALLOWED_CMD_SIZE` writer - NA"]
pub type EDPI_ALLOWED_CMD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn edpi_allowed_cmd_size(&self) -> EDPI_ALLOWED_CMD_SIZE_R {
        EDPI_ALLOWED_CMD_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDPI_CMD_SIZE")
            .field(
                "edpi_allowed_cmd_size",
                &format_args!("{}", self.edpi_allowed_cmd_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDPI_CMD_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn edpi_allowed_cmd_size(&mut self) -> EDPI_ALLOWED_CMD_SIZE_W<EDPI_CMD_SIZE_SPEC> {
        EDPI_ALLOWED_CMD_SIZE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edpi_cmd_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_cmd_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDPI_CMD_SIZE_SPEC;
impl crate::RegisterSpec for EDPI_CMD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edpi_cmd_size::R`](R) reader structure"]
impl crate::Readable for EDPI_CMD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edpi_cmd_size::W`](W) writer structure"]
impl crate::Writable for EDPI_CMD_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDPI_CMD_SIZE to value 0"]
impl crate::Resettable for EDPI_CMD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
