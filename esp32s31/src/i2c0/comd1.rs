#[doc = "Register `COMD1` reader"]
pub type R = crate::R<COMD1_SPEC>;
#[doc = "Register `COMD1` writer"]
pub type W = crate::W<COMD1_SPEC>;
#[doc = "Field `COMMAND1` reader - This is the content of command 1. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type COMMAND1_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND1` writer - This is the content of command 1. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type COMMAND1_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND1_DONE` reader - When command 1 is done in I2C Master mode, this bit changes to highlevel."]
pub type COMMAND1_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND1_DONE` writer - When command 1 is done in I2C Master mode, this bit changes to highlevel."]
pub type COMMAND1_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 1. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 1 is done in I2C Master mode, this bit changes to highlevel."]
    #[inline(always)]
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD1")
            .field("command1", &self.command1())
            .field("command1_done", &self.command1_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 1. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn command1(&mut self) -> COMMAND1_W<'_, COMD1_SPEC> {
        COMMAND1_W::new(self, 0)
    }
    #[doc = "Bit 31 - When command 1 is done in I2C Master mode, this bit changes to highlevel."]
    #[inline(always)]
    pub fn command1_done(&mut self) -> COMMAND1_DONE_W<'_, COMD1_SPEC> {
        COMMAND1_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD1_SPEC;
impl crate::RegisterSpec for COMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd1::R`](R) reader structure"]
impl crate::Readable for COMD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd1::W`](W) writer structure"]
impl crate::Writable for COMD1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD1 to value 0"]
impl crate::Resettable for COMD1_SPEC {}
