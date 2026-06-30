#[doc = "Register `OUT_DSCR_PORT_BLK_CH%s` reader"]
pub type R = crate::R<OUT_DSCR_PORT_BLK_CH_SPEC>;
#[doc = "Register `OUT_DSCR_PORT_BLK_CH%s` writer"]
pub type W = crate::W<OUT_DSCR_PORT_BLK_CH_SPEC>;
#[doc = "Field `OUT_DSCR_PORT_BLK_H_CH` reader - Set the vertical height of tx block size in dscr port mode"]
pub type OUT_DSCR_PORT_BLK_H_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_DSCR_PORT_BLK_H_CH` writer - Set the vertical height of tx block size in dscr port mode"]
pub type OUT_DSCR_PORT_BLK_H_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `OUT_DSCR_PORT_BLK_V_CH` reader - Set the horizontal width of tx block size in dscr port mode"]
pub type OUT_DSCR_PORT_BLK_V_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_DSCR_PORT_BLK_V_CH` writer - Set the horizontal width of tx block size in dscr port mode"]
pub type OUT_DSCR_PORT_BLK_V_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Set the vertical height of tx block size in dscr port mode"]
    #[inline(always)]
    pub fn out_dscr_port_blk_h_ch(&self) -> OUT_DSCR_PORT_BLK_H_CH_R {
        OUT_DSCR_PORT_BLK_H_CH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - Set the horizontal width of tx block size in dscr port mode"]
    #[inline(always)]
    pub fn out_dscr_port_blk_v_ch(&self) -> OUT_DSCR_PORT_BLK_V_CH_R {
        OUT_DSCR_PORT_BLK_V_CH_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_PORT_BLK_CH")
            .field("out_dscr_port_blk_h_ch", &self.out_dscr_port_blk_h_ch())
            .field("out_dscr_port_blk_v_ch", &self.out_dscr_port_blk_v_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Set the vertical height of tx block size in dscr port mode"]
    #[inline(always)]
    pub fn out_dscr_port_blk_h_ch(
        &mut self,
    ) -> OUT_DSCR_PORT_BLK_H_CH_W<'_, OUT_DSCR_PORT_BLK_CH_SPEC> {
        OUT_DSCR_PORT_BLK_H_CH_W::new(self, 0)
    }
    #[doc = "Bits 14:27 - Set the horizontal width of tx block size in dscr port mode"]
    #[inline(always)]
    pub fn out_dscr_port_blk_v_ch(
        &mut self,
    ) -> OUT_DSCR_PORT_BLK_V_CH_W<'_, OUT_DSCR_PORT_BLK_CH_SPEC> {
        OUT_DSCR_PORT_BLK_V_CH_W::new(self, 14)
    }
}
#[doc = "Configures the tx block size in dscr port mode\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_port_blk_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_dscr_port_blk_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DSCR_PORT_BLK_CH_SPEC;
impl crate::RegisterSpec for OUT_DSCR_PORT_BLK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_port_blk_ch::R`](R) reader structure"]
impl crate::Readable for OUT_DSCR_PORT_BLK_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_dscr_port_blk_ch::W`](W) writer structure"]
impl crate::Writable for OUT_DSCR_PORT_BLK_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_DSCR_PORT_BLK_CH%s to value 0x0004_8012"]
impl crate::Resettable for OUT_DSCR_PORT_BLK_CH_SPEC {
    const RESET_VALUE: u32 = 0x0004_8012;
}
