#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_CMD` reader - Set this bit to send read command."]
pub type READ_CMD_R = crate::BitReader;
#[doc = "Field `READ_CMD` writer - Set this bit to send read command."]
pub type READ_CMD_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `PGM_CMD` reader - Set this bit to send programming command."]
pub type PGM_CMD_R = crate::BitReader;
#[doc = "Field `PGM_CMD` writer - Set this bit to send programming command."]
pub type PGM_CMD_W<'a, const O: u8> = crate::BitWriter<'a, CMD_SPEC, O>;
#[doc = "Field `BLK_NUM` reader - The serial number of the block to be programmed. Value 0-3 corresponds to block number 0-3, respectively."]
pub type BLK_NUM_R = crate::FieldReader;
#[doc = "Field `BLK_NUM` writer - The serial number of the block to be programmed. Value 0-3 corresponds to block number 0-3, respectively."]
pub type BLK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to send read command."]
    #[inline(always)]
    pub fn read_cmd(&self) -> READ_CMD_R {
        READ_CMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to send programming command."]
    #[inline(always)]
    pub fn pgm_cmd(&self) -> PGM_CMD_R {
        PGM_CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - The serial number of the block to be programmed. Value 0-3 corresponds to block number 0-3, respectively."]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("read_cmd", &format_args!("{}", self.read_cmd().bit()))
            .field("pgm_cmd", &format_args!("{}", self.pgm_cmd().bit()))
            .field("blk_num", &format_args!("{}", self.blk_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to send read command."]
    #[inline(always)]
    #[must_use]
    pub fn read_cmd(&mut self) -> READ_CMD_W<0> {
        READ_CMD_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to send programming command."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_cmd(&mut self) -> PGM_CMD_W<1> {
        PGM_CMD_W::new(self)
    }
    #[doc = "Bits 2:3 - The serial number of the block to be programmed. Value 0-3 corresponds to block number 0-3, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn blk_num(&mut self) -> BLK_NUM_W<2> {
        BLK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eFuse command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
