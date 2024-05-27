#[doc = "Register `SLAVE_ADDR` reader"]
pub type R = crate::R<SLAVE_ADDR_SPEC>;
#[doc = "Register `SLAVE_ADDR` writer"]
pub type W = crate::W<SLAVE_ADDR_SPEC>;
#[doc = "Field `SLAVE_ADDR` reader - when configured as i2c slave this register is used to configure slave's address."]
pub type SLAVE_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SLAVE_ADDR` writer - when configured as i2c slave this register is used to configure slave's address."]
pub type SLAVE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADDR_10BIT_EN` reader - This register is used to enable slave 10bit address mode."]
pub type ADDR_10BIT_EN_R = crate::BitReader;
#[doc = "Field `ADDR_10BIT_EN` writer - This register is used to enable slave 10bit address mode."]
pub type ADDR_10BIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    pub fn addr_10bit_en(&self) -> ADDR_10BIT_EN_R {
        ADDR_10BIT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE_ADDR")
            .field("slave_addr", &self.slave_addr())
            .field("addr_10bit_en", &self.addr_10bit_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    #[must_use]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W<SLAVE_ADDR_SPEC> {
        SLAVE_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    #[must_use]
    pub fn addr_10bit_en(&mut self) -> ADDR_10BIT_EN_W<SLAVE_ADDR_SPEC> {
        ADDR_10BIT_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_ADDR_SPEC;
impl crate::RegisterSpec for SLAVE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_addr::R`](R) reader structure"]
impl crate::Readable for SLAVE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave_addr::W`](W) writer structure"]
impl crate::Writable for SLAVE_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE_ADDR to value 0"]
impl crate::Resettable for SLAVE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
