#[doc = "Register `COMD7` reader"]
pub struct R(crate::R<COMD7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD7` writer"]
pub struct W(crate::W<COMD7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD7_SPEC>;
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
impl From<crate::W<COMD7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND7` reader - This is the content of command 7. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND7` writer - This is the content of command 7. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND7_W<'a> = crate::FieldWriter<'a, u32, COMD7_SPEC, u16, u16, 14, 0>;
#[doc = "Field `COMMAND7_DONE` reader - When command 7 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND7_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMMAND7_DONE` writer - When command 7 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND7_DONE_W<'a> = crate::BitWriter<'a, u32, COMD7_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 7. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 7 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 7. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command7(&mut self) -> COMMAND7_W {
        COMMAND7_W::new(self)
    }
    #[doc = "Bit 31 - When command 7 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command7_done(&mut self) -> COMMAND7_DONE_W {
        COMMAND7_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd7](index.html) module"]
pub struct COMD7_SPEC;
impl crate::RegisterSpec for COMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd7::R](R) reader structure"]
impl crate::Readable for COMD7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd7::W](W) writer structure"]
impl crate::Writable for COMD7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD7 to value 0"]
impl crate::Resettable for COMD7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
