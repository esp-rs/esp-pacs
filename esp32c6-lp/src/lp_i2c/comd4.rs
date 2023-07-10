#[doc = "Register `COMD4` reader"]
pub struct R(crate::R<COMD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD4` writer"]
pub struct W(crate::W<COMD4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD4_SPEC>;
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
impl From<crate::W<COMD4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND4` reader - This is the content of command 4. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND4_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND4` writer - This is the content of command 4. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type COMMAND4_W<'a, const O: u8> = crate::FieldWriter<'a, COMD4_SPEC, 14, O, u16>;
#[doc = "Field `COMMAND4_DONE` reader - When command 4 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND4_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND4_DONE` writer - When command 4 is done in I2C Master mode, this bit changes to high level."]
pub type COMMAND4_DONE_W<'a, const O: u8> = crate::BitWriter<'a, COMD4_SPEC, O>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 4. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 4 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD4")
            .field("command4", &format_args!("{}", self.command4().bits()))
            .field(
                "command4_done",
                &format_args!("{}", self.command4_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 4. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn command4(&mut self) -> COMMAND4_W<0> {
        COMMAND4_W::new(self)
    }
    #[doc = "Bit 31 - When command 4 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn command4_done(&mut self) -> COMMAND4_DONE_W<31> {
        COMMAND4_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd4](index.html) module"]
pub struct COMD4_SPEC;
impl crate::RegisterSpec for COMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd4::R](R) reader structure"]
impl crate::Readable for COMD4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd4::W](W) writer structure"]
impl crate::Writable for COMD4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD4 to value 0"]
impl crate::Resettable for COMD4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
