#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `DUMMY_OUT` reader - "]
pub type DUMMY_OUT_R = crate::BitReader;
#[doc = "Field `DUMMY_OUT` writer - "]
pub type DUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - "]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - "]
pub type Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - "]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - "]
pub type D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_BIT_ORDER` reader - "]
pub type RD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `RD_BIT_ORDER` writer - "]
pub type RD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_BIT_ORDER` reader - "]
pub type WR_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `WR_BIT_ORDER` writer - "]
pub type WR_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dummy_out(&self) -> DUMMY_OUT_R {
        DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_bit_order(&self) -> RD_BIT_ORDER_R {
        RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wr_bit_order(&self) -> WR_BIT_ORDER_R {
        WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("dummy_out", &self.dummy_out())
            .field("q_pol", &self.q_pol())
            .field("d_pol", &self.d_pol())
            .field("rd_bit_order", &self.rd_bit_order())
            .field("wr_bit_order", &self.wr_bit_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dummy_out(&mut self) -> DUMMY_OUT_W<'_, CTRL_SPEC> {
        DUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W<'_, CTRL_SPEC> {
        Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W<'_, CTRL_SPEC> {
        D_POL_W::new(self, 19)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_bit_order(&mut self) -> RD_BIT_ORDER_W<'_, CTRL_SPEC> {
        RD_BIT_ORDER_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wr_bit_order(&mut self) -> WR_BIT_ORDER_W<'_, CTRL_SPEC> {
        WR_BIT_ORDER_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x000c_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x000c_0000;
}
