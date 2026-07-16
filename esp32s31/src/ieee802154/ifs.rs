#[doc = "Register `IFS` reader"]
pub type R = crate::R<IFS_SPEC>;
#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `SIFS` reader - "]
pub type SIFS_R = crate::FieldReader;
#[doc = "Field `SIFS` writer - "]
pub type SIFS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_WAIT_SYMDEC_ON_TIME` reader - "]
pub type RX_WAIT_SYMDEC_ON_TIME_R = crate::FieldReader;
#[doc = "Field `RX_WAIT_SYMDEC_ON_TIME` writer - "]
pub type RX_WAIT_SYMDEC_ON_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_WAIT_SYMDEC_ON_ENA` reader - "]
pub type RX_WAIT_SYMDEC_ON_ENA_R = crate::BitReader;
#[doc = "Field `RX_WAIT_SYMDEC_ON_ENA` writer - "]
pub type RX_WAIT_SYMDEC_ON_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIFS` reader - "]
pub type LIFS_R = crate::FieldReader<u16>;
#[doc = "Field `LIFS` writer - "]
pub type LIFS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sifs(&self) -> SIFS_R {
        SIFS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rx_wait_symdec_on_time(&self) -> RX_WAIT_SYMDEC_ON_TIME_R {
        RX_WAIT_SYMDEC_ON_TIME_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_wait_symdec_on_ena(&self) -> RX_WAIT_SYMDEC_ON_ENA_R {
        RX_WAIT_SYMDEC_ON_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn lifs(&self) -> LIFS_R {
        LIFS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFS")
            .field("sifs", &self.sifs())
            .field("rx_wait_symdec_on_time", &self.rx_wait_symdec_on_time())
            .field("rx_wait_symdec_on_ena", &self.rx_wait_symdec_on_ena())
            .field("lifs", &self.lifs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sifs(&mut self) -> SIFS_W<'_, IFS_SPEC> {
        SIFS_W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rx_wait_symdec_on_time(&mut self) -> RX_WAIT_SYMDEC_ON_TIME_W<'_, IFS_SPEC> {
        RX_WAIT_SYMDEC_ON_TIME_W::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_wait_symdec_on_ena(&mut self) -> RX_WAIT_SYMDEC_ON_ENA_W<'_, IFS_SPEC> {
        RX_WAIT_SYMDEC_ON_ENA_W::new(self, 15)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn lifs(&mut self) -> LIFS_W<'_, IFS_SPEC> {
        LIFS_W::new(self, 16)
    }
}
#[doc = "IFS\n\nYou can [`read`](crate::Reg::read) this register and get [`ifs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifs::R`](R) reader structure"]
impl crate::Readable for IFS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {}
