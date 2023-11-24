#[doc = "Register `COMD0` reader"]
pub type R = crate::R<COMD0_SPEC>;
#[doc = "Register `COMD0` writer"]
pub type W = crate::W<COMD0_SPEC>;
#[doc = "Field `COMMAND0` reader - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND0_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND0` writer - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND0_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND0_DONE` reader - When command 0 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND0_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND0_DONE` writer - When command 0 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND0_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command0(&self) -> COMMAND0_R {
        COMMAND0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 0 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command0_done(&self) -> COMMAND0_DONE_R {
        COMMAND0_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD0")
            .field("command0", &format_args!("{}", self.command0().bits()))
            .field(
                "command0_done",
                &format_args!("{}", self.command0_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn command0(&mut self) -> COMMAND0_W<COMD0_SPEC> {
        COMMAND0_W::new(self, 0)
    }
    #[doc = "Bit 31 - When command 0 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn command0_done(&mut self) -> COMMAND0_DONE_W<COMD0_SPEC> {
        COMMAND0_DONE_W::new(self, 31)
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
#[doc = "I2C command register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD0_SPEC;
impl crate::RegisterSpec for COMD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd0::R`](R) reader structure"]
impl crate::Readable for COMD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd0::W`](W) writer structure"]
impl crate::Writable for COMD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD0 to value 0"]
impl crate::Resettable for COMD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
