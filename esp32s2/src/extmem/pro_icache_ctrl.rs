#[doc = "Register `PRO_ICACHE_CTRL` reader"]
pub struct R(crate::R<PRO_ICACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_ICACHE_CTRL` writer"]
pub struct W(crate::W<PRO_ICACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_ICACHE_CTRL_SPEC>;
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
impl From<crate::W<PRO_ICACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_ICACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_ENABLE` reader - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type PRO_ICACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_ENABLE` writer - The bit is used to activate the data cache. 0: disable, 1: enable"]
pub type PRO_ICACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_SETSIZE_MODE` reader - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
pub type PRO_ICACHE_SETSIZE_MODE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_SETSIZE_MODE` writer - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
pub type PRO_ICACHE_SETSIZE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_BLOCKSIZE_MODE` reader - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type PRO_ICACHE_BLOCKSIZE_MODE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_BLOCKSIZE_MODE` writer - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
pub type PRO_ICACHE_BLOCKSIZE_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub type PRO_ICACHE_INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub type PRO_ICACHE_INVALIDATE_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_INVALIDATE_DONE` reader - The bit is used to indicate invalidate operation is finished."]
pub type PRO_ICACHE_INVALIDATE_DONE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_LOCK0_EN` reader - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
pub type PRO_ICACHE_LOCK0_EN_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_LOCK0_EN` writer - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
pub type PRO_ICACHE_LOCK0_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_LOCK1_EN` reader - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
pub type PRO_ICACHE_LOCK1_EN_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_LOCK1_EN` writer - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
pub type PRO_ICACHE_LOCK1_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
pub type PRO_ICACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
pub type PRO_ICACHE_AUTOLOAD_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_DONE` reader - The bit is used to indicate conditional-preload operation is finished."]
pub type PRO_ICACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type PRO_ICACHE_PRELOAD_ENA_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
pub type PRO_ICACHE_PRELOAD_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_PRELOAD_DONE` reader - The bit is used to indicate preload operation is finished."]
pub type PRO_ICACHE_PRELOAD_DONE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type PRO_ICACHE_UNLOCK_ENA_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type PRO_ICACHE_UNLOCK_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_UNLOCK_DONE` reader - The bit is used to indicate unlock operation is finished."]
pub type PRO_ICACHE_UNLOCK_DONE_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type PRO_ICACHE_LOCK_ENA_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type PRO_ICACHE_LOCK_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_ICACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_ICACHE_LOCK_DONE` reader - The bit is used to indicate lock operation is finished."]
pub type PRO_ICACHE_LOCK_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn pro_icache_enable(&self) -> PRO_ICACHE_ENABLE_R {
        PRO_ICACHE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
    #[inline(always)]
    pub fn pro_icache_setsize_mode(&self) -> PRO_ICACHE_SETSIZE_MODE_R {
        PRO_ICACHE_SETSIZE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    pub fn pro_icache_blocksize_mode(&self) -> PRO_ICACHE_BLOCKSIZE_MODE_R {
        PRO_ICACHE_BLOCKSIZE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    pub fn pro_icache_invalidate_ena(&self) -> PRO_ICACHE_INVALIDATE_ENA_R {
        PRO_ICACHE_INVALIDATE_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate invalidate operation is finished."]
    #[inline(always)]
    pub fn pro_icache_invalidate_done(&self) -> PRO_ICACHE_INVALIDATE_DONE_R {
        PRO_ICACHE_INVALIDATE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock0_en(&self) -> PRO_ICACHE_LOCK0_EN_R {
        PRO_ICACHE_LOCK0_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
    #[inline(always)]
    pub fn pro_icache_lock1_en(&self) -> PRO_ICACHE_LOCK1_EN_R {
        PRO_ICACHE_LOCK1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn pro_icache_autoload_ena(&self) -> PRO_ICACHE_AUTOLOAD_ENA_R {
        PRO_ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to indicate conditional-preload operation is finished."]
    #[inline(always)]
    pub fn pro_icache_autoload_done(&self) -> PRO_ICACHE_AUTOLOAD_DONE_R {
        PRO_ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    pub fn pro_icache_preload_ena(&self) -> PRO_ICACHE_PRELOAD_ENA_R {
        PRO_ICACHE_PRELOAD_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to indicate preload operation is finished."]
    #[inline(always)]
    pub fn pro_icache_preload_done(&self) -> PRO_ICACHE_PRELOAD_DONE_R {
        PRO_ICACHE_PRELOAD_DONE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn pro_icache_unlock_ena(&self) -> PRO_ICACHE_UNLOCK_ENA_R {
        PRO_ICACHE_UNLOCK_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The bit is used to indicate unlock operation is finished."]
    #[inline(always)]
    pub fn pro_icache_unlock_done(&self) -> PRO_ICACHE_UNLOCK_DONE_R {
        PRO_ICACHE_UNLOCK_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn pro_icache_lock_ena(&self) -> PRO_ICACHE_LOCK_ENA_R {
        PRO_ICACHE_LOCK_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The bit is used to indicate lock operation is finished."]
    #[inline(always)]
    pub fn pro_icache_lock_done(&self) -> PRO_ICACHE_LOCK_DONE_R {
        PRO_ICACHE_LOCK_DONE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_CTRL")
            .field(
                "pro_icache_enable",
                &format_args!("{}", self.pro_icache_enable().bit()),
            )
            .field(
                "pro_icache_setsize_mode",
                &format_args!("{}", self.pro_icache_setsize_mode().bit()),
            )
            .field(
                "pro_icache_blocksize_mode",
                &format_args!("{}", self.pro_icache_blocksize_mode().bit()),
            )
            .field(
                "pro_icache_invalidate_ena",
                &format_args!("{}", self.pro_icache_invalidate_ena().bit()),
            )
            .field(
                "pro_icache_invalidate_done",
                &format_args!("{}", self.pro_icache_invalidate_done().bit()),
            )
            .field(
                "pro_icache_lock0_en",
                &format_args!("{}", self.pro_icache_lock0_en().bit()),
            )
            .field(
                "pro_icache_lock1_en",
                &format_args!("{}", self.pro_icache_lock1_en().bit()),
            )
            .field(
                "pro_icache_autoload_ena",
                &format_args!("{}", self.pro_icache_autoload_ena().bit()),
            )
            .field(
                "pro_icache_autoload_done",
                &format_args!("{}", self.pro_icache_autoload_done().bit()),
            )
            .field(
                "pro_icache_preload_ena",
                &format_args!("{}", self.pro_icache_preload_ena().bit()),
            )
            .field(
                "pro_icache_preload_done",
                &format_args!("{}", self.pro_icache_preload_done().bit()),
            )
            .field(
                "pro_icache_unlock_ena",
                &format_args!("{}", self.pro_icache_unlock_ena().bit()),
            )
            .field(
                "pro_icache_unlock_done",
                &format_args!("{}", self.pro_icache_unlock_done().bit()),
            )
            .field(
                "pro_icache_lock_ena",
                &format_args!("{}", self.pro_icache_lock_ena().bit()),
            )
            .field(
                "pro_icache_lock_done",
                &format_args!("{}", self.pro_icache_lock_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the data cache. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_enable(&mut self) -> PRO_ICACHE_ENABLE_W<0> {
        PRO_ICACHE_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure cache memory size.0: 8KB, 1: 16KB"]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_setsize_mode(&mut self) -> PRO_ICACHE_SETSIZE_MODE_W<2> {
        PRO_ICACHE_SETSIZE_MODE_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to configure cache block size.0: 16 bytes, 1: 32 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_blocksize_mode(&mut self) -> PRO_ICACHE_BLOCKSIZE_MODE_W<3> {
        PRO_ICACHE_BLOCKSIZE_MODE_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_invalidate_ena(&mut self) -> PRO_ICACHE_INVALIDATE_ENA_W<8> {
        PRO_ICACHE_INVALIDATE_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK0_ADDR_REG and PRO_ICACHE_LOCK0_SIZE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_lock0_en(&mut self) -> PRO_ICACHE_LOCK0_EN_W<14> {
        PRO_ICACHE_LOCK0_EN_W::new(self)
    }
    #[doc = "Bit 15 - The bit is used to enable pre-lock operation which is combined with PRO_ICACHE_LOCK1_ADDR_REG and PRO_ICACHE_LOCK1_SIZE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_lock1_en(&mut self) -> PRO_ICACHE_LOCK1_EN_W<15> {
        PRO_ICACHE_LOCK1_EN_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to enable and disable conditional-preload operation. It is combined with pre_dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_ena(&mut self) -> PRO_ICACHE_AUTOLOAD_ENA_W<18> {
        PRO_ICACHE_AUTOLOAD_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_preload_ena(&mut self) -> PRO_ICACHE_PRELOAD_ENA_W<20> {
        PRO_ICACHE_PRELOAD_ENA_W::new(self)
    }
    #[doc = "Bit 22 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_unlock_ena(&mut self) -> PRO_ICACHE_UNLOCK_ENA_W<22> {
        PRO_ICACHE_UNLOCK_ENA_W::new(self)
    }
    #[doc = "Bit 24 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_lock_ena(&mut self) -> PRO_ICACHE_LOCK_ENA_W<24> {
        PRO_ICACHE_LOCK_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_ctrl](index.html) module"]
pub struct PRO_ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_icache_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_ICACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_ICACHE_CTRL to value 0x0100"]
impl crate::Resettable for PRO_ICACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
