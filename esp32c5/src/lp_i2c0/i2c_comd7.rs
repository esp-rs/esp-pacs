#[doc = "Register `I2C_COMD7` reader"]
pub type R = crate::R<I2C_COMD7_SPEC>;
#[doc = "Register `I2C_COMD7` writer"]
pub type W = crate::W<I2C_COMD7_SPEC>;
#[doc = "Field `I2C_COMMAND7` reader - This is the content of command 7. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type I2C_COMMAND7_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_COMMAND7` writer - This is the content of command 7. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type I2C_COMMAND7_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `I2C_COMMAND7_DONE` reader - When command 7 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND7_DONE_R = crate::BitReader;
#[doc = "Field `I2C_COMMAND7_DONE` writer - When command 7 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND7_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 7. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn i2c_command7(&self) -> I2C_COMMAND7_R {
        I2C_COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 7 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command7_done(&self) -> I2C_COMMAND7_DONE_R {
        I2C_COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_COMD7")
            .field("i2c_command7", &self.i2c_command7())
            .field("i2c_command7_done", &self.i2c_command7_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 7. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn i2c_command7(&mut self) -> I2C_COMMAND7_W<I2C_COMD7_SPEC> {
        I2C_COMMAND7_W::new(self, 0)
    }
    #[doc = "Bit 31 - When command 7 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command7_done(&mut self) -> I2C_COMMAND7_DONE_W<I2C_COMD7_SPEC> {
        I2C_COMMAND7_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_comd7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_comd7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_COMD7_SPEC;
impl crate::RegisterSpec for I2C_COMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_comd7::R`](R) reader structure"]
impl crate::Readable for I2C_COMD7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_comd7::W`](W) writer structure"]
impl crate::Writable for I2C_COMD7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_COMD7 to value 0"]
impl crate::Resettable for I2C_COMD7_SPEC {}
