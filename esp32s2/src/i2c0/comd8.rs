#[doc = "Register `COMD8` reader"]
pub struct R(crate::R<COMD8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD8` writer"]
pub struct W(crate::W<COMD8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD8_SPEC>;
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
impl From<crate::W<COMD8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND8` reader - This is the content of command 8. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub type COMMAND8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND8` writer - This is the content of command 8. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
pub type COMMAND8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMD8_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND8_DONE` reader - When command 8 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND8_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMMAND8_DONE` writer - When command 8 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND8_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMD8_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 8. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command8(&self) -> COMMAND8_R {
        COMMAND8_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 8 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command8_done(&self) -> COMMAND8_DONE_R {
        COMMAND8_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 8. It consists of three parts: op_code is the command, 0: RSTART. 1: WRITE. 2: READ. 3: STOP. 4: END. byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more information."]
    #[inline(always)]
    pub fn command8(&mut self) -> COMMAND8_W<0> {
        COMMAND8_W::new(self)
    }
    #[doc = "Bit 31 - When command 8 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command8_done(&mut self) -> COMMAND8_DONE_W<31> {
        COMMAND8_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd8](index.html) module"]
pub struct COMD8_SPEC;
impl crate::RegisterSpec for COMD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd8::R](R) reader structure"]
impl crate::Readable for COMD8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd8::W](W) writer structure"]
impl crate::Writable for COMD8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD8 to value 0"]
impl crate::Resettable for COMD8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
