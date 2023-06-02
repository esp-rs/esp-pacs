#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0_DIS` reader - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
pub type CS0_DIS_R = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
pub type CS0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, MISC_SPEC, O>;
#[doc = "Field `CS1_DIS` reader - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
pub type CS1_DIS_R = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
pub type CS1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, MISC_SPEC, O>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
pub type CK_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, MISC_SPEC, O>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, MISC_SPEC, O>;
#[doc = "Field `AUTO_PER` reader - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
pub type AUTO_PER_R = crate::BitReader;
#[doc = "Field `AUTO_PER` writer - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
pub type AUTO_PER_W<'a, const O: u8> = crate::BitWriter<'a, MISC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
    #[inline(always)]
    pub fn auto_per(&self) -> AUTO_PER_R {
        AUTO_PER_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs0_dis", &format_args!("{}", self.cs0_dis().bit()))
            .field("cs1_dis", &format_args!("{}", self.cs1_dis().bit()))
            .field(
                "ck_idle_edge",
                &format_args!("{}", self.ck_idle_edge().bit()),
            )
            .field(
                "cs_keep_active",
                &format_args!("{}", self.cs_keep_active().bit()),
            )
            .field("auto_per", &format_args!("{}", self.auto_per().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS pin, which means that the SPI device(flash) connected to SPI_CS is in low level when SPI1 transfer starts."]
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<0> {
        CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM) connected to SPI_CS1 is in low level when SPI1 transfer starts."]
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<1> {
        CS1_DIS_W::new(self)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when MSPI is idle. 0: SPI_CLK line is low when MSPI is idle."]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<9> {
        CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<10> {
        CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to enable auto PER function. Hardware will sent out PER command if PES command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn auto_per(&mut self) -> AUTO_PER_W<11> {
        AUTO_PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 misc register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R](R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
