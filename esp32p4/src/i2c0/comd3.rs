#[doc = "Register `COMD3` reader"]
pub type R = crate::R<COMD3_SPEC>;
#[doc = "Register `COMD3` writer"]
pub type W = crate::W<COMD3_SPEC>;
#[doc = "Field `COMMAND3` reader - Configures command 3. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND3_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND3` writer - Configures command 3. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND3_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND3_DONE` reader - Represents whether command 3 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND3_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND3_DONE` writer - Represents whether command 3 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND3_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 3. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 3 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD3")
            .field("command3", &format_args!("{}", self.command3().bits()))
            .field(
                "command3_done",
                &format_args!("{}", self.command3_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 3. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn command3(&mut self) -> COMMAND3_W<COMD3_SPEC> {
        COMMAND3_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 3 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    #[must_use]
    pub fn command3_done(&mut self) -> COMMAND3_DONE_W<COMD3_SPEC> {
        COMMAND3_DONE_W::new(self, 31)
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
#[doc = "I2C command register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD3_SPEC;
impl crate::RegisterSpec for COMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd3::R`](R) reader structure"]
impl crate::Readable for COMD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd3::W`](W) writer structure"]
impl crate::Writable for COMD3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD3 to value 0"]
impl crate::Resettable for COMD3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
