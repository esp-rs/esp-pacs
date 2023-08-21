#[doc = "Register `I2C_COMD5` reader"]
pub type R = crate::R<I2C_COMD5_SPEC>;
#[doc = "Register `I2C_COMD5` writer"]
pub type W = crate::W<I2C_COMD5_SPEC>;
#[doc = "Field `I2C_COMMAND5` reader - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type I2C_COMMAND5_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_COMMAND5` writer - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type I2C_COMMAND5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `I2C_COMMAND5_DONE` reader - When command 5 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND5_DONE_R = crate::BitReader;
#[doc = "Field `I2C_COMMAND5_DONE` writer - When command 5 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND5_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn i2c_command5(&self) -> I2C_COMMAND5_R {
        I2C_COMMAND5_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 5 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command5_done(&self) -> I2C_COMMAND5_DONE_R {
        I2C_COMMAND5_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_COMD5")
            .field(
                "i2c_command5",
                &format_args!("{}", self.i2c_command5().bits()),
            )
            .field(
                "i2c_command5_done",
                &format_args!("{}", self.i2c_command5_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_COMD5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 5. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_command5(&mut self) -> I2C_COMMAND5_W<I2C_COMD5_SPEC, 0> {
        I2C_COMMAND5_W::new(self)
    }
    #[doc = "Bit 31 - When command 5 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_command5_done(&mut self) -> I2C_COMMAND5_DONE_W<I2C_COMD5_SPEC, 31> {
        I2C_COMMAND5_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C command register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_comd5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_comd5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_COMD5_SPEC;
impl crate::RegisterSpec for I2C_COMD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_comd5::R`](R) reader structure"]
impl crate::Readable for I2C_COMD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_comd5::W`](W) writer structure"]
impl crate::Writable for I2C_COMD5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_COMD5 to value 0"]
impl crate::Resettable for I2C_COMD5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
