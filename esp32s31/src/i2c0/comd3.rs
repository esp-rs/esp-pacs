#[doc = "Register `COMD3` reader"]
pub type R = crate::R<COMD3_SPEC>;
#[doc = "Register `COMD3` writer"]
pub type W = crate::W<COMD3_SPEC>;
#[doc = "Field `COMMAND3` reader - This is the content of command 3. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type COMMAND3_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND3` writer - This is the content of command 3. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
pub type COMMAND3_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND3_DONE` reader - When command 3 is done in I2C Master mode, this bit changes to highlevel."]
pub type COMMAND3_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND3_DONE` writer - When command 3 is done in I2C Master mode, this bit changes to highlevel."]
pub type COMMAND3_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 3. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 3 is done in I2C Master mode, this bit changes to highlevel."]
    #[inline(always)]
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD3")
            .field("command3", &self.command3())
            .field("command3_done", &self.command3_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 3. It consists of three parts: op_code is the command, 1: WRITE, 2: STOP, 3: READ, 4: END, 6: RSTART. Byte_num represents the number of bytes that need to be sent or received.ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for moreInformation."]
    #[inline(always)]
    pub fn command3(&mut self) -> COMMAND3_W<'_, COMD3_SPEC> {
        COMMAND3_W::new(self, 0)
    }
    #[doc = "Bit 31 - When command 3 is done in I2C Master mode, this bit changes to highlevel."]
    #[inline(always)]
    pub fn command3_done(&mut self) -> COMMAND3_DONE_W<'_, COMD3_SPEC> {
        COMMAND3_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`comd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD3_SPEC;
impl crate::RegisterSpec for COMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd3::R`](R) reader structure"]
impl crate::Readable for COMD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd3::W`](W) writer structure"]
impl crate::Writable for COMD3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD3 to value 0"]
impl crate::Resettable for COMD3_SPEC {}
