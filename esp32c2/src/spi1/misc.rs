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
#[doc = "Field `CS0_DIS` reader - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS0_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CS0_DIS` writer - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
#[doc = "Field `CS1_DIS` reader - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS1_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CS1_DIS` writer - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<0> {
        CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<1> {
        CS1_DIS_W::new(self)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<9> {
        CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<10> {
        CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 misc register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
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
}
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
