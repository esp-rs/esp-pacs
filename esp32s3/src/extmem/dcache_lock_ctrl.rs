#[doc = "Register `DCACHE_LOCK_CTRL` reader"]
pub struct R(crate::R<DCACHE_LOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_LOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_LOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_LOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_LOCK_CTRL` writer"]
pub struct W(crate::W<DCACHE_LOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_LOCK_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_LOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_LOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type DCACHE_LOCK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type DCACHE_LOCK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_LOCK_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type DCACHE_UNLOCK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type DCACHE_UNLOCK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_LOCK_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_LOCK_DONE` reader - The bit is used to indicate unlock/lock operation is finished."]
pub type DCACHE_LOCK_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn dcache_lock_ena(&self) -> DCACHE_LOCK_ENA_R {
        DCACHE_LOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn dcache_unlock_ena(&self) -> DCACHE_UNLOCK_ENA_R {
        DCACHE_UNLOCK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate unlock/lock operation is finished."]
    #[inline(always)]
    pub fn dcache_lock_done(&self) -> DCACHE_LOCK_DONE_R {
        DCACHE_LOCK_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn dcache_lock_ena(&mut self) -> DCACHE_LOCK_ENA_W<0> {
        DCACHE_LOCK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn dcache_unlock_ena(&mut self) -> DCACHE_UNLOCK_ENA_W<1> {
        DCACHE_UNLOCK_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_lock_ctrl](index.html) module"]
pub struct DCACHE_LOCK_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_lock_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_lock_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_LOCK_CTRL to value 0x04"]
impl crate::Resettable for DCACHE_LOCK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
