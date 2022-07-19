#[doc = "Register `COMD1` reader"]
pub struct R(crate::R<COMD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD1` writer"]
pub struct W(crate::W<COMD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<COMD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND1` reader - This is the content of command 1. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND1` writer - This is the content of command 1. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND1_W<'a> = crate::FieldWriter<'a, u32, COMD1_SPEC, u16, u16, 14, 0>;
#[doc = "Field `COMMAND1_DONE` reader - When command 1 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND1_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMMAND1_DONE` writer - When command 1 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND1_DONE_W<'a> = crate::BitWriter<'a, u32, COMD1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 1. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 1 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 1. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command1(&mut self) -> COMMAND1_W {
        COMMAND1_W::new(self)
    }
    #[doc = "Bit 31 - When command 1 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command1_done(&mut self) -> COMMAND1_DONE_W {
        COMMAND1_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd1](index.html) module"]
pub struct COMD1_SPEC;
impl crate::RegisterSpec for COMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd1::R](R) reader structure"]
impl crate::Readable for COMD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd1::W](W) writer structure"]
impl crate::Writable for COMD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD1 to value 0"]
impl crate::Resettable for COMD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
