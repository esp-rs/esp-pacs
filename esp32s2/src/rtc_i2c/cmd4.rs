#[doc = "Register `CMD4` reader"]
pub struct R(crate::R<CMD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD4` writer"]
pub struct W(crate::W<CMD4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD4_SPEC>;
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
impl From<crate::W<CMD4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND4` reader - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
pub type COMMAND4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND4` writer - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
pub type COMMAND4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD4_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND4_DONE` reader - When command 4 is done, this bit changes to 1."]
pub type COMMAND4_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 4 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command4(&mut self) -> COMMAND4_W<0> {
        COMMAND4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC I2C Command 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd4](index.html) module"]
pub struct CMD4_SPEC;
impl crate::RegisterSpec for CMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd4::R](R) reader structure"]
impl crate::Readable for CMD4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd4::W](W) writer structure"]
impl crate::Writable for CMD4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD4 to value 0x0901"]
impl crate::Resettable for CMD4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0901
    }
}
