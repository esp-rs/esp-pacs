#[doc = "Register `COMD%s` reader"]
pub struct R(crate::R<COMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD%s` writer"]
pub struct W(crate::W<COMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD_SPEC>;
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
impl From<crate::W<COMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND0` reader - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND0` writer - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMD_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND0_DONE` reader - When command 0 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND0_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMMAND0_DONE` writer - When command 0 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND0_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMD_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 0:13 - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn command0(&mut self) -> COMMAND0_W<0> {
        COMMAND0_W::new(self)
    }
    #[doc = "Bit 31 - When command 0 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn command0_done(&mut self) -> COMMAND0_DONE_W<31> {
        COMMAND0_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd](index.html) module"]
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd::R](R) reader structure"]
impl crate::Readable for COMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd::W](W) writer structure"]
impl crate::Writable for COMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for COMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
