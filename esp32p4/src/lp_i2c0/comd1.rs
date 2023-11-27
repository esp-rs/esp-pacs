#[doc = "Register `COMD1` reader"]
pub type R = crate::R<COMD1_SPEC>;
#[doc = "Register `COMD1` writer"]
pub type W = crate::W<COMD1_SPEC>;
#[doc = "Field `COMMAND1` reader - Configures command 1. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND1_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND1` writer - Configures command 1. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND1_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND1_DONE` reader - Represents whether command 1 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND1_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND1_DONE` writer - Represents whether command 1 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND1_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 1. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 1 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD1")
            .field("command1", &format_args!("{}", self.command1().bits()))
            .field(
                "command1_done",
                &format_args!("{}", self.command1_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 1. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn command1(&mut self) -> COMMAND1_W<COMD1_SPEC> {
        COMMAND1_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 1 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    #[must_use]
    pub fn command1_done(&mut self) -> COMMAND1_DONE_W<COMD1_SPEC> {
        COMMAND1_DONE_W::new(self, 31)
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
#[doc = "I2C command register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD1_SPEC;
impl crate::RegisterSpec for COMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd1::R`](R) reader structure"]
impl crate::Readable for COMD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd1::W`](W) writer structure"]
impl crate::Writable for COMD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD1 to value 0"]
impl crate::Resettable for COMD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
