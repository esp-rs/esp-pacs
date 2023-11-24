#[doc = "Register `COMD5` reader"]
pub type R = crate::R<COMD5_SPEC>;
#[doc = "Register `COMD5` writer"]
pub type W = crate::W<COMD5_SPEC>;
#[doc = "Field `COMMAND5` reader - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND5_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND5` writer - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND5_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND5_DONE` reader - When command 5 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND5_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND5_DONE` writer - When command 5 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND5_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command5(&self) -> COMMAND5_R {
        COMMAND5_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 5 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command5_done(&self) -> COMMAND5_DONE_R {
        COMMAND5_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD5")
            .field("command5", &format_args!("{}", self.command5().bits()))
            .field(
                "command5_done",
                &format_args!("{}", self.command5_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn command5(&mut self) -> COMMAND5_W<COMD5_SPEC> {
        COMMAND5_W::new(self, 0)
    }
    #[doc = "Bit 31 - When command 5 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn command5_done(&mut self) -> COMMAND5_DONE_W<COMD5_SPEC> {
        COMMAND5_DONE_W::new(self, 31)
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
#[doc = "I2C command register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD5_SPEC;
impl crate::RegisterSpec for COMD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd5::R`](R) reader structure"]
impl crate::Readable for COMD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd5::W`](W) writer structure"]
impl crate::Writable for COMD5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD5 to value 0"]
impl crate::Resettable for COMD5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
